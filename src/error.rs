use std::fmt;
use std::error;
use std::fmt::Display;
use hyper::http::uri::InvalidUri;
use std::borrow::BorrowMut;

pub type Result<T> = std::result::Result<T, ElectrumRpcError>;

pub enum ElectrumRpcError {
    AddressError(InvalidUri),
    HyperHttpError(hyper::http::Error),
    HyperHttpStreamError(hyper::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for ElectrumRpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AddressError(e) => write!(f, "the provided address couldn't parsed: {}", e),
            Self::HyperHttpError(e) => write!(f, "while calling method was occurred error: {}", e),
            Self::HyperHttpStreamError(e) => write!(f, "while sending request was occurred error: {}", e),
            Self::JsonError(e) => write!(f, "while working with json was occurred error: {}", e),
        }
    }
}

impl fmt::Debug for ElectrumRpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl error::Error for ElectrumRpcError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::AddressError(ref e) => Some(e),
            Self::HyperHttpError(ref e) => Some(e),
            Self::HyperHttpStreamError(ref e) => Some(e),
            Self::JsonError(ref e) => Some(e),
        }
    }
}

impl From<InvalidUri> for ElectrumRpcError {
    fn from(err: InvalidUri) -> Self {
        Self::AddressError(err)
    }
}

impl From<hyper::http::Error> for ElectrumRpcError {
    fn from(err: hyper::http::Error) -> Self {
        Self::HyperHttpError(err)
    }
}

impl From<hyper::Error> for ElectrumRpcError {
    fn from(err: hyper::Error) -> Self {
        Self::HyperHttpStreamError(err)
    }
}

impl From<serde_json::Error> for ElectrumRpcError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err)
    }
}