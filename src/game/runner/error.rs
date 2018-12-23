// File: error.rs
// File Created: 23 Dec 2018 14:17
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    InputError(String),
    InvalidArgumentsError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::InputError(e.to_string())
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::ParseError(e.to_string())
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::ParseError(_) => match other {
                Error::ParseError(_) => true,
                _ => false,
            },

            Error::InputError(_) => match other {
                Error::InputError(_) => true,
                _ => false,
            },

            Error::InvalidArgumentsError => match other {
                Error::InvalidArgumentsError => true,
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partial_eq_error() {
        assert!(Error::ParseError(String::default()) == Error::ParseError(String::default()));
        assert!(Error::InputError(String::default()) == Error::InputError(String::default()));
        assert!(Error::InvalidArgumentsError == Error::InvalidArgumentsError);
        assert!(!(Error::ParseError(String::default()) == Error::InvalidArgumentsError));
        assert!(!(Error::InputError(String::default()) == Error::InvalidArgumentsError));
        assert!(!(Error::InvalidArgumentsError == Error::ParseError(String::default())));
    }

    #[test]
    fn test_from_io_error() {
        let ioerr = io::Error::from(io::ErrorKind::AddrInUse);
        let err = Error::from(ioerr);
        assert_eq!(err, Error::InputError(String::default()));
    }

    #[test]
    fn test_from_parse_int_error() {
        let parse_int_err = "a".parse::<usize>().expect_err("expected an error");
        let err = Error::from(parse_int_err);
        assert_eq!(err, Error::ParseError(String::default()));
    }
}
