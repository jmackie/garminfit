use failure::{
    Backtrace,
    Context,
    Fail,
};
use std::{
    convert::Into,
    fmt,
    result,
};

/// A type alias for handling errors throughout this crate.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur while decoding a fit-encoded
/// data.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub(crate) fn reading<S, E>(what: S) -> impl FnOnce(E) -> Error
    where
        S: Into<String>,
        E: Fail,
    {
        move |err| {
            Error::from(err.context(ErrorKind::Read {
                what: what.into()
            }))
        }
    }

    pub(crate) fn decoding<S, E>(what: S) -> impl FnOnce(E) -> Error
    where
        S: Into<String>,
        E: Fail,
    {
        move |err| {
            Error::from(err.context(ErrorKind::Decode {
                what: what.into()
            }))
        }
    }

    pub(crate) fn seek<E: Fail>(err: E) -> Error {
        Error::from(err.context(ErrorKind::Seek))
    }

    pub(crate) fn unknown_file_header_size(size: u8) -> Error {
        Error::from(ErrorKind::UnknownFileHeaderSize(size))
    }

    pub(crate) fn unsupported_protocol_version(
        upper_bound: u8,
        got: u8,
    ) -> Error {
        Error::from(ErrorKind::UnsupportedProtocolVersion {
            upper_bound,
            got,
        })
    }

    pub(crate) fn unknown_architecture(arch: u8) -> Error {
        Error::from(ErrorKind::UnknownArchitecture(arch))
    }

    pub(crate) fn unknown_base_type(type_id: u8) -> Error {
        Error::from(ErrorKind::UnknownBaseType {
            type_id,
        })
    }

    pub(crate) fn unknown_message(num: u16) -> Error {
        Error::from(ErrorKind::UnknownMessage(num))
    }

    pub(crate) fn unknown_field(num: u8) -> Error {
        Error::from(ErrorKind::UnknownField(num))
    }

    pub(crate) fn unknown_type<S: Into<String>>(
        type_name: S,
        type_id: usize,
    ) -> Error {
        Error::from(ErrorKind::UnknownType {
            type_name: type_name.into(),
            type_id,
        })
    }

    pub(crate) fn not_fit() -> Error {
        Error::from(ErrorKind::NotFit)
    }

    pub(crate) fn missing_definition(key: u8) -> Error {
        Error::from(ErrorKind::MissingDefinition(key))
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

/// The specific kind of error that can occur.
pub enum ErrorKind {
    /// An error occured while attempting to read bytes.
    Read { what: String },
    /// An error occured while attempting to decode data.
    Decode { what: String },
    /// An error occured while seeking.
    Seek,
    /// Encountered a bad file header size.
    UnknownFileHeaderSize(u8),
    /// Encountered an unsupported protocol version.
    UnsupportedProtocolVersion {
        /// The highest supported (major) version.
        upper_bound: u8,
        /// The version we got.
        got: u8,
    },
    /// Encountered an unknown base type identifier.
    UnknownBaseType { type_id: u8 },
    /// Encountered an unknown type identifier.
    UnknownType { type_name: String, type_id:   usize },
    /// Bad file magic.
    NotFit,
    /// Encountered an unknown file architecture.
    UnknownArchitecture(u8),
    /// Encountered an unknown message number.
    UnknownMessage(u16),
    /// Encountered an unknown field definition number.
    UnknownField(u8),
    /// A data message referenced an unknown definition
    /// message.
    MissingDefinition(u8),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Read {
                ref what,
            } => write!(f, "reading {}", what),

            ErrorKind::Decode {
                ref what,
            } => write!(f, "decoding {}", what),

            ErrorKind::Seek => write!(f, "seek error"),

            ErrorKind::UnknownFileHeaderSize(size) => {
                write!(f, "bad file header size: {}", size)
            },

            ErrorKind::UnsupportedProtocolVersion {
                upper_bound,
                got,
            } => {
                write!(
                    f,
                    "only protocol versions <= {}.X are currently supported: \
                     got {}",
                    upper_bound, got
                )
            },

            ErrorKind::UnknownArchitecture(arch) => {
                write!(f, "wierd architecture, expecting 0 or 1: got {}", arch)
            },

            ErrorKind::UnknownBaseType {
                type_id,
            } => write!(f, "unknown base type id: {}", type_id),

            ErrorKind::UnknownMessage(num) => {
                write!(f, "encountered unknown message number: {}", num)
            },
            ErrorKind::UnknownField(num) => {
                write!(
                    f,
                    "encountered unknown field definition number: {}",
                    num
                )
            },

            ErrorKind::UnknownType {
                ref type_name,
                type_id,
            } => {
                write!(
                    f,
                    "unknown type: {} does not map to any {}",
                    type_id, type_name
                )
            },

            ErrorKind::NotFit => {
                write!(f, "bad file magic, probably not a FIT file")
            },

            ErrorKind::MissingDefinition(key) => {
                write!(
                    f,
                    "data message referenced an unknown definition message \
                     (key: {})",
                    key
                )
            },
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error {
            ctx,
        }
    }
}
