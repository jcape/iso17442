//! ISO 17442 Types

#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod alloc_;
#[cfg(feature = "serde")]
mod serde_;

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};
use core::{
    borrow::Borrow,
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};
use ref_cast::{RefCastCustom, ref_cast_custom};
use thiserror::Error as ThisError;

/// The size of a Legal Entity ID
const LEI_SIZE: usize = 20;

/// The size of an LOU
const ISSUER_SIZE: usize = 4;

const LOU_START: usize = 0;
const LOU_END: usize = LOU_START + ISSUER_SIZE;

/// The size of an entry
const ID_SIZE: usize = 14;

const ID_START: usize = LOU_END;
const ID_END: usize = ID_START + ID_SIZE;

/// The size of the checked portion of an LEI
const CHECKED_SIZE: usize = ISSUER_SIZE + ID_SIZE;

/// The position of the tens digit of the checksum
const CHECK_TENS_POS: usize = 18;

/// The position of the ones didit of the checksum
const CHECK_ONES_POS: usize = 19;

const fn validate(bytes: &[u8]) -> Result<(), Error> {
    if bytes.len() != LEI_SIZE {
        return Err(Error::InvalidLength(bytes.len(), LEI_SIZE));
    }

    let mut check_str_bytes = [0u8; LEI_SIZE * 2];

    let mut i = 0;
    let mut check_pos = 0;
    while i < CHECKED_SIZE {
        if bytes[i].is_ascii_uppercase() {
            let checkval = bytes[i] - 55;
            let tens = checkval / 10;
            let ones = checkval % 10;
            check_str_bytes[check_pos] = tens + 48;
            check_pos += 1;
            check_str_bytes[check_pos] = ones + 48;
            check_pos += 1;
        } else if bytes[i].is_ascii_digit() {
            check_str_bytes[check_pos] = bytes[i];
            check_pos += 1;
        } else {
            return Err(Error::InvalidCharacter(i));
        }

        i += 1;
    }

    check_str_bytes[check_pos] = b'0';
    check_pos += 1;
    check_str_bytes[check_pos] = b'0';
    check_pos += 1;

    let (check_bytes, _trailer) = check_str_bytes.as_slice().split_at(check_pos);

    // SAFETY: We are building these bytes ourselves from ascii characters
    #[allow(unsafe_code)]
    let src = unsafe { str::from_utf8_unchecked(check_bytes) };

    let result = u128::from_str_radix(src, 10);
    if let Ok(check_sum) = result {
        let check_digits = 98 - (check_sum % 97);
        if check_digits < 1 || check_digits > 98 {
            return Err(Error::CheckDigitFail);
        }

        #[allow(clippy::cast_possible_truncation)]
        let tens = check_digits as u8 / 10;
        #[allow(clippy::cast_possible_truncation)]
        let ones = check_digits as u8 % 10;

        if bytes[CHECK_TENS_POS] != tens + 48 || bytes[CHECK_ONES_POS] != ones + 48 {
            Err(Error::CheckDigitFail)
        } else {
            Ok(())
        }
    } else {
        Err(Error::CheckDigitParse)
    }
}

/// An enumeration of errors
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ThisError)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Error {
    /// The string has the wrong length for an LEI.
    #[error("The string has the wrong length for an LEI.")]
    InvalidLength(usize, usize),

    /// The string contains invalid characters for an LEI.
    #[error("The string contains an invalid character at {0} for an LEI.")]
    InvalidCharacter(usize),

    /// The check digits string could not be parsed.
    #[error("The check digits string could not be parsed.")]
    CheckDigitParse,

    /// The check digits did not validate.
    #[error("The check digits did not validate.")]
    CheckDigitFail,
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Self::CheckDigitParse
    }
}

/// A Legal Entity ID
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCastCustom)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct lei([u8]);

impl lei {
    #[ref_cast_custom]
    pub(crate) const fn ref_cast(bytes: &[u8]) -> &Self;

