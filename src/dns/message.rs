use dns::header::Header;
use dns::question::Question;
use dns::answer::Answer;

pub struct Message {
    pub header: Header,

    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn unpack(buffer: &[u8]) -> Message {
        let id: u16 = (buffer[0] as u16) << 8 | buffer[1] as u16;
        let body: u16 = (buffer[2] as u16) << 8 | buffer[3] as u16;
        let question_count: u16 = (buffer[4] as u16) << 8 | buffer[5] as u16;
        let answer_count: u16 = (buffer[6] as u16) << 8 | buffer[7] as u16;
        let ns_count: u16 = (buffer[8] as u16) << 8 | buffer[9] as u16;
        let ar_count: u16 = (buffer[10] as u16) << 8 | buffer[11] as u16;

        let mut offset: usize = 12;

        let mut questions = Vec::with_capacity(question_count as usize);

        for _ in 0..question_count {
            match Question::unpack(buffer, offset) {
                (question, updated_offset) => {
                    questions.push(question);
                    offset = updated_offset;
                }
            }
        }

        Message {
            header: Header {
                id: id,
                query_response: (body & (1 << 15)) >> 15,
                operation_code: (body & (15 << 11)) >> 11,
                authoritative_answer: (body & (1 >> 10)) >> 10,
                truncation_flag: (body & (1 >> 9)) >> 9,
                recursion_desired: (body & (1 >> 8)) >> 8,
                recursion_available: (body & (1 >> 7)) >> 7,
                unused: (body & (7 << 4)) >> 4,
                error_code: (body & (15 << 0)) >> 0,
                question_count: question_count,
                answer_count: answer_count,
                ns_count: ns_count,
                ar_count: ar_count,
            },
            questions: questions,
            answers: Vec::new(), // TODO parse answers
        }
    }

    pub fn pack(&self, buffer: &mut [u8]) -> usize {
        let mut offset: usize = 0;

        buffer[offset] = (self.header.id >> 8) as u8;
        buffer[offset + 1] = self.header.id as u8;
        offset += 2;

        let mut body: u16 = 0;

        body = body | self.header.query_response << 15;
        body = body | self.header.operation_code << 11;
        body = body | self.header.authoritative_answer << 10;
        body = body | self.header.truncation_flag << 9;
        body = body | self.header.recursion_desired << 8;
        body = body | self.header.recursion_available << 7;
        body = body | self.header.unused << 4;
        body = body | self.header.error_code;

        buffer[offset] = (body >> 8) as u8;
        buffer[offset + 1] = body as u8;
        offset += 2;

        buffer[offset] = (self.header.question_count >> 8) as u8;
        buffer[offset + 1] = self.header.question_count as u8;
        offset += 2;

        buffer[offset] = (self.header.answer_count >> 8) as u8;
        buffer[offset + 1] = self.header.answer_count as u8;
        offset += 2;

        buffer[offset] = (self.header.ns_count >> 8) as u8;
        buffer[offset + 1] = self.header.ns_count as u8;
        offset += 2;

        buffer[offset] = (self.header.ar_count >> 8) as u8;
        buffer[offset + 1] = self.header.ar_count as u8;
        offset += 2;

        for question in self.questions.iter() {
            offset = question.pack(buffer, offset);
        }

        for answer in self.answers.iter() {
            offset = answer.pack(buffer, offset);
        }

        offset
    }
}
