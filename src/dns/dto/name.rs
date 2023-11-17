use core::fmt;
use std::io::Write;
use std::hash::{Hash, Hasher};

use crate::dns::compression::{LabelTree, ReferencedLabel};
use std::collections::VecDeque;

use super::label::Label;


#[derive(Clone, PartialEq, Eq)]
pub struct Name {
    pub labels: VecDeque<Label>,
}

impl Name {
    pub fn unserialize(stream: &[u8], offset: usize) -> Result<(Name, usize), ()> {
        let mut i = offset;
        let mut labels: VecDeque<Label> = VecDeque::new();
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
                        labels.push_back(label);
                        i = read_head;
                    }
                    Err(_) => return Err(()),
                }
            }
        }
        labels.make_contiguous();

        return Ok((Name { labels }, i + 1));
    }

    pub fn serialize(mut self, bytes: &mut Vec<u8>, tree: &mut LabelTree) {
        let reference = tree.find_best_reference(&self);
        let mut new_references: Vec<ReferencedLabel> = Vec::with_capacity(self.labels.len());

        while self.labels.len() > 0 {
            let label = self.labels.pop_front().unwrap();
            if self.labels.len() >= reference.index {
                let pointer = bytes.len();
                label.serialize(bytes);
                new_references.push(ReferencedLabel::new(label, pointer as u16));
            } else {
                new_references.push(ReferencedLabel::new(label, 0));
            }
        }

        if reference.is_valid() {
            bytes.push(0xC0 | (reference.position >> 8) as u8);
            bytes.push(reference.position as u8);
        } else {
            bytes.push(0);
        }

        tree.insert(new_references);
    }

    pub fn get_string(&self) -> String {
        let mut s: Vec<u8> = Vec::with_capacity(16);
        if self.labels.len() >= 1 {
            s = self.labels[0].value.clone().into();
            if self.labels.len() > 1 {
                for label in self.labels.as_slices().0[1..].iter() {
                    s.push(b'.');
                    let _ = s.write_all(label.value.as_bytes());
                }
            }
        } 

        String::from_utf8(s).unwrap()
    }

    pub fn as_labels(&self) -> &[Label]{
        self.labels.as_slices().0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.get_string()
        )
    }
}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.labels.as_slices().0[0..self.labels.len() - 1].iter().for_each(|l| l.hash(state));
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        let mut labels = value
                .split(".")
                .into_iter()
                .map(|l| Label::from(l))
                .collect::<VecDeque<Label>>();
        labels.make_contiguous();
        Name {
            labels 
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
