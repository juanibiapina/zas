use std::net::UdpSocket;
use std::str;

struct Question {
    name: Vec<String>,
    rrtype: u16,
    class: u16,
}

struct Message {
    id: u16, // 2 bytes
    query_response: u16, // 1 bit
    operation_code: u16, // 4 bits
    authoritative_answer: u16, // 1 bit
    truncation_flag: u16, // 1 bit
    recursion_desired: u16, // 1 bit
    recursion_available: u16, // 1 bit
    unused: u16, // 3 bits
    error_code: u16, // 4 bits
    question_count: u16, // 2 bytes
    answer_count: u16, // 2 bytes
    ns_count: u16, // 2 bytes
    ar_count: u16, // 2 bytes

    questions: Vec<Question>,
}

fn unpack(buffer: &[u8]) -> Message {
    let id: u16 = (buffer[0] as u16) << 8 | buffer[1] as u16;
    let body: u16 = (buffer[2] as u16) << 8 | buffer[3] as u16;
    let question_count: u16 = (buffer[4] as u16) << 8 | buffer[5] as u16;
    let answer_count: u16 = (buffer[6] as u16) << 8 | buffer[7] as u16;
    let ns_count: u16 = (buffer[8] as u16) << 8 | buffer[9] as u16;
    let ar_count: u16 = (buffer[10] as u16) << 8 | buffer[11] as u16;

    let mut offset: usize = 12;

    let mut questions = Vec::with_capacity(question_count as usize);

    for _ in 0..question_count {
        let mut name = Vec::new();

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

        questions.push(Question{
            name: name,
            rrtype: rrtype,
            class: class,
        })
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
    }
}

fn pack(message: &Message) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    buffer.push((message.id >> 8) as u8);
    buffer.push(message.id as u8);

    let mut body: u16 = 0;

    body = body | message.query_response << 15;
    body = body | message.operation_code << 11;
    body = body | message.authoritative_answer << 10;
    body = body | message.truncation_flag << 9;
    body = body | message.recursion_desired << 8;
    body = body | message.recursion_available << 7;
    body = body | message.unused << 4;
    body = body | message.error_code;

    buffer.push((body >> 8) as u8);
    buffer.push(body as u8);

    buffer.push((message.question_count >> 8) as u8);
    buffer.push(message.question_count as u8);

    buffer.push((message.answer_count >> 8) as u8);
    buffer.push(message.answer_count as u8);

    buffer.push((message.ns_count >> 8) as u8);
    buffer.push(message.ns_count as u8);

    buffer.push((message.ar_count >> 8) as u8);
    buffer.push(message.ar_count as u8);

    for question in &message.questions {
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

    buffer
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:12043").unwrap();

    let mut buffer: [u8; 512] = [0; 512];
    let (size, source) = socket.recv_from(&mut buffer).unwrap();

    let query_message = unpack(&buffer[..size]);

    let answer_message = Message { query_response: 0, ..query_message };

    let result = pack(&answer_message);

    socket.send_to(&result, &source).unwrap();

    drop(socket);
}
