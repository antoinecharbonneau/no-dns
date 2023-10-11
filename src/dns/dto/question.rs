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

    /// # QTYPE (Question Type)
    ///
    /// The type of the resource prompted
    ///
    /// See TYPE enum for more details
    ///
    /// Length: 2 bytes
    pub qtype: TYPE,

    /// # QCLASS (Question Class)
    ///
    /// The class of the resource prompted
    ///
    /// See CLASS enum for more details
    ///
    /// Length: 2 bytes
    pub qclass: CLASS,
}

impl Question {
    pub fn unserialize(stream: &[u8], offset: u16) -> (Question, u16) {
        let (qname, mut i) = Name::unserialize(stream, offset as usize).unwrap();
        let qtype =
            TYPE::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        i += 2;
        let qclass =
            CLASS::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        i += 2;
        return (
            Question {
                qname,
                qtype,
                qclass,
            },
            i as u16,
        );
    }

    pub fn serialize(&self, bytes: &mut Vec<u8>, lt: &mut LabelTree) {
        self.qname.serialize(bytes, lt);
        bytes.extend_from_slice(&[
            (self.qtype.to_u16() >> 8) as u8,
            self.qtype.to_u16() as u8,
            (self.qclass.to_u16() >> 8) as u8,
            self.qclass.to_u16() as u8,
        ]);
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QNAME: {}\nQTYPE: {}\nQCLASS: {}\n",
            self.qname, self.qtype, self.qclass
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
        assert_eq!(question.qtype.to_u16(), 0x4422);
        assert_eq!(question.qclass.to_u16(), 0x0110);

        let question_bytes = [
            0, 0, 0, 0, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o',
            b'm', 0, 0x44, 0x22, 0x01, 0x10,
        ];
        let (question, offset) = Question::unserialize(&question_bytes, 4);
        assert_eq!(offset as usize, question_bytes.len());
        assert_eq!(question.qname.to_string(), "www.google.com");
        assert_eq!(question.qtype.to_u16(), 0x4422);
        assert_eq!(question.qclass.to_u16(), 0x0110);
    }

    #[test]
    fn basic_serialize_test() {
        let question_bytes = [
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0x44, 0x44, 0x01, 0x01,
        ];
        let question = Question {
            qname: Name::from("www.google.com"),
            qtype: TYPE::from_u16(0x4444),
            qclass: CLASS::from_u16(0x0101),
        };
        let mut bytes: Vec<u8> = Vec::with_capacity(question_bytes.len());
        let mut lt = LabelTree::default();
        question.serialize(&mut bytes, &mut lt);
        assert_eq!(bytes.as_slice(), question_bytes);
    }
}