    /// Create a new LEI reference from a byte slice.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] when the given string is of the wrong length.
    /// - [`Error::InvalidCharacter`] when the given string contains an invalid character.
    /// - [`Error::CheckDigitParse`] when the check digits contain invalid characters.
    /// - [`Error::CheckDigitFail`] when the check digit does not match the ID.
    pub const fn from_bytes(bytes: &[u8]) -> Result<&Self, Error> {
        if let Err(e) = validate(bytes) {
            Err(e)
        } else {
            Ok(Self::ref_cast(bytes))
        }
    }

    /// Create a new LEI reference from a string slice.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] when the given string is of the wrong length.
    /// - [`Error::InvalidCharacter`] when the given string contains an invalid character.
    /// - [`Error::CheckDigitParse`] when the check digits contain invalid characters.
    /// - [`Error::CheckDigitFail`] when the check digit does not match the ID.
    pub const fn from_str_slice(s: &str) -> Result<&Self, Error> {
        lei::from_bytes(s.as_bytes())
    }

    /// Get a reference to the byte slice backing this string.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get a reference to the validated LEI reference as a string slice.
    #[allow(unsafe_code)]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        // SAFETY: The validate function ensures that only ascii uppercase and digit characters are
        // contined in this slice
        unsafe { str::from_utf8_unchecked(&self.0) }
    }

    /// Split this LEI into three parts: issuer, ID, and check digit.
    #[must_use]
    pub const fn split(&self) -> (&str, &str, u8) {
        // SAFETY: The validate function ensures that only ascii uppercase and digit characters are
        // contined in this slice
        let whole = self.as_str();

        let (issuer, remainder) = whole.split_at(LOU_END);
        let (id, check_digits) = remainder.split_at(ID_END);

        if let Ok(val) = u8::from_str_radix(check_digits, 10) {
            (issuer, id, val)
        } else {
            panic!("Unparseable check digits somehow passed validation.");
        }
    }

    /// The issuer of this LEI as a string slice.
    #[must_use]
    pub const fn lou(&self) -> &str {
        let (issuer, _remainder) = self.as_str().split_at(LOU_END);
        issuer
    }

    /// The ID part of this LEI as a string slice.
    #[must_use]
    pub const fn id(&self) -> &str {
        let (_issuer, remainder) = self.as_str().split_at(LOU_END);
        let (id, _remainder) = remainder.split_at(ID_END);
        id
    }

    /// The check digit of this LEI, as an unsigned integer between 2 and 97.
    #[must_use]
    pub const fn check_digits(&self) -> u8 {
        self.split().2
    }
}

impl AsRef<[u8]> for lei {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for lei {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for lei {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/// An owned Legal Entity ID
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Lei([u8; LEI_SIZE]);

impl Lei {
    /// Create a new owned LEI from the given LEI borrow.
    #[must_use]
    pub const fn from_lei(src: &lei) -> Self {
        Self::from_bytes_unchecked(src.as_bytes())
    }

    /// Create a new owned Legal Entity ID from the give byte slice.
    ///
    /// This will copy the bytes into a new owned LEI structure.
    ///
    /// # Examples
    /// ```
    /// use iso17442_types::Lei;
    ///
    /// const LEI_BYTES: &[u8] = b"YZ83GD8L7GG84979J516";
    ///
    /// let l = Lei::from_bytes(LEI_BYTES).expect("Could not parse LEI bytes");
    ///
    /// assert_eq!(LEI_BYTES, l.as_bytes());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] when the given string is of the wrong length.
    /// - [`Error::InvalidCharacter`] when the given string contains an invalid character.
    /// - [`Error::CheckDigitParse`] when the check digits contain invalid characters.
    /// - [`Error::CheckDigitFail`] when the check digit does not match the ID
    pub const fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if let Err(e) = validate(bytes) {
            Err(e)
        } else {
            Ok(Self::from_bytes_unchecked(bytes))
        }
    }

