use crate::dns::answer::Answer;
use crate::dns::header::Header;
use crate::dns::question::Question;

pub struct Message {
    pub header: Header,

    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn unpack(buffer: &[u8]) -> Message {
        let offset: usize = 0;

        let (header, mut offset) = Header::unpack(buffer, offset);

        let mut questions = Vec::with_capacity(header.question_count as usize);

        for _ in 0..header.question_count {
            match Question::unpack(buffer, offset) {
                (question, updated_offset) => {
                    questions.push(question);
                    offset = updated_offset;
                }
            }
        }

        Message {
            header,
            questions,
            answers: Vec::new(), // TODO parse answers
        }
    }

    pub fn pack(&self, buffer: &mut [u8]) -> usize {
        let mut offset: usize = 0;

        offset = self.header.pack(buffer, offset);

        for question in self.questions.iter() {
            offset = question.pack(buffer, offset);
        }

        for answer in self.answers.iter() {
            offset = answer.pack(buffer, offset);
        }

        offset
    }
}
