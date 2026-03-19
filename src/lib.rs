use std::fmt;
use std::str::FromStr;

/// A 5-digit IRS Electronic Transmitter Identification Number (ETIN).
///
/// An ETIN is assigned by the IRS to authorized electronic return transmitters.
/// It is always exactly 5 ASCII digits (e.g. `"00111"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Etin(String);

/// Error returned when an ETIN string is invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseEtinError {
    /// The input was not exactly 5 ASCII digits.
    Invalid,
}

impl fmt::Display for ParseEtinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => f.write_str("invalid ETIN: must be exactly 5 ASCII digits"),
        }
    }
}

impl std::error::Error for ParseEtinError {}

impl Etin {
    /// Create a new `Etin` after validating the input.
    pub fn new(s: &str) -> Result<Self, ParseEtinError> {
        s.parse()
    }

    /// Return the ETIN as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Etin {
    type Err = ParseEtinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 5 && s.bytes().all(|b| b.is_ascii_digit()) {
            Ok(Self(s.to_owned()))
        } else {
            Err(ParseEtinError::Invalid)
        }
    }
}

impl fmt::Display for Etin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Etin {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_etin() {
        let e = Etin::new("00111").unwrap();
        assert_eq!(e.as_str(), "00111");
        assert_eq!(e.to_string(), "00111");
    }

    #[test]
    fn too_short() {
        assert!(Etin::new("1234").is_err());
    }

    #[test]
    fn too_long() {
        assert!(Etin::new("123456").is_err());
    }

    #[test]
    fn non_digit() {
        assert!(Etin::new("abcde").is_err());
    }

    #[test]
    fn from_str_trait() {
        let e: Etin = "00112".parse().unwrap();
        assert_eq!(e.as_str(), "00112");
    }

    #[test]
    fn equality() {
        let a = Etin::new("00001").unwrap();
        let b = Etin::new("00002").unwrap();
        assert_ne!(a, b);
        assert_eq!(a, a.clone());
    }
}
