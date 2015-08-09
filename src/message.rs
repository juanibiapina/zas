use question::Question;
use answer::Answer;

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

