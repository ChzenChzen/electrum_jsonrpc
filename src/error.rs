use std::net::AddrParseError;
use std::fmt;
use std::error;
use crate::error::ElectrumRpcError::AddressError;
use std::fmt::Display;
use hyper::http::uri::InvalidUri;

pub type Result<T> = std::result::Result<T, ElectrumRpcError>;

pub enum ElectrumRpcError {
    AddressError(InvalidUri),
}

impl fmt::Display for ElectrumRpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError(_) => write!(f, "the provided address couldn't parsed."),
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
        match &*self {
            AddressError(ref e) => Some(e),
        }
    }
}

impl From<InvalidUri> for ElectrumRpcError {
    fn from(err: InvalidUri) -> Self {
        Self::AddressError(err)
    }
}
