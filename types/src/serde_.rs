//! Serde Support

use crate::{Error, Lei, lei};
use core::fmt::{Formatter, Result as FmtResult};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

impl Serialize for &lei {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

struct LeiVisitor;

impl<'de> Visitor<'de> for LeiVisitor {
    type Value = &'de lei;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("a borrowed lei string")
    }

    fn visit_borrowed_str<E: DeError>(self, v: &'de str) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v.as_bytes())
    }

    fn visit_borrowed_bytes<E: DeError>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        lei::from_bytes(v).map_err(|err| match err {
            Error::InvalidLength(len, _expected) => {
                DeError::invalid_length(len, &"20 ASCII digits and upper-case characters")
            }
            Error::InvalidCharacter(pos) => DeError::invalid_value(
                Unexpected::Char(char::from_u32(u32::from(v[pos])).unwrap_or_default()),
                &"A-Z, 0-9",
            ),
            Error::CheckDigitParse => DeError::invalid_value(
                Unexpected::Bytes(v),
                &"20 ASCII digits and upper-case characters that correctly generate check digits",
            ),
            Error::CheckDigitFail => DeError::invalid_value(
                Unexpected::Bytes(v),
                &"20 ASCII digits and upper-case characters with matching check digits",
            ),
        })
    }
}

impl<'de> Deserialize<'de> for &'de lei {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(LeiVisitor)
    }
}

impl Serialize for Lei {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

struct OwnedLeiVisitor;

impl<'de> Visitor<'de> for OwnedLeiVisitor {
    type Value = Lei;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str("a LEI string")
    }

    #[cfg(feature = "alloc")]
    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        self.visit_str(&v)
    }

    #[cfg(feature = "alloc")]
    fn visit_byte_buf<E: DeError>(self, v: Vec<u8>) -> Result<Self::Value, E> {
        self.visit_bytes(&v)
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v.as_bytes())
    }

    fn visit_borrowed_str<E: DeError>(self, v: &'de str) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v.as_bytes())
    }

    fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
        self.visit_borrowed_bytes(v)
    }

    fn visit_borrowed_bytes<E: DeError>(self, v: &'de [u8]) -> Result<Self::Value, E> {
        Lei::from_bytes(v).map_err(|err| match err {
            Error::InvalidLength(len, _expected) => {
                DeError::invalid_length(len, &"20 ASCII digits and upper-case characters")
            }
            Error::InvalidCharacter(pos) => DeError::invalid_value(
                Unexpected::Char(char::from_u32(u32::from(v[pos])).unwrap_or_default()),
                &"A-Z, 0-9",
            ),
            Error::CheckDigitParse => DeError::invalid_value(
                Unexpected::Bytes(v),
                &"20 ASCII digits and upper-case characters that correctly generate check digits",
            ),
            Error::CheckDigitFail => DeError::invalid_value(
                Unexpected::Bytes(v),
                &"20 ASCII digits and upper-case characters with matching check digits",
            ),
        })
    }
}

impl<'de> Deserialize<'de> for Lei {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(OwnedLeiVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::{Lei, lei};
    use alloc::{collections::BTreeMap, string::String};
    use core::ops::Deref;

    const LEI_VALUE: &str = "YZ83GD8L7GG84979J516";

    #[test]
    fn roundtrip() {
        let l = lei::from_str_slice(LEI_VALUE).expect("Could not construct LEI slice");
        let val = serde_json::json!({
            "lei": l,
        });

        // serialize
        let out = serde_json::to_string(&val).expect("Could not serialize value");
        assert_eq!("{\"lei\":\"YZ83GD8L7GG84979J516\"}", &out);

        // deserialize
        let val = serde_json::from_str::<BTreeMap<String, Lei>>(&out)
            .expect("Could not deserialize from JSON");

        assert_eq!(val.get("lei").map(Deref::deref), Some(l));
    }
}
