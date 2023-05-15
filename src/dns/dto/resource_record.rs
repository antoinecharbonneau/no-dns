use core::fmt;

use super::name::Name;
use super::enums::{TYPE, CLASS};
use super::question::Question;

#[derive(Clone, PartialEq)]
pub struct ResourceRecord {
    /// Name field of the resource record
    pub name: Name,

    /// Type of resource record
    pub resource_type: TYPE,

    /// Class of resource record
    pub class: CLASS,

    /// Time to live (seconds)
    pub ttl: u32,

    /// Number of bytes in RDATA
    pub rdlength: u16,

    /// Resource data
    pub rdata: Vec<u8>,
}

impl ResourceRecord {

    pub fn unserialize(stream: &[u8], offset: u16) -> (ResourceRecord, u16) {
        let (name, mut i) = Name::unserialize(stream, offset as usize).unwrap();
        let resource_type = TYPE::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        i += 2;
        let class = CLASS::from_u16((stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16);
        i += 2;
        let ttl = (stream[i as usize] as u32) << 24 | (stream[(i + 1) as usize] as u32) << 16 | (stream[(i + 2) as usize] as u32) << 8 | (stream[(i + 3) as usize] as u32);
        i += 4;
        let rdlength = (stream[i as usize] as u16) << 8 | stream[(i + 1) as usize] as u16;
        i += 2;
        let rdata = stream[i as usize..i + rdlength as usize].to_vec();
        i += rdlength as usize;
        return (ResourceRecord{name, resource_type, class, ttl, rdlength, rdata}, i as u16);
    }

    pub fn serialize(&self) -> Box<[u8]> {
        let mut bytes: Vec::<u8> = Vec::new();
        bytes.extend_from_slice(&self.name.serialize());
        bytes.extend_from_slice(&[
            (self.resource_type.to_u16() >> 8) as u8,
            self.resource_type.to_u16() as u8,
            (self.class.to_u16() >> 8) as u8,
            self.class.to_u16() as u8,
            (self.ttl >> 24) as u8,
            (self.ttl >> 16) as u8,
            (self.ttl >> 8) as u8,
            self.ttl as u8,
            (self.rdlength >> 8) as u8,
            self.rdlength as u8,
        ]);
        bytes.extend_from_slice(self.rdata.as_slice());
        return bytes.into_boxed_slice();
    }

    pub fn get_question(&self) -> Question {
        return Question {
            qname: self.name.clone(),
            qtype: self.resource_type.clone(),
            qclass: self.class.clone(),
        }
    }
}

impl fmt::Display for ResourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NAME: {}\nTYPE: {}\nCLASS: {}\nTTL: {}\nRDLENGTH: {}\nRDATA: {:?}\n",
        self.name,
        self.resource_type,
        self.class,
        self.ttl,
        self.rdlength,
        self.rdata
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
        let rr_bytes = [3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0, 0x00, 0x01, 0x00, 0x01, 0, 0, 0x0E, 0x10, 0x00, 0x04, 172, 217, 13, 132];
        let expected_offset = rr_bytes.len();
        let (rr, offset) = ResourceRecord::unserialize(&rr_bytes, 0);
        assert_eq!(offset as usize, expected_offset);
        assert_eq!(rr.name.to_string(), "www.google.com");
        assert_eq!(rr.resource_type.to_u16(), 1);
        assert_eq!(rr.class.to_u16(), 1);
        assert_eq!(rr.ttl, 3600);
        assert_eq!(rr.rdlength, 4);
        assert_eq!(rr.rdata, [172, 217, 13, 132]);
    }

    #[test]
    fn weird_unserialize_test() {
        let rr_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 127, 0, 0, 1];
        let expected_offset = rr_bytes.len();
        let (rr, offset) = ResourceRecord::unserialize(&rr_bytes, 1);
        assert_eq!(offset as usize, expected_offset);
        assert_eq!(rr.name.to_string(), "");
        assert_eq!(rr.resource_type.to_u16(), 0);
        assert_eq!(rr.class.to_u16(), 0);
        assert_eq!(rr.ttl, 15);
        assert_eq!(rr.rdlength, 4);
        assert_eq!(rr.rdata, [127, 0, 0, 1]);
    }
    

    #[test]
    fn basic_serialize_test() {
        let rr_bytes = [3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0, 0x00, 0x01, 0x00, 0x01, 0, 0, 0x0E, 0x10, 0x00, 0x04, 172, 217, 13, 132];
        let rr = ResourceRecord{
            name: Name::from("www.google.com"),
            resource_type: TYPE::from_u16(1),
            class: CLASS::from_u16(1),
            ttl: 3600,
            rdlength: 4,
            rdata: vec![172, 217, 13, 132]
        };
        let serialized_rr = rr.serialize();
        assert_eq!(rr_bytes, *serialized_rr);
    }
}