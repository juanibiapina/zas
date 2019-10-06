pub struct Header {
    pub id: u16,                   // 2 bytes
    pub query_response: u16,       // 1 bit
    pub operation_code: u16,       // 4 bits
    pub authoritative_answer: u16, // 1 bit
    pub truncation_flag: u16,      // 1 bit
    pub recursion_desired: u16,    // 1 bit
    pub recursion_available: u16,  // 1 bit
    pub unused: u16,               // 3 bits
    pub error_code: u16,           // 4 bits
    pub question_count: u16,       // 2 bytes
    pub answer_count: u16,         // 2 bytes
    pub ns_count: u16,             // 2 bytes
    pub ar_count: u16,             // 2 bytes
}

impl Header {
    pub fn unpack(buffer: &[u8], offset: usize) -> (Header, usize) {
        let id: u16 = u16::from(buffer[offset]) << 8 | u16::from(buffer[offset + 1]);
        let body: u16 = u16::from(buffer[offset + 2]) << 8 | u16::from(buffer[offset + 3]);
        let question_count: u16 =
            u16::from(buffer[offset + 4]) << 8 | u16::from(buffer[offset + 5]);
        let answer_count: u16 = u16::from(buffer[offset + 6]) << 8 | u16::from(buffer[offset + 7]);
        let ns_count: u16 = u16::from(buffer[offset + 8]) << 8 | u16::from(buffer[offset + 9]);
        let ar_count: u16 = u16::from(buffer[offset + 10]) << 8 | u16::from(buffer[offset + 11]);

        (
            Header {
                id,
                query_response: (body & (1 << 15)) >> 15,
                operation_code: (body & (15 << 11)) >> 11,
                authoritative_answer: (body & (1 >> 10)) >> 10,
                truncation_flag: (body & (1 >> 9)) >> 9,
                recursion_desired: (body & (1 >> 8)) >> 8,
                recursion_available: (body & (1 >> 7)) >> 7,
                unused: (body & (7 << 4)) >> 4,
                error_code: (body & 15),
                question_count,
                answer_count,
                ns_count,
                ar_count,
            },
            offset + 12,
        )
    }

    pub fn pack(&self, buffer: &mut [u8], offset: usize) -> usize {
        let mut offset: usize = offset;

        buffer[offset] = (self.id >> 8) as u8;
        buffer[offset + 1] = self.id as u8;
        offset += 2;

        let mut body: u16 = 0;

        body |= self.query_response << 15;
        body |= self.operation_code << 11;
        body |= self.authoritative_answer << 10;
        body |= self.truncation_flag << 9;
        body |= self.recursion_desired << 8;
        body |= self.recursion_available << 7;
        body |= self.unused << 4;
        body |= self.error_code;

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
