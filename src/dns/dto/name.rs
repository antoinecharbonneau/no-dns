use core::fmt;
use std::io::Write;

use crate::dns::compression::{LabelTree, ReferencedLabel};

use super::label::Label;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub labels: Vec<Label>,
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

    pub fn serialize(&self, bytes: &mut Vec<u8>, tree: &mut LabelTree) {
        let reference = tree.find_best_reference(self);
        let mut references: Vec<ReferencedLabel> = Vec::with_capacity(self.labels.len());
        for i in 0..(self.labels.len() - reference.index) {
            references.push(ReferencedLabel::new(self.labels[i].clone(), bytes.len() as u16));
            self.labels[i].serialize(bytes);
        }
        if reference.is_valid() {
            bytes.push(0xC0 | (reference.position >> 8) as u8);
            bytes.push(reference.position as u8);
        } else {
            bytes.push(0);
        }
        self.labels[(self.labels.len() - reference.index)..]
            .iter()
            .for_each(|l| references.push(ReferencedLabel::new(l.clone(), 0)));
        tree.insert(references);
    }

    pub fn get_string(&self) -> String {
        let mut s: Vec<u8> = Vec::with_capacity(16);
        if self.labels.len() >= 1 {
            s = self.labels[0].value.clone().into();
            if self.labels.len() > 1 {
                for label in self.labels[1..].iter() {
                    s.push(b'.');
                    let _ = s.write_all(label.value.as_bytes());
                }
            }
        } 

        String::from_utf8(s).unwrap()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: Vec<u8> = Vec::with_capacity(16);
        if self.labels.len() >= 1 {
            s = self.labels[0].value.clone().into();
            if self.labels.len() > 1 {
                for label in self.labels[1..].iter() {
                    s.push(b'.');
                    let _ = s.write_all(label.value.as_bytes());
                }
            }
        } 
        write!(
            f,
            "{}",
            String::from_utf8(s).unwrap(),
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
        let mut bytes: Vec<u8> = Vec::with_capacity(expected.len());
        let mut lt: LabelTree = LabelTree::default();
        name.serialize(&mut bytes, &mut lt);
        assert_eq!(bytes, expected);
    }
}
