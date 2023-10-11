use core::fmt;

use crate::dns::compression::LabelTree;

use super::header::Header;
use super::question::Question;
use super::resource_record::ResourceRecord;

/// # DNS datagram
///
/// Contains a complete DNS request.
///
/// ## Structure
/// Header (12 bytes)
///
/// x Questions
///
/// y Answers (Resource record)
///
/// z Authorities (Resource record)
///
/// w Additionals (Resource record)
pub struct Datagram {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additionals: Vec<ResourceRecord>,
}

impl Datagram {
    pub fn unserialize(stream: &[u8]) -> Datagram {
        let mut offset: u16 = 0;
        let header = Header::unserialize(stream);
        offset += Header::LENGTH;
        let mut questions: Vec<Question> = Vec::new();
        for _ in 0..header.qdcount {
            let question: Question;
            (question, offset) = Question::unserialize(stream, offset);
            questions.push(question);
        }
        let mut answers: Vec<ResourceRecord> = Vec::new();
        for _ in 0..header.ancount {
            let answer: ResourceRecord;
            (answer, offset) = ResourceRecord::unserialize(stream, offset);
            answers.push(answer);
        }
        let mut authorities: Vec<ResourceRecord> = Vec::new();
        for _ in 0..header.nscount {
            let authority: ResourceRecord;
            (authority, offset) = ResourceRecord::unserialize(stream, offset);
            authorities.push(authority);
        }
        let mut additionals: Vec<ResourceRecord> = Vec::new();
        for _ in 0..header.arcount {
            let additional: ResourceRecord;
            (additional, offset) = ResourceRecord::unserialize(stream, offset);
            additionals.push(additional);
        }

        return Datagram {
            header,
            questions,
            answers,
            authorities,
            additionals,
        };
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(512);
        let mut lt = LabelTree::default();

        bytes.extend_from_slice(self.header.serialize().as_slice());

        for i in 0..self.header.qdcount {
            let question = self
                .questions
                .get(i as usize)
                .expect("Question index does not exist.");
            question.serialize(&mut bytes, &mut lt);
        }

        for i in 0..self.header.ancount {
            let answer = self
                .answers
                .get(i as usize)
                .expect("Answer index does not exist.");
            answer.serialize(&mut bytes, &mut lt);
        }

        for i in 0..self.header.nscount {
            let authority = self
                .authorities
                .get(i as usize)
                .expect("Authority index does not exist.");
            authority.serialize(&mut bytes, &mut lt);
        }

        for i in 0..self.header.arcount {
            let additional = self
                .additionals
                .get(i as usize)
                .expect("Question index does not exist.");
            additional.serialize(&mut bytes, &mut lt);
        }

        log::debug!("{:?}", lt);
        bytes
    }
}

impl fmt::Display for Datagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("");
        output.push_str(&format!("HEADER\n{}", self.header.to_string()));
        for i in 0..self.header.qdcount as usize {
            output.push_str(&format!(
                "Question {}\n{}",
                i,
                &self
                    .questions
                    .get(i)
                    .expect("Question index does not exist")
            ));
        }
        for i in 0..self.header.ancount as usize {
            output.push_str(&format!(
                "Answer {}\n{}",
                i,
                &self.answers.get(i).expect("Answer index does not exist")
            ));
        }
        for i in 0..self.header.nscount as usize {
            output.push_str(&format!(
                "Authority {}\n{}",
                i,
                &self
                    .authorities
                    .get(i)
                    .expect("Authority index does not exist")
            ));
        }
        for i in 0..self.header.arcount as usize {
            output.push_str(&format!(
                "Additional {}\n{}",
                i,
                &self
                    .additionals
                    .get(i)
                    .expect("Additional index does not exist")
            ));
        }

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_unserialize_serialize_test() {
        let datagram_bytes = [
            // header
            0x44, 0x44, 0b10000000, 0b00000000, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
            // question 1
            3, b'w', b'w', b'w', 6, b'g', b'o', b'o', b'g', b'l', b'e', 3, b'c', b'o', b'm', 0x00,
            0x00, 0x01, 0x00, 0x01, // answer 1
            0xC0, 0x0C, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x68, 0x00, 0x04, 127, 0, 0, 1,
        ];
        let datagram = Datagram::unserialize(&datagram_bytes);
        
        assert_eq!(datagram.serialize().as_slice(), datagram_bytes);
    }
}
