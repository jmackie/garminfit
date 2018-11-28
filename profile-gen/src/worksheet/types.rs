use base::KNOWN_BASE_TYPES;
use calamine;
use error::{
    Error,
    Result,
};
use inflector::Inflector;
use itertools::Itertools;
use proc_macro2::{
    Ident,
    Literal,
    Span,
    TokenStream,
};
use std::{
    collections::HashMap,
    convert::From,
    iter,
};
use util;

/// The worksheet what we want.
static WORKSHEET_NAME: &'static str = "Types";

/// The sheet relevant to this module.
pub struct Sheet(calamine::Range<calamine::DataType>);

impl Sheet {
    fn rows(&self) -> calamine::Rows<calamine::DataType> {
        self.0.rows()
    }
}

/// Attempt to open the types worksheet.
pub fn open_sheet<R>(workbook: &mut R) -> Result<Sheet>
where
    R: calamine::Reader,
{
    workbook
        .worksheet_range(WORKSHEET_NAME)
        .ok_or(Error::missing_sheet(WORKSHEET_NAME))?
        .map_err(Error::bad_sheet(WORKSHEET_NAME))
        .map(Sheet)
}

#[derive(Clone)]
pub struct Type {
    pub name:      String,
    pub base_type: String,
    pub values:    Vec<Value>,
    pub comment:   Option<String>,
}

#[derive(Clone)]
pub struct Value {
    pub name:    String,
    pub value:   u64,
    pub comment: Option<String>,
}

pub fn generate_module(
    sdk_version: &str,
    types: &[Type],
) -> Result<TokenStream> {
    let mut tokens = quote! {
        #![doc="Generated for FIT SDK profile version: "]
        #![doc=#sdk_version]

        use byteorder::ByteOrder;
        use error;
        use profile;
    };

    for ty in types {
        tokens.extend(match ty.values.len() {
            0 => generate_type_prim(&ty),
            // TODO: For now we just capture the comment associated with this
            // "Min" value, but it should really be represented in
            // the type.
            1 if ty.values[0].name == "Min" => {
                let more_comment =
                    ty.values[0].comment.clone().unwrap_or(String::new());

                generate_type_prim(&Type {
                    comment: ty.comment.clone().map(|comment| {
                        format!("{}; {}", comment, more_comment)
                    }),
                    ..ty.clone()
                })
            },
            _ => generate_type_enum(&ty),
        });
    }
    Ok(tokens)
}

