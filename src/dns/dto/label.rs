use core::fmt;
use std::hash::Hash;

#[derive(Clone, PartialEq, Hash, Eq, Ord, PartialOrd, Default)]
pub struct Label {
    value: String,
}

impl Label {
    pub fn unserialize(stream: &[u8], offset: usize) -> Result<(Label, usize), ()> {
        let length = stream[offset] as usize;
        let value: String = stream[offset + 1..offset + 1 + length]
            .iter()
            .map(|b| b.clone() as char)
            .collect();
        let label = Label { value };
        if !label.is_valid() {
            return Err(());
        }

        Ok((label, offset + 1 + length))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![self.value.len() as u8];
        bytes.append(&mut self.value.clone().into_bytes());

        bytes
    }

    pub fn is_valid(&self) -> bool {
        let value: Vec<char> = self.value.chars().collect();
        value.len() < 64
            && char::is_ascii_alphanumeric(&value[0])
            && char::is_ascii_alphanumeric(&value[value.len() - 1])
            && value
                .iter()
                .all(|c| char::is_ascii_alphanumeric(c) || c == &'-')
            && !value.iter().all(char::is_ascii_digit)
    }
}

impl From<&str> for Label {
    /// # Label::from
    ///
    /// Warning: the value is not validated
    /// You can use Label::is_valid to validate it.
    ///
    /// If you do not validate it and use the label for requests,
    /// it is undefined behavior and the value might not be accepted
    /// by remote servers, marking the request as if it has a format error.
    fn from(value: &str) -> Self {
        return Label {
            value: value.to_string(),
        };
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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
