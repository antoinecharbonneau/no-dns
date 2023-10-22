use core::fmt;
use rand::Rng;

#[derive(Clone)]

pub struct Header {
    content: [u8; 12],
}

impl Header {
    pub fn get_id(&self) -> u16 {
        (self.content[0] as u16) << 8 | self.content[1] as u16
    }

    pub fn set_id(&mut self, id: u16) {
        self.content[0..2].copy_from_slice(&id.to_be_bytes()); 
    }

    pub fn is_question(&self) -> bool {
        self.content[2] & 0x80 == 0x00
    }
    
    pub fn set_question(&mut self, is_question: bool) {
        self.content[2] &= 0x7F;
        self.content[2] |= (!is_question as u8) << 7;
    }

    pub fn get_opcode(&self) -> OPCODE {
        OPCODE::from_u8((self.content[2] & 0x78) >> 3)
    }

    pub fn set_opcode(&mut self, opcode: OPCODE) {
        self.content[2] &= 0x87;
        self.content[2] |= opcode.to_u8() << 3;
    }

    pub fn is_authoritative_answer(&self) -> bool {
        self.content[2] & 0x04 == 0x04
    }

    pub fn set_authoritative_answer(&mut self, is_authoritative_answer: bool) {
        self.content[2] &= 0xFB;
        self.content[2] |= (is_authoritative_answer as u8) << 2;
    }
    
    pub fn is_truncated(&self) -> bool {
        self.content[2] & 0x02 == 0x02
    }

    pub fn set_truncated(&mut self, is_truncated: bool) {
        self.content[2] &= 0xFD;
        self.content[2] |= (is_truncated as u8) << 1;
    }

    pub fn is_recursion_desired(&self) -> bool {
        self.content[2] & 0x01 == 0x01
    }

    pub fn set_recursion_desired(&mut self, is_recursion_desired: bool) {
        self.content[2] &= 0xFE;
        self.content[2] |= is_recursion_desired as u8;
    }

    pub fn is_recursion_available(&self) -> bool {
        self.content[3] & 0x80 == 0x80
    }

    pub fn set_recursion_available(&mut self, is_recursion_available: bool) {
        self.content[3] &= 0x7F;
        self.content[3] |= (is_recursion_available as u8) << 7;
    }

    pub fn is_authenticated_data(&self) -> bool {
        self.content[3] & 0x20 == 0x20
    }

    pub fn set_authenticated_data(&mut self, is_authenticated_data: bool) {
        self.content[3] &= 0xBF;
        self.content[3] |= (is_authenticated_data as u8) << 6;
    }

    pub fn is_checked_data(&self) -> bool {
        self.content[3] & 0x10 == 0x10
    }

    pub fn set_checked_data(&mut self, is_checked_data: bool) {
        self.content[3] &= 0xEF;
        self.content[3] |= (is_checked_data as u8) << 5;
    }

    pub fn get_rcode(&self) -> RCODE {
        RCODE::from_u8(self.content[3] & 0x0f)
    }
    
    pub fn set_rcode(&mut self, rcode: RCODE) {
        self.content[3] &= 0xF0;
        self.content[3] |= rcode.to_u8();
    }

    pub fn question_count(&self) -> u16 {
        (self.content[4] as u16) << 8 | self.content[5] as u16
    }

    pub fn set_question_count(&mut self, count: u16) {
        self.content[4..6].copy_from_slice(&count.to_be_bytes());
    }

    pub fn answer_count(&self) -> u16 {
        (self.content[6] as u16) << 8 | self.content[7] as u16
    }

    pub fn set_answer_count(&mut self, count: u16) {
        self.content[6..8].copy_from_slice(&count.to_be_bytes());
    }

    pub fn authority_count(&self) -> u16 {
        (self.content[8] as u16) << 8 | self.content[9] as u16
    }

    pub fn set_authority_count(&mut self, count: u16) {
        self.content[8..10].copy_from_slice(&count.to_be_bytes());
    }

    pub fn additional_count(&self) -> u16 {
        (self.content[10] as u16) << 8 | self.content[11] as u16
    }

    pub fn set_additional_count(&mut self, count: u16) {
        self.content[10..12].copy_from_slice(&count.to_be_bytes());
    }

    fn get_z(&self) -> bool {
        self.content[2] & 0x40 == 0x40
    }

    pub fn unserialize(stream: &[u8]) -> Self {
        let mut content: [u8; 12] = [0; 12];
        content.copy_from_slice(&stream[0..12]);
        Self {
            content
        }
    }

    pub fn serialize(self) -> [u8; 12] {
        self.content
    }

    #[inline]
    fn generate_id() -> u16 {
        rand::thread_rng().gen()
    }

    /// Generate a question header with sane default values
    /// id: random
    /// qr: question
    pub fn new_question() -> Self {
        let mut header = Self { 
            content: [
                0,      0,
                0x01,   0,
                0,      1,
                0,      0,
                0,      0,
                0,      0,
            ],
        };
        header.set_id(Self::generate_id());

        header
    }

