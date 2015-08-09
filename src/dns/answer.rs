pub struct Answer {
    pub name: Vec<String>,
    pub rrtype: u16,
    pub class: u16,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

