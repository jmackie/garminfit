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
    iter,
};
use util;

/// The worksheet what we want.
static WORKSHEET_NAME: &'static str = "Messages";

/// Wrapped worksheet.
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
pub struct Message {
    name:    String,
    fields:  Vec<Field>,
    comment: Option<String>,
}

#[derive(Clone)]
pub struct Field {
    name:    String,
    def_num: u8,
    type_:   String, // Either<Type, BaseType>
    scale:   Option<f64>,
    offset:  Option<f64>,
    units:   Option<String>,
    refs:    Option<Vec<(String, String)>>,
    comment: Option<String>,
}

pub fn generate_module(
    sdk_version: &str,
    messages: &[Message],
    mesg_nums: HashMap<String, u64>,
) -> Result<TokenStream> {
    // Module header:
    let mut tokens = quote! {
        #![doc="Generated for FIT SDK profile version: "]
        #![doc=#sdk_version]

        use byteorder::ByteOrder;
        use error;
        use profile;
        use types;

        /// The actual data of a `Message`.
        #[derive(Debug)]
        pub struct Field<T> {
            value:  T,
            scale:  Option<f64>,
            offset: Option<f64>,
            pub units:  Option<&'static str>,
        }

        // TODO: Finish these impls

        impl types::field::Field for Field<profile::base::Float64> {
            type Value = f64;

            fn value(&self) -> Self::Value {
                self.value.0 / self.scale.unwrap_or(1.0) - self.offset.unwrap_or(0.0)
            }
        }
    };

    tokens.extend(generate_message_enum(&messages));

    let mut numbered_messages = Vec::new();
    // Hacky way of sequencing `Result`s.
    messages.iter().cloned().fold(Ok(()), |accum, mesg| {
        match accum {
            Ok(_) => {
                match mesg_nums.get(&mesg.name) {
                    Some(i) => {
                        numbered_messages.push((*i, mesg));
                        accum
                    },
                    // We can't allow this to pass.
                    None => Err(Error::missing_message(mesg.name)),
                }
            },
            _ => accum,
        }
    })?;
    tokens.extend(generate_message_decode_impl(&numbered_messages));

    for message in messages {
        tokens.extend(generate_message_inner(&message));
    }
    Ok(tokens)
}

