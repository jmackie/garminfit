//! NOTE: names should just be camel-cased versions of the
//! names in the FIT SDK.

use byteorder::ByteOrder;
use error::Result;
use std::{
    default::Default,
    f32,
    f64,
};

macro_rules! base_type {
    (
        $sdk_name:expr,
        $name:ident,
        $type:ident,
        $read_method:ident,
        $invalid:expr
    ) => {
        #[doc=$sdk_name]
        #[derive(Debug)]
        pub struct $name(pub $type);

        impl $name {
            base_type_decode!($name, $read_method);

            pub(crate) fn is_valid(&self) -> bool {
                self.0 == $invalid
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name($invalid)
            }
        }
    };
}

macro_rules! base_type_decode {
    ($name:ident,read_u8) => {
        pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> Result<Self> {
            Ok($name(buffer[0]))
        }
    };
    ($name:ident,read_i8) => {
        pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> Result<Self> {
            Ok($name(buffer[0] as i8))
        }
    };
    ($name:ident, $read_method:ident) => {
        pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> Result<Self> {
            let x = T::$read_method(buffer);
            Ok($name(x))
        }
    };
}

base_type!("enum", Enum, u8, read_u8, 0xFF);

base_type!("sint8", Sint8, i8, read_i8, 0x7F); // 2's complement format
base_type!("uint8", Uint8, u8, read_u8, 0xFF);

base_type!("sint16", Sint16, i16, read_i16, 0x7FFF); // 2's complement format
base_type!("uint16", Uint16, u16, read_u16, 0xFFFF);

base_type!("sint32", Sint32, i32, read_i32, 0x7FFFFF); // 2's complement format
base_type!("uint32", Uint32, u32, read_u32, 0xFFFFFF);

base_type!("float32", Float32, f32, read_f32, f32::MAX);
base_type!("float64", Float64, f64, read_f64, f64::MAX);

base_type!("uint8z", Uint8z, u8, read_u8, 0x00);
base_type!("uint16z", Uint16z, u16, read_u16, 0x0000);
base_type!("uint32z", Uint32z, u32, read_u32, 0x00000000);

base_type!("sint64", Sint64, i64, read_i64, 0x7FFFFFFFFFFFFFFF); // 2's complement format
base_type!("uint64", Uint64, u64, read_u64, 0xFFFFFFFFFFFFFFFF);
base_type!("uint64z", Uint64z, u64, read_u64, 0x0000000000000000);

/// "string"
/// Null terminated string encoded in UTF-8 format.
#[derive(Debug)]
pub struct Utf8String(pub String);

impl Utf8String {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> Result<Self> {
        unsafe {
            let string = String::from_utf8_unchecked(buffer.to_vec());
            Ok(Utf8String(string))
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        true // TODO?
    }
}

impl Default for Utf8String {
    fn default() -> Self {
        Utf8String(String::new())
    }
}

/// "byte"
/// Array of bytes.  Field is invalid if all bytes are
/// invalid.
#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> Result<Self> {
        Ok(Bytes(buffer.to_vec()))
    }

    pub(crate) fn is_valid(&self) -> bool {
        true // TODO?
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Bytes(Vec::new())
    }
}

/// "bool"
/// TODO: Because it doesn't seem to be documented anywhere.
#[derive(Debug)]
pub struct Bool(pub bool);

impl Bool {
    pub(crate) fn decode<T: ByteOrder>(buffer: &[u8]) -> Result<Self> {
        panic!(
            "bool hasn't been implemented yet, but here's what it should \
             decode: {:?}",
            buffer
        )
    }

    pub(crate) fn is_valid(&self) -> bool {
        true // TODO?
    }
}

impl Default for Bool {
    fn default() -> Self {
        Bool(false)
    }
}
