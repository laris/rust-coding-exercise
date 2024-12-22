use std::fmt;
use std::convert::TryFrom;
use std::str::FromStr;
use crate::{Error, Result};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChunkType {
    data1: u8,
    data2: u8,
    data3: u8,
    data4: u8,
}
impl fmt::Display for ChunkType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{}, {}, {}, {}", self.data1, self.data2, self.data3, self.data4);
        Ok(())
    }
}
impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [self.data1, self.data2, self.data3, self.data4]
    }

    fn is_valid(&self) -> bool {
        self.data1.is_ascii_alphabetic() &&
        self.data2.is_ascii_alphabetic() &&
        self.data3.is_ascii_alphabetic() &&
        self.data4.is_ascii_alphabetic()
    }

    fn is_critical(&self) -> bool {
        /*
        if self.data1 & 0b0010_0000 == 0 {
            true
        } else {
            false
        }
        */
        self.data1.is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.data2.is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.data3.is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.data4.is_ascii_lowercase()
    }

    fn to_string(&self) -> String {
        std::string::String::from_utf8(vec![self.data1, self.data2, self.data3, self.data4]).unwrap()
    }
}
/*
impl ToString for ChunkType {
    fn to_string(&self) -> String {
        std::string::String::from_utf8(vec![self.data1, self.data2, self.data3, self.data4])
    }
}
impl From<String> for ChunkType {
    //type Error = Error;
    fn from(string: String) -> Self {
        let bytes = string.into_bytes();
        Self { data1: bytes[0], data2: bytes[1], data3: bytes[2],data4: bytes[3], }
    }
}
*/

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        //Ok(Self { data1: bytes[0], data2: bytes[1], data3: bytes[2],data4: bytes[3], })
        let chunktype = Self { data1: bytes[0], data2: bytes[1], data3: bytes[2],data4: bytes[3], };
        if chunktype.is_valid() {
            Ok(chunktype)
        } else {
            Err("invalid chunk type".into())
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        let chunktype = Self { data1: bytes[0], data2: bytes[1], data3: bytes[2],data4: bytes[3], };
        if chunktype.is_valid() {
            Ok(chunktype)
        } else {
            Err("invalid chunk type".into())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }
    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }
    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
