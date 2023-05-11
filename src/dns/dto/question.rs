use super::name::Name;
use super::enums::{TYPE, CLASS};

#[derive(Clone)]
pub struct Question {
    /// Name used in the question.
    pub qname: Name,

    /// Question type
    pub qtype: TYPE,

    /// Question class
    pub qclass: CLASS,
}

impl Question {
    pub fn unserialize(stream: &[u8], offset: u16) -> (Question, u16) {
        let (qname, mut i) = Name::unserialize(stream, offset);
        let qtype = TYPE::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        i += 2;
        let qclass = CLASS::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        i += 2;
        return (Question{qname, qtype, qclass}, i);
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut bytes: Vec::<u8> = Vec::new();
        bytes.extend_from_slice(&self.qname.serialize());
        bytes.extend_from_slice(&[
            (self.qtype.to_u16() >> 8) as u8,
            self.qtype.to_u16() as u8,
            (self.qclass.to_u16() >> 8) as u8,
            self.qclass.to_u16() as u8,            
        ]);

        return bytes.into_boxed_slice();
    }

    pub fn to_string(&self) -> String {
        return format!("QNAME: {}\nQTYPE: {}\nQCLASS: {}\n", self.qname.to_string(), self.qtype.to_string(), self.qclass.to_string())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unserialize_test() {
        let question_bytes = [3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0, 0x44, 0x22, 0x01, 0x10];
        let (question, offset) = Question::unserialize(&question_bytes, 0);
        assert_eq!(offset as usize, question_bytes.len());
        assert_eq!(question.qname.value, "www.google.com");
        assert_eq!(question.qtype.to_u16(), 0x4422);
        assert_eq!(question.qclass.to_u16(), 0x0110);


        let question_bytes = [0, 0, 0, 0, 3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0, 0x44, 0x22, 0x01, 0x10];
        let (question, offset) = Question::unserialize(&question_bytes, 4);
        assert_eq!(offset as usize, question_bytes.len());
        assert_eq!(question.qname.value, "www.google.com");
        assert_eq!(question.qtype.to_u16(), 0x4422);
        assert_eq!(question.qclass.to_u16(), 0x0110);
    }

    #[test]
    fn basic_serialize_test() {
        let question_bytes = [3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0, 0x44, 0x44, 0x01, 0x01];
        let question = Question{qname: Name{value: String::from("www.google.com")}, qtype: TYPE::from_u16(0x4444), qclass: CLASS::from_u16(0x0101)};
        let serialized_question = question.serialize();
        assert_eq!(*serialized_question, question_bytes);
    }
}