#[macro_use]
extern crate failure;

use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "I/O Error")]
    Io,
    #[fail(display = "{:?}", msg)]
    Str { msg: String },
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        ErrorKind::Str {
            msg: error.to_owned(),
        }.into()
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error: &'a str) -> Error {
        ErrorKind::Str {
            msg: error.to_owned(),
        }.into()
    }
}
