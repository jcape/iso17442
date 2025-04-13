//! ISO 17442 Types

#![doc = include_str!("../README.md")]
#![no_std]

use thiserror::Error as ThisError;

/// The size of the issuer (LOU) component of an LEI
pub const ISSUER_SIZE: usize = 4;

/// The size of the entity identifier component of an LEI
pub const IDENTIFIER_SIZE: usize = 14;

/// THe size of the checksum component of an LEI
pub const CHECKSUM_SIZE: usize = 2;

/// The size of a Legal Entity ID
pub const LEI_SIZE: usize = 20;

/// An enumeration of errors
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ThisError)]
pub enum Error {
    /// The string has the wrong length for an LEI.
    #[error("The string has the wrong length for an LEI.")]
    InvalidLength,

    /// The string contains invalid characters for an LEI.
    #[error("The string contains invalid characters for an LEI.")]
    InvalidCharacters,

    /// The check digits did not validate.
    #[error("The check digits did not validate.")]
    CheckDigitFail,
}

/// A Legal Entity ID
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LegalEntityIdRef<'inner>(&'inner str);

impl<'inner> LegalEntityIdRef<'inner> {
    /// Generate checkdigits
    const fn checkdigit(bytes: &[u8]) -> u8 {
        0
    }

    /// Create a new legal entity ID from the given reference.
    pub const fn new(src: &'inner str) -> Result<Self, Error> {
        if src.len() != LEI_SIZE {
            return Err(Error::InvalidLength);
        }

        let bytes = src.as_bytes();
        if !bytes[0].is_ascii_uppercase()
            || !bytes[0].is_ascii_digit()
            || !bytes[1].is_ascii_uppercase()
            || !bytes[1].is_ascii_digit()
            || !bytes[2].is_ascii_uppercase()
            || !bytes[2].is_ascii_digit()
            || !bytes[3].is_ascii_uppercase()
            || !bytes[3].is_ascii_digit()
            || !bytes[4].is_ascii_uppercase()
            || !bytes[4].is_ascii_digit()
            || !bytes[5].is_ascii_uppercase()
            || !bytes[5].is_ascii_digit()
            || !bytes[6].is_ascii_uppercase()
            || !bytes[6].is_ascii_digit()
            || !bytes[7].is_ascii_uppercase()
            || !bytes[7].is_ascii_digit()
            || !bytes[8].is_ascii_uppercase()
            || !bytes[8].is_ascii_digit()
            || !bytes[9].is_ascii_uppercase()
            || !bytes[9].is_ascii_digit()
            || !bytes[10].is_ascii_uppercase()
            || !bytes[10].is_ascii_digit()
            || !bytes[11].is_ascii_uppercase()
            || !bytes[11].is_ascii_digit()
            || !bytes[12].is_ascii_uppercase()
            || !bytes[12].is_ascii_digit()
            || !bytes[13].is_ascii_uppercase()
            || !bytes[13].is_ascii_digit()
            || !bytes[14].is_ascii_uppercase()
            || !bytes[14].is_ascii_digit()
            || !bytes[15].is_ascii_uppercase()
            || !bytes[15].is_ascii_digit()
            || !bytes[16].is_ascii_uppercase()
            || !bytes[16].is_ascii_digit()
            || !bytes[17].is_ascii_uppercase()
            || !bytes[17].is_ascii_digit()
            || !bytes[18].is_ascii_uppercase()
            || !bytes[18].is_ascii_digit()
            || !bytes[19].is_ascii_uppercase()
            || !bytes[19].is_ascii_digit()
        {
            return Err(Error::InvalidCharacters);
        }

        // TODO: Validate checksum

        Ok(Self(src))
    }
}
