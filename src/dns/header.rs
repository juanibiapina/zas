pub struct Header {
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
}

impl Header {
    pub fn pack(&self, buffer: &mut [u8], offset: usize) -> usize {
        let mut offset: usize = offset;

        buffer[offset] = (self.id >> 8) as u8;
        buffer[offset + 1] = self.id as u8;
        offset += 2;

        let mut body: u16 = 0;

        body = body | self.query_response << 15;
        body = body | self.operation_code << 11;
        body = body | self.authoritative_answer << 10;
        body = body | self.truncation_flag << 9;
        body = body | self.recursion_desired << 8;
        body = body | self.recursion_available << 7;
        body = body | self.unused << 4;
        body = body | self.error_code;

        buffer[offset] = (body >> 8) as u8;
        buffer[offset + 1] = body as u8;
        offset += 2;

        buffer[offset] = (self.question_count >> 8) as u8;
        buffer[offset + 1] = self.question_count as u8;
        offset += 2;

        buffer[offset] = (self.answer_count >> 8) as u8;
        buffer[offset + 1] = self.answer_count as u8;
        offset += 2;

        buffer[offset] = (self.ns_count >> 8) as u8;
        buffer[offset + 1] = self.ns_count as u8;
        offset += 2;

        buffer[offset] = (self.ar_count >> 8) as u8;
        buffer[offset + 1] = self.ar_count as u8;
        offset += 2;

        offset
    }
}
