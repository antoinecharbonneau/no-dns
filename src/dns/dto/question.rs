use crate::dns::compression::LabelTree;

use super::enums::{CLASS, TYPE};
use super::name::Name;
use core::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Question {
    /// # QNAME (Question name)
    ///
    /// The domain name used in the question.
    ///
    /// Length: Dynamic (see Name struct for more informations)
    pub qname: Name,
    pub content: [u8; 4],
}

impl Question {
    pub fn unserialize(stream: &[u8], offset: u16) -> (Self, u16) {
        let (qname, mut i) = Name::unserialize(stream, offset as usize).unwrap();
        let mut content: [u8; 4] = [0; 4];
        content.copy_from_slice(&stream[i..i + 4]);

        (Self {qname, content}, i as u16 + 4)
    }

    pub fn serialize(self, bytes: &mut Vec<u8>, lt: &mut LabelTree) {
        self.qname.serialize(bytes, lt);
        bytes.extend_from_slice(&self.content);
    }

    pub fn get_type(&self) -> TYPE {
        TYPE::from_u16((self.content[0] as u16) << 8 | self.content[1] as u16)
    }

    pub fn get_class(&self) -> CLASS {
        CLASS::from_u16((self.content[2] as u16) << 8 | self.content[3] as u16)
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QNAME: {}\nQTYPE: {}\nQCLASS: {}\n",
            self.qname, self.get_type(), self.get_class()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unserialize_test() {
        let question_bytes = [
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0x44, 0x22, 0x01, 0x10,
        ];
        let (question, offset) = Question::unserialize(&question_bytes, 0);
        assert_eq!(offset as usize, question_bytes.len());
        assert_eq!(question.qname.to_string(), "www.google.com");
        assert_eq!(question.get_type().to_u16(), 0x4422);
        assert_eq!(question.get_class().to_u16(), 0x0110);

        let question_bytes = [
            0, 0, 0, 0, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o',
            b'm', 0, 0x44, 0x22, 0x01, 0x10,
        ];
        let (question, offset) = Question::unserialize(&question_bytes, 4);
        assert_eq!(offset as usize, question_bytes.len());
        assert_eq!(question.qname.to_string(), "www.google.com");
        assert_eq!(question.get_type().to_u16(), 0x4422);
        assert_eq!(question.get_class().to_u16(), 0x0110);
    }
}
