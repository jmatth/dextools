use reqwest;
use serde_json;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::option::NoneError;

#[derive(Debug)]
pub enum Error {
	IOError(io::Error),
	ReqwestError(reqwest::Error),
	ReqwestHeaderStrError(reqwest::header::ToStrError),
	StringError(String),
	JsonError(serde_json::Error),
	NoneError(),
	ChronoParseError(chrono::ParseError),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::IOError(err) => write!(f, "IO error: {:?}", err),
			Error::NoneError() => write!(f, "Unexpected empty Option"),
			Error::ReqwestError(err) => write!(f, "HTTP error: {:?}", err),
			Error::ReqwestHeaderStrError(err) => {
				write!(f, "Error converting header to str: {:?}", err)
			}
			Error::StringError(err) => write!(f, "Unknown error: {}", err),
			Error::JsonError(err) => write!(f, "JSON error: {:?}", err),
			Error::ChronoParseError(err) => write!(f, "Chrono parse error: {}", err),
		}
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}

impl From<NoneError> for Error {
	fn from(_: NoneError) -> Self {
		Error::NoneError()
	}
}

impl From<io::Error> for Error {
	fn from(error: io::Error) -> Self {
		Error::IOError(error)
	}
}

impl From<reqwest::Error> for Error {
	fn from(error: reqwest::Error) -> Self {
		Error::ReqwestError(error)
	}
}

impl From<reqwest::header::ToStrError> for Error {
	fn from(error: reqwest::header::ToStrError) -> Self {
		Error::ReqwestHeaderStrError(error)
	}
}

impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Self {
		Error::JsonError(error)
	}
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Error::StringError(error)
	}
}

impl From<&str> for Error {
	fn from(error: &str) -> Self {
		Error::StringError(error.to_string())
	}
}

impl From<chrono::ParseError> for Error {
	fn from(error: chrono::ParseError) -> Self {
		Error::ChronoParseError(error)
	}
}
