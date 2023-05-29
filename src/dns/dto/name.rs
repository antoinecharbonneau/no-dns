use core::fmt;

use super::label::Label;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name {
    labels: Vec<Label>,
}

impl Name {
    pub fn unserialize(stream: &[u8], offset: usize) -> Result<(Name, usize), ()> {
        let mut i = offset;
        let mut labels: Vec<Label> = Vec::new();
        while stream[i] > 0 {
            let sequence_info = stream[i] as usize;

            // Check if there is a reference
            if sequence_info & 0xC0 == 0xC0 {
                let referenced_address = (sequence_info & 0b00111111) << 8 | stream[i + 1] as usize;
                match Name::unserialize(stream, referenced_address) {
                    Ok((mut name, _)) => {
                        labels.append(&mut name.labels);
                        return Ok((Name { labels }, i + 2));
                    }
                    Err(()) => return Err(()),
                }
            } else {
                match Label::unserialize(stream, i) {
                    Ok((label, read_head)) => {
                        labels.push(label);
                        i = read_head;
                    }
                    Err(_) => return Err(()),
                }
            }
        }

        return Ok((Name { labels }, i + 1));
    }

    /// TODO: Add compression
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = self
            .labels
            .iter()
            .map(|l| l.serialize())
            .flatten()
            .collect::<Vec<u8>>();
        bytes.push(0);

        return bytes;
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.labels
                .iter()
                .map(|l| l.to_string())
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name {
            labels: value
                .split(".")
                .into_iter()
                .map(|l| Label::from(l))
                .collect::<Vec<Label>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_unserialize() {
        let value: Vec<u8> = vec![
            4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        let offset = value.len();
        let (name, i) = Name::unserialize(&value, 1).unwrap();
        assert_eq!(name.to_string(), "www.google.com");
        assert_eq!(i as usize, offset);

        let value: Vec<u8> = vec![
            4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0xD, 0xE, 0xA, 0xD, 0xB, 0xE, 0xE, 0xF,
        ];
        let offset = value.len() - 8;
        let (name, i) = Name::unserialize(&value, 5).unwrap();
        assert_eq!(name.to_string(), "google.com");
        assert_eq!(i as usize, offset);
    }

    #[test]
    fn test_referenced_unserialize() {
        let value: Vec<u8> = vec![
            0xc0, 0x07, 4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c',
            b'o', b'm', 0,
        ];
        let offset = 2;
        let (name, i) = Name::unserialize(&value, 0).unwrap();
        assert_eq!(name.to_string(), "google.com");
        assert_eq!(i as usize, offset);

        let value: Vec<u8> = vec![
            4, b't', b'e', b's', b't', 0xc0, 0x0c, 4, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o',
            b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        let offset = 7;
        let (name, i) = Name::unserialize(&value, 0).unwrap();
        assert_eq!(name.to_string(), "test.google.com");
        assert_eq!(i as usize, offset);
    }

    #[test]
    fn test_empty_unserialize() {
        let value: Vec<u8> = vec![0, 0, 0, 0];
        let expected_offset = 1;
        let (name, offset) = Name::unserialize(&value, 0).unwrap();
        assert_eq!(name.to_string(), "");
        assert_eq!(offset, expected_offset);
    }

    #[test]
    fn test_serialize() {
        let name = Name::from("www.google.com");
        let expected = [
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
        ];
        assert_eq!(*name.serialize(), expected);
    }
}
