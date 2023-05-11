use core::fmt;

#[derive(Clone)]

/// # DNS Header
/// 
/// The header of a dns packet
/// 
/// Contains all the information about the packet.
/// 
/// Length: 12 bytes
pub struct Header {
    /// # ID
    /// 
    /// Id of the request
    /// 
    /// Length: 2 bytes
    pub id: u16,

    /// # QR (Question or Reply)
    /// 
    /// If the request is a question or a reply
    /// 
    /// Length: 1 bit
    pub qr: bool,

    /// # OPCODE (Operation code)
    /// 
    /// Operation code of the request
    /// 
    /// See OPCODE enum for more details
    /// 
    /// Length: 4 bits
    pub opcode: OPCODE,

    /// # AA (Authoritative answer)
    /// 
    /// If the response comes from an authority
    /// 
    /// Length: 1 bit
    pub aa: bool,

    /// # TC (Truncated)
    /// 
    /// Whether the message is truncated and should be retried
    /// over a TCP connection.
    /// 
    /// Length: 1 bit
    pub tc: bool,

    /// # RC (Recursion desired)
    /// 
    /// Is recursion desired
    /// 
    /// Length: 1 bit
    pub rd: bool, 

    /// # RA (Recursion available)
    /// 
    ///  Is recursion available
    /// 
    /// Length: 1 bit
    pub ra: bool,

    /// # Z (Future use)
    /// 
    /// Set to 0 by default, but followed on automatically
    /// 
    /// Length: 1 bit
    pub z: bool,

    /// # AD (Authenticated data)
    /// 
    /// Is it verified data (DNSSEC)
    /// 
    /// Length: 1 bit
    pub ad: bool,

    /// # CD (Checked data)
    /// 
    /// Is unverified data accepted (DNSSEC)
    /// 
    /// Length: 1 bit
    pub cd: bool,

    /// RCODE (Response code)
    /// 
    /// The response code from the server
    /// 
    /// Length: 4 bit
    pub rcode: RCODE,

    /// # QDCOUNT (Question count)
    /// 
    /// How many questions the packet contains
    /// 
    /// Length: 2 bytes
    pub qdcount: u16,

    /// # ANCOUNT (Answer count)
    /// 
    /// How many answer resource records the packet contains
    /// 
    /// Length: 2 bytes
    pub ancount: u16,

    /// # NSCOUNT (Authority count)
    /// 
    /// How many authority resource records the packet contains
    /// 
    /// Length: 2 bytes
    pub nscount: u16,

    /// # ARCOUNT (Additional count)
    /// 
    /// How many additional resource records the packet contains
    /// 
    /// Length: 2 bytes
    pub arcount: u16,
}

impl Header {
    pub fn unserialize(stream: &[u8]) -> Header {
        return Header {
            id:         (stream[0] as u16) << 8 | stream[1] as u16,
            qr:         stream[2] & 0x80 == 0x80,
            opcode:     OPCODE::from_u8((stream[2] & 0x78) >> 3),
            aa:         stream[2] & 0x04 == 0x04,
            tc:         stream[2] & 0x02 == 0x02,
            rd:         stream[2] & 0x01 == 0x01,
            ra:         stream[3] & 0x80 == 0x80,
            z:          stream[3] & 0x40 == 0x40,
            ad:         stream[3] & 0x20 == 0x20,
            cd:         stream[3] & 0x10 == 0x10,
            rcode:      RCODE::from_u8(stream[3] & 0x0F),
            qdcount:    (stream[4] as u16) << 8 | stream[5] as u16,
            ancount:    (stream[6] as u16) << 8 | stream[7] as u16,
            nscount:    (stream[8] as u16) << 8 | stream[9] as u16,
            arcount:    (stream[10] as u16) << 8 | stream[11] as u16,
        }
    }

    pub fn serialize(&self) -> [u8; 12] {
        let header_bytes: [u8; 12] = [
            (self.id >> 8) as u8,
            self.id as u8,
            (self.qr as u8) << 7 | self.opcode.to_u8() << 3 | (self.aa as u8) << 2 | (self.tc as u8) << 1 | self.rd as u8,
            (self.ra as u8) << 7 | (self.z as u8) << 6 | (self.ad as u8) << 5 | (self.cd as u8) << 4 | self.rcode.to_u8(),
            (self.qdcount >> 8) as u8,
            self.qdcount as u8,
            (self.ancount >> 8) as u8,
            self.ancount as u8,
            (self.nscount >> 8) as u8,
            self.nscount as u8,
            (self.arcount >> 8) as u8,
            self.arcount as u8,
        ];
        header_bytes
    }

    pub const LENGTH: u16 = 0x0C;
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "ID: {}\nQR: {}\nOPCODE: {}\nAA: {}\nTC: {}\nRD: {}\nRA: {}\nZ: {}\nAD: {}\nCD: {}\nRCODE: {}\nQDCOUNT: {}\nANCOUNT: {}\nNSCOUNT: {}\nARCOUNT: {}\n",
            self.id,
            self.qr,
            self.opcode,
            self.aa,
            self.tc,
            self.rd,
            self.ra,
            self.z,
            self.ad,
            self.cd,
            self.rcode,
            self.qdcount,
            self.ancount,
            self.nscount,
            self.arcount
        )
        // let mut output = String::from("");
        // output.push_str(&format!("ID: {}\n", self.id));
        // output.push_str(&format!("QR: {}\n", self.qr));
        // output.push_str(&format!("OPCODE: {}\n", self.opcode.to_string()));
        // output.push_str(&format!("AA: {}\n", self.aa));
        // output.push_str(&format!("TC: {}\n", self.tc));
        // output.push_str(&format!("RD: {}\n", self.rd));
        // output.push_str(&format!("RA: {}\n", self.ra));
        // output.push_str(&format!("Z: {}\n", self.z));
        // output.push_str(&format!("AD: {}\n", self.ad));
        // output.push_str(&format!("CD: {}\n", self.cd));
        // output.push_str(&format!("RCODE: {}\n", self.rcode.to_string()));
        // output.push_str(&format!("QDCOUNT: {}\n", self.qdcount));
        // output.push_str(&format!("ANCOUNT: {}\n", self.ancount));
        // output.push_str(&format!("NSCOUNT: {}\n", self.nscount));
        // output.push_str(&format!("ARCOUNT: {}\n", self.arcount));

        // output
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
            _ => result = unsafe{std::mem::transmute_copy::<OPCODE, u8>(self)},
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
    NotImplemented(u8)
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
            _ => result = RCODE::NotImplemented(value)
        }

        return result;
    }

    fn to_u8(&self) -> u8 {
        let result: u8;
        match self {
            RCODE::NotImplemented(value) => result = *value,
            _ => result = unsafe{std::mem::transmute_copy::<RCODE, u8>(self)},
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
        let bytes = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let header = Header::unserialize(&bytes);
        assert_eq!(bytes, header.serialize());

        let bytes = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
        let header = Header::unserialize(&bytes);
        assert_eq!(bytes, header.serialize());
    }
}