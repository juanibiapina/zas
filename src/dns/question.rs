use std::str;

pub struct Question {
    pub name: Vec<String>,
    pub rrtype: u16,
    pub class: u16,
}

impl Question {
    pub fn unpack(buffer: &[u8], offset: usize) -> (Question, usize) {
        let mut name = Vec::new();
        let mut offset = offset;

        loop {
            let size: usize = buffer[offset] as usize;
            offset += 1;

            if size == 0 {
                break;
            }

            name.push(
                str::from_utf8(&buffer[offset..offset + size])
                    .unwrap()
                    .to_string(),
            );
            offset += size;
        }

        let rrtype: u16 = u16::from(buffer[offset]) << 8 | u16::from(buffer[offset + 1]);
        offset += 2;

        let class: u16 = u16::from(buffer[offset]) << 8 | u16::from(buffer[offset + 1]);
        offset += 2;

        (
            Question {
                name,
                rrtype,
                class,
            },
            offset,
        )
    }

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

        offset
    }
}
