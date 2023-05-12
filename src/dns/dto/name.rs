use core::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub value: String
}

impl Name {
    pub fn unserialize(stream: &[u8], offset: u16) -> (Name, u16) {
        let mut i: u16 = 0;
        let mut name = String::from("");
        while stream[(offset + i) as usize] > 0 {
            let sequence_length: u16 = stream[(offset + i) as usize] as u16;
            
            // Check if there is a reference
            if sequence_length & 0xC0 == 0xC0 {
                let referenced_address = ((sequence_length as u16) & 0b00111111) << 8 | stream[(offset + i + 1) as usize] as u16;
                let (referenced_value, _) = Name::unserialize(stream, referenced_address);
                name.push_str(&referenced_value.value);
                return (Name{value: name}, offset + i + 2);
            }

            for j in 1..=sequence_length as u16 {
                name.push(stream[(offset + i + j) as usize] as char);
            }
            name.push('.');
            i += sequence_length + 1;
        }

        if name.len() > 1 {
            name.truncate(name.len() - 1);
        }
        
        return (Name{value: name}, offset + i + 1);
    }

    /// TODO: Add compression
    pub fn serialize(&self) -> Box<[u8]> {
        let mut bytes: Vec::<u8> = Vec::new();
        let elements = self.value.split(".");
        for element in elements {
            bytes.push(element.len() as u8);
            for c in element.chars() {
                bytes.push(c as u8);
            }
        }
        bytes.push(0);
        return bytes.into_boxed_slice();
    }

}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_unserialize() {
        let value: Vec<u8> = vec![4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0];
        let offset = value.len();
        let (name, i) = Name::unserialize(&value, 1);
        assert_eq!(name.value, "www.google.com");
        assert_eq!(i as usize, offset);

        let value: Vec<u8> = vec![4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0, 0xD, 0xE, 0xA, 0xD, 0xB, 0xE, 0xE, 0xF];
        let offset = value.len() - 8;
        let (name, i) = Name::unserialize(&value, 5);
        assert_eq!(name.value, "google.com");
        assert_eq!(i as usize, offset);
    }

    #[test]
    fn test_referenced_unserialize() {
        let value: Vec<u8> = vec![0xc0, 0x07, 4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0];
        let offset = 2;
        let (name, i) = Name::unserialize(&value, 0);
        assert_eq!(name.value, "google.com");
        assert_eq!(i as usize, offset);

        let value: Vec<u8> = vec![4, b't', b'e', b's', b't', 0xc0, 0x0c, 4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0];
        let offset = 7;
        let (name, i) = Name::unserialize(&value, 0);
        assert_eq!(name.value, "test.google.com");
        assert_eq!(i as usize, offset);
    }

    #[test]
    fn test_empty_unserialize() {
        let value: Vec<u8> = vec![0, 0, 0, 0];
        let expected_offset = 1;
        let (name, offset) = Name::unserialize(&value, 0);
        assert_eq!(name.value, "");
        assert_eq!(offset, expected_offset);
    }

    #[test]
    fn test_serialize() {
        let input = "www.google.com";
        let name = Name{value: input.to_string()};
        let expected = [3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0];
        assert_eq!(*name.serialize(), expected);
    }
}