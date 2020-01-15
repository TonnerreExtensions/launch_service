use std::string::FromUtf8Error;

pub trait Deserializable {
    fn deserialize(bytes: Vec<u8>) -> Result<Self, FromUtf8Error>
        where Self: std::marker::Sized;
}

#[cfg(test)]
mod deserializer_test {
    use std::string::FromUtf8Error;

    use crate::utils::serde::deserializer::Deserializable;

    impl Deserializable for String {
        fn deserialize(mut bytes: Vec<u8>) -> Result<Self, FromUtf8Error> where Self: std::marker::Sized {
            String::from_utf8(bytes)
        }
    }

    #[test]
    fn test_deserialize() {
        let mut bytes: Vec<u8> = vec![72, 101, 108, 108, 111];
        let res = String::deserialize(bytes).unwrap();
        assert_eq!(res, "Hello")
    }
}