fn generate_message_enum(messages: &[Message]) -> TokenStream {
    let variants = messages.iter().map(|mesg| {
        let name = Ident::new(&mesg.name, Span::call_site());
        quote! { #name(#name) }
    });

    quote! {
        /// All the FIT message types.
        #[derive(Debug)]
        pub enum Message {
            #(#variants,)*
            Unknown {
                data: Vec<u8>,
                mesg_num: u16,
                field_def_num: u8,
            }
        }
    }
}

fn generate_message_decode_impl(
    numbered_messages: &[(u64, Message)],
) -> TokenStream {
    let match_arms = numbered_messages.iter().map(|(n, mesg)| {
        let num = Literal::u64_unsuffixed(*n);
        let name = Ident::new(&mesg.name, Span::call_site());
        quote! { #num => #name::decode::<T>(buffer, field_def_num).map(Message::#name) }
    });

    quote! {
        impl Message {
            pub(crate) fn decode<T: ByteOrder>(
                buffer: &[u8],
                mesg_num: u16,
                field_def_num: u8,
            ) -> error::Result<Self> {
                match mesg_num {
                    #(#match_arms,)*
                    _ => Ok(Message::Unknown {
                        data: buffer.to_vec(),
                        mesg_num,
                        field_def_num
                    }),
                }
            }
        }
    }
}

fn generate_message_inner(message: &Message) -> TokenStream {
    let name = Ident::new(&message.name, Span::call_site());
    let comment = match message.comment {
        Some(ref comment) => quote! { #[doc=#comment] },
        None => TokenStream::new(),
    };

    let variants = message.fields.iter().filter_map(|field| {
        let comment = match field.comment {
            Some(ref comment) => quote! { #[doc=#comment] },
            None => TokenStream::new(),
        };
        let variant = Ident::new(&field.name, Span::call_site());
        let namespace = if KNOWN_BASE_TYPES.contains(&field.type_) {
            Ident::new("base", Span::call_site())
        }
        else {
            Ident::new("types", Span::call_site())
        };
        let member =
            match KNOWN_BASE_TYPES.get(&field.type_).map(String::as_str) {
                // Handle this type belonging to the base types.
                Some("string") => Ident::new("Utf8String", Span::call_site()),
                Some("byte") => Ident::new("Bytes", Span::call_site()),
                Some(ty) => {
                    Ident::new(&util::uppercase_first(ty), Span::call_site())
                },
                // Otherwise it belongs to sdk types.
                None => {
                    Ident::new(&field.type_.to_pascal_case(), Span::call_site())
                },
            };
        Some(quote! {
            #comment
            #variant(Field<profile::#namespace::#member>)
        })
    });

    let decode_impl = generate_message_inner_decode_impl(message);

    quote! {
        #comment
        #[derive(Debug)]
        pub enum #name {
            #(#variants,)*
            Unknown {
                data: Vec<u8>,
                field_def_num: u8,
            }
        }

        #decode_impl
    }
}

fn generate_message_inner_decode_impl(message: &Message) -> TokenStream {
    let message_name = Ident::new(&message.name, Span::call_site());

    let match_arms = message.fields.iter().filter_map(|field| {
        let field_name = Ident::new(&field.name, Span::call_site());
        let field_def_num = Literal::u8_unsuffixed(field.def_num);

        let value = {
            let namespace = if KNOWN_BASE_TYPES.contains(&field.type_) {
                Ident::new("base", Span::call_site())
            }
            else {
                Ident::new("types", Span::call_site())
            };

            let member = match KNOWN_BASE_TYPES
                .get(&field.type_)
                .map(String::as_str)
            {
                // Handle this type belonging to the base types.
                Some("string") => Ident::new("Utf8String", Span::call_site()),
                Some("byte") => Ident::new("Bytes", Span::call_site()),
                Some(ty) => {
                    Ident::new(&util::uppercase_first(ty), Span::call_site())
                },
                // Otherwise it belongs to sdk types.
                None => {
                    match String::as_str(&field.type_) {
                        "bool" => return None, // NOTE: idk
                        _ => {
                            Ident::new(
                                &field.type_.to_pascal_case(),
                                Span::call_site(),
                            )
                        },
                    }
                },
            };

            quote! {
                profile::#namespace::#member::decode::<T>(buffer)?
            }
        };

        let scale = match field.scale {
            Some(scale) => {
                let scale_lit = Literal::f64_unsuffixed(scale);
                quote! { Some(#scale_lit) }
            },
            None => quote! { None },
        };
        let offset = match field.offset {
            Some(offset) => {
                let offset_lit = Literal::f64_unsuffixed(offset);
                quote! { Some(#offset_lit) }
            },
            None => quote! { None },
        };
        let units = match field.units {
            Some(ref unit) => {
                let unit_lit = Literal::string(unit);
                quote! { Some(#unit_lit) }
            },
            None => quote! { None },
        };

        Some(quote! {
            #field_def_num => Ok(#message_name::#field_name(Field {
                value: #value,
                scale: #scale,
                offset: #offset,
                units: #units,
            }))
        })
    });

    quote! {
        impl #message_name {
            pub(crate) fn decode<T: ByteOrder>(
                buffer: &[u8],
                field_def_num: u8,
            ) -> error::Result<Self> {
                match field_def_num {
                    #(#match_arms,)*
                    _ => Ok(#message_name::Unknown {
                        data: buffer.to_vec(),
                        field_def_num
                    }),
                }
            }
        }
    }
}

/// Extract `Message`s from the "Messages" worksheet.
pub fn extract(sheet: &Sheet) -> Vec<Message> {
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
                Row::Field {
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

            Some(Message::from(
                iter::once(header_row).chain(value_rows).collect::<Vec<_>>(),
            ))
        })
        .collect()
}

/// `Row` enumerates the *recognised* row formats within
/// the "Messages" worksheet.
#[derive(Debug)]
enum Row<'a> {
    GroupBanner {
        name: String,
    },
    Header {
        mesg_name: String,
        comment:   Option<String>,
    },
    Field {
        def_num:          u8,
        name:             String,
        type_:            String,
        scale:            Option<f64>,
        offset:           Option<f64>,
        units:            Option<String>,
        ref_field_names:  Option<Vec<String>>,
        ref_field_values: Option<Vec<String>>,
        comment:          Option<String>,
    },
    Empty,
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
    // NOTE: the logic below is very brittle. But at least
    // it is all contained here.
    fn from(row: &'a [calamine::DataType]) -> Self {
        #[rustfmt_skip]
        match *row {
            // A group of messages is headed by a sortof banner row
            // which has a SHOUTING label in the fourth column.
            [ calamine::DataType::Empty               // Message Name
            , calamine::DataType::Empty               // Field Definition Number
            , calamine::DataType::Empty               // Field Name
            , calamine::DataType::String(ref name)    // Field Type
            , ref rest..
            ] if util::is_shouting(name) && util::all_empty(rest) => {
                Row::GroupBanner {
                    name: name.to_pascal_case(),
                }
            },

            [ calamine::DataType::String(ref name)    // Message Name
            , calamine::DataType::Empty               // Field Definition Number
            , calamine::DataType::Empty               // Field Name
            , calamine::DataType::Empty               // Field Type
            , _                                       // Array
            , _                                       // Components
            , _                                       // Scale
            , _                                       // Offset
            , _                                       // Units
            , _                                       // Bits
            , _                                       // Accumulate
            , _                                       // Ref Field Name
            , _                                       // Ref Field Value
            , ref comment_cell                        // Comment
            , ..
            ] => {
                Row::Header {
                    mesg_name: name.to_pascal_case(),
                    comment: match comment_cell {
                        calamine::DataType::String(comment) =>
                            Some(sanitize_comment(comment)),
                        _ =>
                            None,
                    }
                }
            },

            [ calamine::DataType::Empty               // Message Name
            , calamine::DataType::Float(def_num)      // Field Definition Number
            , calamine::DataType::String(ref name)    // Field Name
            , calamine::DataType::String(ref type_)   // Field Type
            , _                                       // Array
            , _                                       // Components
            , ref scale_cell                          // Scale
            , ref offset_cell                         // Offset
            , ref units_cell                          // Units
            , _                                       // Bits
            , _                                       // Accumulate
            , ref ref_field_name_cell                 // Ref Field Name
            , ref ref_field_value_cell                // Ref Field Value
            , ref comment_cell                        // Comment
            , ..
            ] => {
                Row::Field {
                    def_num: def_num as u8,
                    name: name.to_pascal_case(),
                    type_: type_.to_string(),
                    scale: match *scale_cell {
                        calamine::DataType::Float(scale) =>
                            Some(scale),
                        calamine::DataType::Int(scale) =>
                            Some(scale as f64),
                        _ =>
                            None,
                    },
                    offset: match *offset_cell {
                        calamine::DataType::Float(offset) =>
                            Some(offset),
                        calamine::DataType::Int(offset) =>
                            Some(offset as f64),
                        _ =>
                            None,
                    },
                    units: match units_cell {
                        calamine::DataType::String(units) =>
                            Some(units.to_string()),
                        _ =>
                            None,
                    },
                    ref_field_names: match ref_field_name_cell {
                        calamine::DataType::String(names) =>
                            Some(names.split(",").map(String::from).collect()),
                        _ =>
                            None,
                    },
                    ref_field_values: match ref_field_value_cell {
                        calamine::DataType::String(values) =>
                            Some(values.split(",").map(String::from).collect()),
                        _ =>
                            None,
                    },
                    comment: match comment_cell {
                        calamine::DataType::String(comment) =>
                            Some(sanitize_comment(comment)),
                        _ =>
                            None,
                    }
                }
            },

            ref row if util::all_empty(row) => Row::Empty,

            ref row => Row::Unknown(row),
        }
    }
}

// Convert from a sanitised `Row` representation to the
// actual `Message` struct.
impl<'a> From<Vec<Row<'a>>> for Message {
    fn from(rows: Vec<Row>) -> Self {
        rows.into_iter().fold(
            Message {
                name:    String::new(),
                fields:  Vec::new(),
                comment: None,
            },
            |mut acc, row| {
                match row {
                    Row::Header {
                        mesg_name,
                        comment,
                    } => {
                        acc.name = mesg_name;
                        acc.comment = comment;
                    },
                    Row::Field {
                        def_num,
                        name,
                        type_,
                        scale,
                        offset,
                        units,
                        ref_field_names,
                        ref_field_values,
                        comment,
                    } => {
                        let field = Field {
                            name,
                            def_num,
                            type_,
                            scale,
                            offset,
                            units,
                            refs: ref_field_names.and_then(|names| {
                                ref_field_values.and_then(|values| {
                                    Some(
                                        names.into_iter().zip(values).collect(),
                                    )
                                })
                            }),
                            comment,
                        };
                        acc.fields.push(field);
                    },
                    _ => (),
                };
                acc
            },
        )
    }
}

/// Comments are converted to rust comments, so need to
/// escape things that look like markdown.
fn sanitize_comment(comment: &str) -> String {
    comment.replace("[", "\\[").replace("]", "\\]")
}
