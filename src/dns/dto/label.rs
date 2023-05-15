use core::fmt;
use std::hash::Hash;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Label {
    value: String,
}

impl Label {
    pub fn unserialize(stream: &[u8], offset: usize) -> Result<(Label, usize), u8> {
        let length = stream[offset] as usize;
        let value = stream[offset + 1..offset + 1 + length].iter().map(|b| b.clone() as char).collect::<Vec<char>>();

        if value.len() < 0 || value.len() > 63 {
            return Err(0);
        }

        if !char::is_ascii_alphanumeric(&value[0]) || !char::is_ascii_alphanumeric(&value[value.len() - 1]) {
            return Err(1);
        }

        if !value.iter().all(|c| {char::is_ascii_alphanumeric(c) || c == &'-'}) {
            return Err(2);
        }

        if value.iter().all(char::is_ascii_digit) {
            return Err(3);
        }

        Ok((Label{ value: value.into_iter().collect() }, offset + 1 + length))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![self.value.len() as u8];
        bytes.append(&mut self.value.clone().into_bytes());
        
        bytes
    }
}

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        return Label {value: value.to_string()}
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unserialize_test() {
        let bytes = [3, b'w', b'w', b'w'];
        let (label, byte_read) = Label::unserialize(&bytes, 0).unwrap();
        assert_eq!(label.to_string(), "www");
        assert_eq!(byte_read, bytes.len());


        let bytes = [0, 3, b'w', b'w', b'w'];
        let (label, byte_read) = Label::unserialize(&bytes, 1).unwrap();
        assert_eq!(label.to_string(), "www");
        assert_eq!(byte_read, bytes.len());
    }

    #[test]
    fn basic_serialize_test() {
        let bytes = [3, b'w', b'w', b'w'];
        assert_eq!(Label::from("www").serialize(), bytes);
    }
}