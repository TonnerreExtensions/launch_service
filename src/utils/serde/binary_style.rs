use crate::utils::serde::binary_style::BinaryStyle::Value;
use crate::utils::serde::serializer::Serializable;

pub enum BinaryStyle {
    Value,
    Reference,
}

impl From<u8> for BinaryStyle {
    fn from(input: u8) -> Self {
        match input {
            0 => BinaryStyle::Value,
            1 => BinaryStyle::Reference,
            _ => panic!("Unexpected value for identifier")
        }
    }
}

impl BinaryStyle {
    pub fn convert_to_byte(self) -> u8 {
        match self {
            BinaryStyle::Value => 0,
            BinaryStyle::Reference => 1
        }
    }
}

