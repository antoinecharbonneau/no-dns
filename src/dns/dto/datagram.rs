use core::fmt;

use crate::dns::compression::LabelTree;

use super::header::Header;
use super::question::Question;
use super::resource_record::ResourceRecord;

#[derive(Clone)]
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
        for _ in 0..header.question_count() {
            let question: Question;
            (question, offset) = Question::unserialize(stream, offset);
            questions.push(question);
        }
        let mut answers: Vec<ResourceRecord> = Vec::new();
        for _ in 0..header.answer_count() {
            let answer: ResourceRecord;
            (answer, offset) = ResourceRecord::unserialize(stream, offset);
            answers.push(answer);
        }
        let mut authorities: Vec<ResourceRecord> = Vec::new();
        for _ in 0..header.authority_count() {
            let authority: ResourceRecord;
            (authority, offset) = ResourceRecord::unserialize(stream, offset);
            authorities.push(authority);
        }
        let mut additionals: Vec<ResourceRecord> = Vec::new();
        for _ in 0..header.additional_count() {
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

    pub fn serialize(mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(512);
        let mut lt = LabelTree::default();
        
        let question_count = self.header.question_count() as usize;
        let answer_count = self.header.answer_count() as usize;
        let authority_count = self.header.authority_count() as usize;
        let additional_count = self.header.additional_count() as usize;

        bytes.extend_from_slice(self.header.serialize().as_slice());

        for _i in 0..question_count {
            let question = self
                .questions
                .pop()
                .expect("Question index does not exist.");
            question.serialize(&mut bytes, &mut lt);
        }

        for _i in 0..answer_count {
            let answer = self
                .answers
                .pop()
                .expect("Answer index does not exist.");
            answer.serialize(&mut bytes, &mut lt);
        }

        for _i in 0..authority_count {
            let authority = self
                .authorities
                .pop()
                .expect("Authority index does not exist.");
            authority.serialize(&mut bytes, &mut lt);
        }

        for _i in 0..additional_count {
            let additional = self
                .additionals
                .pop()
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
        for i in 0..self.header.question_count() as usize {
            output.push_str(&format!(
                "Question {}\n{}",
                i,
                &self
                    .questions
                    .get(i)
                    .expect("Question index does not exist")
            ));
        }
        for i in 0..self.header.answer_count() as usize {
            output.push_str(&format!(
                "Answer {}\n{}",
                i,
                &self.answers.get(i).expect("Answer index does not exist")
            ));
        }
        for i in 0..self.header.authority_count() as usize {
            output.push_str(&format!(
                "Authority {}\n{}",
                i,
                &self
                    .authorities
                    .get(i)
                    .expect("Authority index does not exist")
            ));
        }
        for i in 0..self.header.additional_count() as usize {
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
