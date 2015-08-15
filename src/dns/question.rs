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

            name.push(str::from_utf8(&buffer[offset .. offset + size]).unwrap().to_string());
            offset += size;
        }

        let rrtype: u16 = (buffer[offset] as u16) << 8 | buffer[offset+1] as u16;
        offset += 2;

        let class: u16 = (buffer[offset] as u16) << 8 | buffer[offset+1] as u16;
        offset += 2;

        (Question{
            name: name,
            rrtype: rrtype,
            class: class,
        }, offset)
    }
}