    /// Create a new owned LEI from the given byte array.
    ///
    /// # Examples
    /// ```
    /// use iso17442_types::Lei;
    ///
    /// const LEI_BYTES: [u8; 20] = *b"YZ83GD8L7GG84979J516";
    ///
    /// let l = Lei::from_byte_array(LEI_BYTES.clone()).expect("Could not parse LEI bytes");
    ///
    /// assert_eq!(&LEI_BYTES, l.as_bytes());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] when the given string is of the wrong length.
    /// - [`Error::InvalidCharacter`] when the given string contains an invalid character.
    /// - [`Error::CheckDigitParse`] when the check digits contain invalid characters.
    /// - [`Error::CheckDigitFail`] when the check digit does not match the ID.
    pub const fn from_byte_array(bytes: [u8; LEI_SIZE]) -> Result<Self, Error> {
        if let Err(e) = validate(&bytes) {
            Err(e)
        } else {
            Ok(Self(bytes))
        }
    }

    /// Create a new owned LEI from the given string slice.
    ///
    /// # Examples
    /// ```
    /// use iso17442_types::Lei;
    ///
    /// const LEI_STR: &str = "YZ83GD8L7GG84979J516";
    ///
    /// let l = Lei::from_str_slice(LEI_STR).expect("Could not parse LEI bytes");
    ///
    /// assert_eq!(LEI_STR, l.as_str());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidLength`] when the given string is of the wrong length.
    /// - [`Error::InvalidCharacter`] when the given string contains an invalid character.
    /// - [`Error::CheckDigitParse`] when the check digits contain invalid characters.
    /// - [`Error::CheckDigitFail`] when the check digit does not match the ID.
    pub const fn from_str_slice(src: &str) -> Result<Self, Error> {
        Self::from_bytes(src.as_bytes())
    }

    /// Copy the given slice into bytes
    pub(crate) const fn from_bytes_unchecked(slice: &[u8]) -> Self {
        let mut bytes = [0u8; LEI_SIZE];
        bytes.copy_from_slice(slice);

        Self(bytes)
    }
}

impl Borrow<lei> for Lei {
    fn borrow(&self) -> &lei {
        self
    }
}

impl Deref for Lei {
    type Target = lei;

    fn deref(&self) -> &Self::Target {
        lei::ref_cast(&self.0)
    }
}

impl Display for Lei {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl From<&lei> for Lei {
    fn from(value: &lei) -> Self {
        Lei::from_lei(value)
    }
}

impl TryFrom<[u8; LEI_SIZE]> for Lei {
    type Error = Error;

    fn try_from(bytes: [u8; LEI_SIZE]) -> Result<Self, Self::Error> {
        Self::from_byte_array(bytes)
    }
}

impl TryFrom<&[u8]> for Lei {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes)
    }
}

impl FromStr for Lei {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_slice(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::borrow::ToOwned;
    use core::{borrow::Borrow, str::FromStr};

    #[yare::parameterized(
        ok_1 = { "YZ83GD8L7GG84979J516", None },
        poo = { "YZ83GD8L7GG849💩16", Some(Error::InvalidCharacter(14)) },
        bad_check_1 = { "YZ83GD8L7GG84979J563", Some(Error::CheckDigitFail) },
        bad_check_2 = { "315700K7NYVSQJNTN401", Some(Error::CheckDigitFail) },
        missing_check = { "315700K7NYVSQJNTN4", Some(Error::InvalidLength(18, LEI_SIZE)) },
        blank = { "", Some(Error::InvalidLength(0, LEI_SIZE)) },
    )]
    fn check(s: &str, err: Option<Error>) {
        let result = lei::from_str_slice(s);
        assert_eq!(err, result.err());

        if let Ok(l) = result {
            let owned = Lei::from_str(s).expect("Could not parse as owned?");
            assert_eq!(l.to_owned(), owned);
            assert_eq!(<Lei as Borrow<lei>>::borrow(&owned), l);
        }
    }
}