fn generate_type_enum(ty: &Type) -> TokenStream {
    let name = Ident::new(&ty.name, Span::call_site());
    let comment = match ty.comment {
        Some(ref comment) => quote! { #[doc=#comment] },
        None => TokenStream::new(),
    };

    // NOTE: ty.values.len() could be 0
    let variants = ty.values.iter().filter_map(|val| {
        // Not all Types have an Unknown value, so we insert one.
        if val.name == "Unknown" {
            return None
        };

        let name = Ident::new(&val.name, Span::call_site());
        let value = Literal::u64_unsuffixed(val.value);
        let comment = match val.comment {
            Some(ref comment) => quote! { #[doc=#comment] },
            None => TokenStream::new(),
        };
        Some(quote! {
            #comment
            #name = #value
        })
    });

    let decode_impl = generate_type_enum_decode_impl(ty);

    quote! {
        #comment
        #[derive(Debug)]
        pub enum #name {
            #(#variants,)*
            Unknown,
        }

        #decode_impl
    }
}

fn generate_type_prim(ty: &Type) -> TokenStream {
    let name = Ident::new(&ty.name, Span::call_site());
    let comment = match ty.comment {
        Some(ref comment) => quote! { #[doc=#comment] },
        None => TokenStream::new(),
    };

    let prim_type = Ident::new(
        match ty.base_type.as_str() {
            "uint8" => "u8",
            "uint32" => "u32",
            other => other,
        },
        Span::call_site(),
    );

    let decode_impl = generate_type_prim_decode_impl(ty);

    quote! {
        #comment
        #[derive(Debug)]
        pub struct #name(pub #prim_type);

        #decode_impl
    }
}

fn generate_type_enum_decode_impl(ty: &Type) -> TokenStream {
    let type_name = Ident::new(&ty.name, Span::call_site());

    let base_type = match KNOWN_BASE_TYPES
        .get(&ty.base_type)
        .map(String::as_str)
    {
        // Do we have a valid base type?
        Some("string") => Ident::new("Utf8String", Span::call_site()),
        Some("bytes") => Ident::new("Byte", Span::call_site()),
        Some(ty) => Ident::new(&util::uppercase_first(ty), Span::call_site()),
        None => panic!("unknown base type: {}", ty.base_type),
    };

    let match_arms = ty.values.iter().map(|val| {
        let name = Ident::new(&val.name, Span::call_site());
        let value = Literal::u64_unsuffixed(val.value);
        quote! { #value => Ok(#type_name::#name) }
    });

    quote! {
        impl #type_name {
            pub(crate) fn decode<T: ByteOrder>(
                buffer: &[u8],
            ) -> error::Result<Self> {
                let base_value = profile::base::#base_type::decode::<T>(buffer)?;
                match base_value.0 {
                    #(#match_arms,)*
                    _ => Ok(#type_name::Unknown),
                }
            }
        }
    }
}

fn generate_type_prim_decode_impl(ty: &Type) -> TokenStream {
    let type_name = Ident::new(&ty.name, Span::call_site());
    let decode_body = match ty.base_type.as_str() {
        "uint8" => {
            quote! {
                Ok(#type_name(buffer[0]))
            }
        },
        "uint32" => {
            quote! {
                Ok(#type_name(T::read_u32(buffer)))
            }
        },
        _ => quote! { unimplemented!() },
    };

    quote! {
        impl #type_name {
            pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> error::Result<Self> {
                #decode_body
            }
        }
    }
}

/// Extract `Type`s from a worksheet.
pub fn extract(sheet: &Sheet) -> Vec<Type> {
    sheet
        .rows()
        // Ignore the header.
        .skip(1)
        // Extract the interesting rows.
        .filter_map(|raw| {
            let row = Row::from(raw);
            match row {
                Row::Header {
                    ..
                } => Some(row),
                Row::Value {
                    ..
                } => Some(row),
                _ => None,
            }
        })
        .peekable()
        .batching(|iter| {
            let header_row = match iter.next() {
                None => return None,
                Some(row) => {
                    if !row.is_header() {
                        // We can only panic inside this closure.
                        panic!("expecting type header row, got: {:?}", row)
                    }
                    row
                },
            };
            let value_rows =
                iter.peeking_take_while(Row::not_header).collect::<Vec<_>>();

            Some(Type::from(
                iter::once(header_row).chain(value_rows).collect::<Vec<_>>(),
            ))
        })
        .collect()
}

/// `Row` enumerates the *recognised* row formats within
/// the "Types" worksheet.
#[derive(Debug)]
enum Row<'a> {
    Header { type_name: String, base_type: String, comment:   Option<String> },
    Value { name:    String, value:   u64, comment: Option<String> },
    Empty,

    // Catch unknown formats,
    // because std::convert::From is infallible.
    Unknown(&'a [calamine::DataType]),
}

impl<'a> Row<'a> {
    fn is_header(&self) -> bool {
        match *self {
            Row::Header {
                ..
            } => true,
            _ => false,
        }
    }

    fn not_header(&self) -> bool {
        !self.is_header()
    }
}

// Sanitize a raw worksheet row into a `Row`.
impl<'a> From<&'a [calamine::DataType]> for Row<'a> {
    // NOTE: This logic is very brittle.
    // But at least it's all contained here.
    fn from(row: &'a [calamine::DataType]) -> Self {
        #[rustfmt_skip]
        match *row {

            // Type Name | Base Type | Value Name | Value | Comment
            // ----------|-----------|------------|-------|--------
            // type_name | base_type |            |       | comment
            [ calamine::DataType::String(ref type_name)
            , calamine::DataType::String(ref base_type)
            , ref rest..
            , ref comment
            ] if util::all_empty(rest) => {
                Row::Header {
                    type_name: type_name.trim().to_pascal_case(),
                    base_type: base_type.trim().to_string(),
                    comment: match comment {
                        calamine::DataType::String(s) => Some(s.trim().to_string()),
                        _ => None,
                    },
                }
            },

            // Type Name | Base Type | Value Name | Value | Comment
            // ----------|-----------|------------|-------|--------
            //           |           |    name    | value | comment
            [ calamine::DataType::Empty
            , calamine::DataType::Empty
            , calamine::DataType::String(ref name)
            , ref value
            , ref comment
            , _..
            ] => {
                Row::Value {
                    name: VALUE_NAME_QUIRKS
                            .get(&name[..])
                            .unwrap_or(&&name[..])
                            .trim()
                            .to_pascal_case(),
                    value: match value {
                        calamine::DataType::String(s) => util::parse_hex(s.trim()),
                        calamine::DataType::Float(f) => f.trunc() as u64,
                        _ => panic!("no appropriate value field for {}", name),
                    },
                    comment: match comment {
                        calamine::DataType::String(s) => Some(s.trim().to_string()),
                        _ => None,
                    },
                }
            },
            ref row if util::all_empty(row) => Row::Empty,

            ref row => Row::Unknown(row),
        }
    }
}

// Convert from a sanitised `Row` representation to the
// actual `Type` struct.
impl<'a> From<Vec<Row<'a>>> for Type {
    fn from(rows: Vec<Row>) -> Self {
        rows.into_iter().fold(
            Type {
                name:      String::new(),
                base_type: String::new(),
                values:    Vec::new(),
                comment:   None,
            },
            |mut acc, row| {
                match row {
                    Row::Header {
                        type_name,
                        base_type,
                        comment,
                    } => {
                        acc.name = type_name;
                        acc.base_type = base_type;
                        acc.comment = comment;
                    },
                    Row::Value {
                        name,
                        value,
                        comment,
                    } => {
                        // Check the value isn't deprecated.
                        // At the time of writing there is only one
                        // such value, hence the simple if.
                        if &name != "Forecast" {
                            acc.values.push(Value {
                                name,
                                value,
                                comment,
                            });
                        }
                    },
                    _ => (),
                };
                acc
            },
        )
    }
}

lazy_static! {
    /// Rename value names that cant be valid identifiers.
    static ref VALUE_NAME_QUIRKS: HashMap<&'static str, &'static str> = {
        #[rustfmt_skip]
        [(
            "30_degree_lat_pulldown",
            "thirty_degree_lat_pulldown",
         ),
         (
            "45_degree_cable_external_rotation",
            "fourty_five_degree_cable_external_rotation",
         ),
         (
            "90_degree_cable_external_rotation",
            "ninety_degree_cable_external_rotation",
         ),
         (
            "45_degree_plank",
            "fourty_five_degree_plank",
         ),
         (
            "45_degree_plank",
            "fourty_five_degree_plank",
         ),
         (
            "3_way_calf_raise",
            "three_way_calf_raise",
         ),
         (
            "4iiiis",
            "Fouriiiis",
         ),
         (
            "90_degree_static_hold",
            "ninety_degree_static_hold",
         ),
         (
            "1partcarbon",
            "onepartcarbon",
         ),
         (
            "3_way_weighted_calf_raise",
            "three_way_weighted_calf_raise",
         ),
         (
            "3_way_single_leg_raise",
            "three_way_single_leg_raise",
         ),
         (
            "3_way_single_leg_calf_raise",
            "three_way_single_leg_calf_raise",
         ),
         (
            "3_way_weighted_single_leg_calf_raise",
            "three_way_weighted_single_leg_calf_raise",
         ),
        ].iter().cloned().collect()
    };
}
