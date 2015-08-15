pub struct Answer {
    pub name: Vec<String>,
    pub rrtype: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl Answer {
    pub fn pack(&self, buffer: &mut [u8], offset: usize) -> usize {
        let mut offset: usize = offset;

        for part in self.name.iter() {
            buffer[offset] = part.len() as u8;
            offset += 1;

            for byte in part.to_owned().into_bytes().iter() {
                buffer[offset] = *byte;
                offset += 1;
            }
        }

        buffer[offset] = 0 as u8;
        offset += 1;

        buffer[offset] = (self.rrtype >> 8) as u8;
        buffer[offset + 1] = self.rrtype as u8;
        offset += 2;

        buffer[offset] = (self.class >> 8) as u8;
        buffer[offset + 1] = self.class as u8;
        offset += 2;

        buffer[offset] = ((self.ttl & (256 << 24)) >> 24) as u8;
        buffer[offset + 1] = ((self.ttl & (256 << 16)) >> 16) as u8;
        buffer[offset + 2] = ((self.ttl & (256 << 8)) >> 8) as u8;
        buffer[offset + 3] = ((self.ttl & (256 << 0)) >> 0) as u8;
        offset += 4;

        buffer[offset] = (self.length >> 8) as u8;
        buffer[offset + 1] = self.length as u8;
        offset += 2;

        for byte in &self.data {
            buffer[offset] = *byte;
            offset += 1;
        }

        offset
    }
}
