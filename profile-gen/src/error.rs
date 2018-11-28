use failure::{
    Backtrace,
    Context,
    Fail,
};
use std::{
    convert::Into,
    fmt,
    path::PathBuf,
    result,
};

/// A type alias for handling errors throughout this crate.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur while processing the FIT
/// profile.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub(crate) fn opening<E: Fail>(path: PathBuf) -> impl FnOnce(E) -> Error {
        |err| Error::from(err.context(ErrorKind::Open(path)))
    }

    pub(crate) fn missing_sheet<S>(name: S) -> Error
    where
        S: Into<String>,
    {
        Error::from(ErrorKind::MissingSheet(name.into()))
    }

    pub(crate) fn bad_sheet<E, S>(name: S) -> impl FnOnce(E) -> Error
    where
        S: Into<String>,
    {
        |_| Error::from(ErrorKind::BadSheet(name.into()))
    }

    pub(crate) fn rustfmt<E: Fail>(err: E) -> Error {
        Error::from(err.context(ErrorKind::Rustfmt))
    }

    pub fn missing_type(type_name: String) -> Error {
        Error::from(ErrorKind::MissingType(type_name))
    }

    pub(crate) fn missing_message(mesg_name: String) -> Error {
        Error::from(ErrorKind::MissingMessage(mesg_name))
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
    /// An error occurred while trying to open a workbook.
    Open(PathBuf),
    /// An error occured while trying to open a worksheet.
    MissingSheet(String),
    /// An error occured while trying to read a worksheet.
    BadSheet(String),
    /// An error occured while formatting some code.
    Rustfmt,
    /// An error occurred while trying to access a specific
    /// type.
    MissingType(String),
    /// An error occurred while trying to access a specific
    /// message.
    MissingMessage(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::Open(ref path) => write!(f, "error opening {:?}", path),
            ErrorKind::MissingSheet(ref name) => {
                write!(f, "{} sheet not found", name)
            },
            ErrorKind::BadSheet(ref name) => {
                write!(f, "error trying to read {} worksheet", name)
            },
            ErrorKind::Rustfmt => write!(f, "error while formatting code"),
            ErrorKind::MissingType(ref type_name) => {
                write!(f, "{} type not found", type_name)
            },
            ErrorKind::MissingMessage(ref mesg_name) => {
                write!(f, "{} message not found", mesg_name)
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