    pub fn new_reply() -> Self {
        let mut header = Self { 
            content: [
                0,      0,
                0x81,   0,
                0,      1,
                0,      1,
                0,      0,
                0,      0,
            ],
        };
        header.set_id(Self::generate_id());

        header
    }

    pub const LENGTH: u16 = 0x0C;
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "ID: {}\nQR: {}\nOPCODE: {}\nAA: {}\nTC: {}\nRD: {}\nRA: {}\nZ: {}\nAD: {}\nCD: {}\nRCODE: {}\nQDCOUNT: {}\nANCOUNT: {}\nNSCOUNT: {}\nARCOUNT: {}\n",
            self.get_id(),
            self.is_question(),
            self.get_opcode(),
            self.is_authoritative_answer(),
            self.is_truncated(),
            self.is_recursion_desired(),
            self.is_recursion_available(),
            self.get_z(),
            self.is_authenticated_data(),
            self.is_checked_data(),
            self.get_rcode(),
            self.question_count(),
            self.answer_count(),
            self.authority_count(),
            self.additional_count()
        )
    }
}

#[derive(Clone)]
#[repr(u8)]
pub enum OPCODE {
    /// Normal query
    QUERY = 0,

    /// Inverse query
    /// i.e. From an ip, get a domain
    IQUERY = 1,

    /// Status
    STATUS = 3,

    /// Notify
    NOTIFY = 4,

    /// Update
    UPDATE = 5,

    /// DNS Stateful Operation
    DSO = 6,

    /// Not implemented
    /// Value 7-15
    NotImplemented(u8),
}

impl OPCODE {
    pub fn from_u8(value: u8) -> OPCODE {
        let result: OPCODE;
        match value {
            0 => result = OPCODE::QUERY,
            1 => result = OPCODE::IQUERY,
            2 => result = OPCODE::STATUS,
            4 => result = OPCODE::NOTIFY,
            5 => result = OPCODE::UPDATE,
            6 => result = OPCODE::DSO,
            _ => result = OPCODE::NotImplemented(value),
        }

        return result;
    }

    pub fn to_u8(&self) -> u8 {
        let result: u8;
        match self {
            OPCODE::NotImplemented(value) => result = *value,
            _ => result = unsafe { std::mem::transmute_copy::<OPCODE, u8>(self) },
        };

        return result;
    }
}

impl fmt::Display for OPCODE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String;
        match self {
            OPCODE::QUERY => result = String::from("Query"),
            OPCODE::IQUERY => result = String::from("Inverse query"),
            OPCODE::STATUS => result = String::from("Status"),
            OPCODE::NOTIFY => result = String::from("Notify"),
            OPCODE::UPDATE => result = String::from("Update"),
            OPCODE::DSO => result = String::from("DNS Stateful Operations (DSO)"),
            OPCODE::NotImplemented(value) => result = format!("Not implemented: {}", *value),
        };

        write!(f, "{result}")
    }
}

#[derive(Clone)]
#[repr(u8)]
pub enum RCODE {
    /// No error during execution
    NoError = 0,
    /// Format error in request
    FormError = 1,
    /// Server failure
    ServFail = 2,
    /// Non-existent domain
    NXDomain = 3,
    /// Not implemented on server
    NotImp = 4,
    /// Refused by server
    Refused = 5,
    /// Value could exist, but isn't implemented
    NotImplemented(u8),
}

impl RCODE {
    fn from_u8(value: u8) -> RCODE {
        let result: RCODE;
        match value {
            0 => result = RCODE::NoError,
            1 => result = RCODE::FormError,
            2 => result = RCODE::ServFail,
            3 => result = RCODE::NXDomain,
            4 => result = RCODE::NotImp,
            5 => result = RCODE::Refused,
            _ => result = RCODE::NotImplemented(value),
        }

        return result;
    }

    fn to_u8(&self) -> u8 {
        let result: u8;
        match self {
            RCODE::NotImplemented(value) => result = *value,
            _ => result = unsafe { std::mem::transmute_copy::<RCODE, u8>(self) },
        };

        return result;
    }
}

impl fmt::Display for RCODE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String;
        match self {
            RCODE::NoError => result = String::from("No error"),
            RCODE::FormError => result = String::from("Format error"),
            RCODE::ServFail => result = String::from("Server failure"),
            RCODE::NXDomain => result = String::from("Non-existent domain"),
            RCODE::NotImp => result = String::from("Not implemented"),
            RCODE::Refused => result = String::from("Refused"),
            RCODE::NotImplemented(value) => result = format!("Value not implemented: {}", *value),
        };

        write!(f, "{result}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_unserialize_test() {
        let bytes = [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let header = Header::unserialize(&bytes);
        assert_eq!(bytes, header.serialize());

        let bytes = [
            0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        ];
        let header = Header::unserialize(&bytes);
        assert_eq!(bytes, header.serialize());
    }
}
