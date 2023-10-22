use core::fmt;

use crate::dns::compression::LabelTree;

use super::enums::{CLASS, TYPE};
use super::name::Name;
use super::question::Question;

#[derive(Clone, PartialEq)]
// pub struct ResourceRecord {
//     /// Name field of the resource record
//     pub name: Name,
//
//     /// Type of resource record
//     pub resource_type: TYPE,
//
//     /// Class of resource record
//     pub class: CLASS,
//
//     /// Time to live (seconds)
//     pub ttl: u32,
//
//     /// Number of bytes in RDATA
//     pub rdlength: u16,
//
//     /// Resource data
//     pub rdata: Vec<u8>,
// }
pub struct ResourceRecord {
    pub name: Name,
    content: Vec<u8>,
}

impl ResourceRecord {
    pub fn unserialize(stream: &[u8], offset: u16) -> (ResourceRecord, u16) {
        let (name, content_begin) = Name::unserialize(stream, offset as usize).unwrap();
        let rdlength: usize = (stream[content_begin + 8] as usize) << 8 | stream[content_begin + 9] as usize;
        let content_end = content_begin + 10 + rdlength;

        let mut content: Vec<u8> = vec![0; 10 + rdlength];
        content.copy_from_slice(&stream[content_begin..content_end]);
        // let resource_type =
        //     TYPE::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        // i += 2;
        // let class =
        //     CLASS::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        // i += 2;
        // let ttl = (stream[i as usize] as u32) << 24
        //     | (stream[(i + 1) as usize] as u32) << 16
        //     | (stream[(i + 2) as usize] as u32) << 8
        //     | (stream[(i + 3) as usize] as u32);
        // i += 4;
        // let rdlength = (stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16;
        // i += 2;
        // let rdata = stream[i as usize..i + rdlength as usize].to_vec();
        // i += rdlength as usize;
        return (
            ResourceRecord {
                name,
                content,
            },
            content_end as u16,
        );
    }

    pub fn get_type(&self) -> TYPE {
        TYPE::from_u16((self.content[0] as u16) << 8 | self.content[1] as u16)
    }

    pub fn get_class(&self) -> CLASS {
        CLASS::from_u16((self.content[2] as u16) << 8 | self.content[3] as u16)
    }

    pub fn get_ttl(&self) -> u32 {
        (self.content[4] as u32) << 24
        | (self.content[5] as u32) << 16
        | (self.content[6] as u32) << 8
        |  self.content[7] as u32
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        self.content[4..8].copy_from_slice(&ttl.to_be_bytes());
    }

    pub fn get_rdata_length(&self) -> usize {
        (self.content[8] as usize) << 8 | self.content[9] as usize
    }

    pub fn get_rdata(&self) -> &[u8] {
        &self.content[10..10 + self.get_rdata_length()]
    }

    pub fn serialize(self, bytes: &mut Vec<u8>, lt: &mut LabelTree) {
        self.name.serialize(bytes, lt);
        bytes.extend_from_slice(self.content.as_slice());
    }

    pub fn get_question(&self) -> Question {
        let mut content: [u8; 4] = [0; 4];
        content.copy_from_slice(&self.content[0..4]);

        return Question {
            qname: self.name.clone(),
            content
        };
    }
}

impl fmt::Display for ResourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NAME: {}\nTYPE: {}\nCLASS: {}\nTTL: {}\nRDLENGTH: {}\nRDATA: {:?}\n",
            self.name, self.get_type(), self.get_class(), self.get_ttl(), self.get_rdata_length(), self.get_rdata()
        )
    }
}

impl fmt::Debug for ResourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unserialize_test() {
        let rr_bytes = [
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0x00, 0x01, 0x00, 0x01, 0, 0, 0x0E, 0x10, 0x00, 0x04, 172, 217, 13, 132,
        ];
        let expected_offset = rr_bytes.len();
        let (rr, offset) = ResourceRecord::unserialize(&rr_bytes, 0);
        assert_eq!(offset as usize, expected_offset);
        assert_eq!(rr.name.to_string(), "www.google.com");
        assert_eq!(rr.get_type().to_u16(), 1);
        assert_eq!(rr.get_class().to_u16(), 1);
        assert_eq!(rr.get_ttl(), 3600);
        assert_eq!(rr.get_rdata_length(), 4);
        assert_eq!(rr.get_rdata(), [172, 217, 13, 132]);
    }

    #[test]
    fn weird_unserialize_test() {
        let rr_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 127, 0, 0, 1];
        let expected_offset = rr_bytes.len();
        let (rr, offset) = ResourceRecord::unserialize(&rr_bytes, 1);
        assert_eq!(offset as usize, expected_offset);
        assert_eq!(rr.name.to_string(), "");
        assert_eq!(rr.get_type().to_u16(), 0);
        assert_eq!(rr.get_class().to_u16(), 0);
        assert_eq!(rr.get_ttl(), 15);
        assert_eq!(rr.get_rdata_length(), 4);
        assert_eq!(rr.get_rdata(), [127, 0, 0, 1]);
    }

    #[test]
    fn basic_serialize_test() {
        let rr_bytes = [
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0x00, 0x01, 0x00, 0x01, 0, 0, 0x0E, 0x10, 0x00, 0x04, 172, 217, 13, 132,
        ];
        let rr = ResourceRecord {
            name: Name::from("www.google.com"),
            content: vec![0x00, 0x01, 0x00, 0x01, 0, 0, 0x0E, 0x10, 0x00, 0x04, 172, 217, 13, 132]
        };
        let mut bytes: Vec<u8> = Vec::with_capacity(rr_bytes.len());
        let mut lt = LabelTree::default();
        rr.serialize(&mut bytes, &mut lt);
        assert_eq!(rr_bytes, bytes.as_slice());
    }
}
