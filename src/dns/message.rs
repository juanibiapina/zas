use dns::question::Question;
use dns::answer::Answer;

pub struct Message {
    pub id: u16, // 2 bytes
    pub query_response: u16, // 1 bit
    pub operation_code: u16, // 4 bits
    pub authoritative_answer: u16, // 1 bit
    pub truncation_flag: u16, // 1 bit
    pub recursion_desired: u16, // 1 bit
    pub recursion_available: u16, // 1 bit
    pub unused: u16, // 3 bits
    pub error_code: u16, // 4 bits
    pub question_count: u16, // 2 bytes
    pub answer_count: u16, // 2 bytes
    pub ns_count: u16, // 2 bytes
    pub ar_count: u16, // 2 bytes

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
            questions: questions,
            answers: Vec::new(), // TODO parse answers
        }
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.push((self.id >> 8) as u8);
        buffer.push(self.id as u8);

        let mut body: u16 = 0;

        body = body | self.query_response << 15;
        body = body | self.operation_code << 11;
        body = body | self.authoritative_answer << 10;
        body = body | self.truncation_flag << 9;
        body = body | self.recursion_desired << 8;
        body = body | self.recursion_available << 7;
        body = body | self.unused << 4;
        body = body | self.error_code;

        buffer.push((body >> 8) as u8);
        buffer.push(body as u8);

        buffer.push((self.question_count >> 8) as u8);
        buffer.push(self.question_count as u8);

        buffer.push((self.answer_count >> 8) as u8);
        buffer.push(self.answer_count as u8);

        buffer.push((self.ns_count >> 8) as u8);
        buffer.push(self.ns_count as u8);

        buffer.push((self.ar_count >> 8) as u8);
        buffer.push(self.ar_count as u8);

        for question in &self.questions {
            for part in &question.name {
                buffer.push(part.len() as u8);
                let bytes = part.to_owned().into_bytes();
                for byte in &bytes {
                    buffer.push(*byte);
                }
            }

            buffer.push(0 as u8);

            buffer.push((question.rrtype >> 8) as u8);
            buffer.push(question.rrtype as u8);

            buffer.push((question.class >> 8) as u8);
            buffer.push(question.class as u8);
        }

        for answer in &self.answers {
            for part in &answer.name {
                buffer.push(part.len() as u8);
                let bytes = part.to_owned().into_bytes();
                for byte in &bytes {
                    buffer.push(*byte);
                }
            }

            buffer.push(0 as u8);

            buffer.push((answer.rrtype >> 8) as u8);
            buffer.push(answer.rrtype as u8);

            buffer.push((answer.class >> 8) as u8);
            buffer.push(answer.class as u8);

            buffer.push(((answer.ttl & (256 << 24)) >> 24) as u8);
            buffer.push(((answer.ttl & (256 << 16)) >> 16) as u8);
            buffer.push(((answer.ttl & (256 << 8)) >> 8) as u8);
            buffer.push(((answer.ttl & (256 << 0)) >> 0) as u8);

            buffer.push((answer.length >> 8) as u8);
            buffer.push(answer.length as u8);

            for byte in &answer.data {
                buffer.push(*byte);
            }
        }

        buffer
    }
}
