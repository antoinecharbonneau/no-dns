use core::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(u16)]
/// # TYPE
///
/// The type of the record.
///
/// ## More info
///
/// The functioning of types is specified in RFC 1035
/// and extended in many RFCs.
///
/// https://en.wikipedia.org/wiki/List_of_DNS_record_types
pub enum TYPE {
    /// # A type
    ///
    /// IPv4 IP request
    A = 1,

    /// # AAAA type
    ///
    /// IPv6 IP request
    AAAA = 28,

    /// # Not yet implemented
    ///
    /// Used to keep compatibility with unimplemented types.
    ///
    /// ## Behavior
    ///
    /// Default behavior is to forward the request to an upstream
    /// server which hopefully will know how to handle the given request
    NotImplemented(u16),
}

impl TYPE {
    pub fn from_u16(value: u16) -> TYPE {
        let result: TYPE;
        match value {
            1 => result = TYPE::A,
            28 => result = TYPE::AAAA,

            _ => result = TYPE::NotImplemented(value),
        }

        return result;
    }

    pub fn to_u16(&self) -> u16 {
        let result: u16;
        match self {
            TYPE::NotImplemented(value) => result = *value,
            // Should actually be safe since the unsafe aspect
            // Occurs if unexpected values come in, but such unexpected
            // values are covered in the NotImplemented case.
            _ => result = unsafe { std::mem::transmute_copy::<TYPE, u16>(self) },
        }

        return result;
    }
}

impl fmt::Display for TYPE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String;
        match self {
            TYPE::A => result = String::from("IPv4 host address"),
            TYPE::AAAA => result = String::from("IPv6 host address"),

            TYPE::NotImplemented(value) => result = format!("Not implemented: {}", value),
        }
        write!(f, "{result}")
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(u16)]
/// # CLASS
///
/// The class of the record
///
/// ## More info
///
/// Almost always IN (Internet), but 2 more values currently exist:
/// HS (Hesiod) and CH (Chaos).
///
/// Not really used, only IN class is implemented, others are forwarded
/// to upstream.
pub enum CLASS {
    IN = 1,
    NotImplemented(u16),
}

impl CLASS {
    pub fn from_u16(value: u16) -> CLASS {
        let result: CLASS;
        match value {
            1 => result = CLASS::IN,
            _ => result = CLASS::NotImplemented(value),
        }
        return result;
    }

    pub fn to_u16(&self) -> u16 {
        let result: u16;
        match self {
            CLASS::NotImplemented(value) => result = *value,
            _ => result = unsafe { std::mem::transmute_copy::<CLASS, u16>(self) },
        }
        return result;
    }
}

impl fmt::Display for CLASS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String;
        match self {
            CLASS::IN => result = String::from("IN (Internet)"),
            CLASS::NotImplemented(value) => result = format!("Not implemented: {}", *value),
        }
        write!(f, "{result}")
    }
}
