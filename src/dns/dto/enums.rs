#[derive(Clone)]
#[repr(u16)]
pub enum TYPE {
    A = 1,
    AAAA = 28,
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
            _ => result = unsafe{std::mem::transmute_copy::<TYPE, u16>(self)},
        }

        return result;
    }

    pub fn to_string(&self) -> String {
        
        let result: String;
        match self {
            TYPE::A => result = String::from("IPv4 host address"),
            TYPE::AAAA => result = String::from("IPv6 host address"),

            TYPE::NotImplemented(value) => result = format!("Not implemented: {}", value),
        }

        return result;
    }
}

#[derive(Clone)]
#[repr(u16)]
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
            _ => result = unsafe{std::mem::transmute_copy::<CLASS, u16>(self)},
        }
        return result;
    }

    pub fn to_string(&self) -> String {
        let result: String;
        match self {
            CLASS::IN => result = String::from("IN (Internet)"),
            CLASS::NotImplemented(value) => result = format!("Not implemented: {}", *value),
        }
        return result;
    }
}