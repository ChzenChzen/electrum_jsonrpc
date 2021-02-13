use std::net::AddrParseError;
use std::fmt;
use std::error;
use crate::error::ElectrumRpcError::AddressError;

pub type Result<T> = std::result::Result<T, ElectrumRpcError>;

pub enum ElectrumRpcError {
    AddressError(AddrParseError),
}

impl fmt::Display for ElectrumRpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError(..) => write!(f, "the provided address couldn't parsed."),
        }
    }
}

impl fmt::Debug for ElectrumRpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <ElectrumRpcError as std::fmt::Display>::fmt(self, f)
    }
}

impl error::Error for ElectrumRpcError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            AddressError(ref e) => e.source(),
        }
    }
}

impl From<AddrParseError> for ElectrumRpcError {
    fn from(err: AddrParseError) -> Self {
        Self::AddressError(err)
    }
}
