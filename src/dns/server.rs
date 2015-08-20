use std::thread;
use std::net::UdpSocket;

use dns::header::Header;
use dns::answer::Answer;
use dns::message::Message;

pub struct Server {
    pub thread: thread::JoinHandle<u8>,
}

impl Server {
    pub fn create() -> Server {
        let thread = Server::create_thread();

        Server{
            thread: thread,
        }
    }

    fn create_thread() -> thread::JoinHandle<u8> {
        thread::spawn(move || {
            let socket = UdpSocket::bind("127.0.0.1:12043").unwrap();

            let mut buffer: [u8; 512] = [0; 512];

            loop {
                let (size, source) = socket.recv_from(&mut buffer).unwrap();

                let query_message = Message::unpack(&buffer[..size]);

                let answer_message: Message;

                if query_message.questions[0].name.last().unwrap() == "dev" {
                    let mut answers = Vec::new();

                    answers.push(Answer{
                        name: query_message.questions[0].name.clone(),
                        rrtype: 1,
                        class: 1,
                        ttl: 0,
                        length: 4,
                        data: vec!(127, 0, 0, 1),
                    });

                    answer_message = Message {
                        header: Header {
                            query_response: 1,
                            answer_count: 1,
                            ns_count: 0,
                            ar_count: 0,
                            ..query_message.header
                        },
                        answers: answers,
                        ..query_message
                    };
                } else {
                    answer_message = Message {
                        header: Header {
                            query_response: 1,
                            answer_count: 0,
                            ns_count: 0,
                            ar_count: 0,
                            error_code: 3,
                            ..query_message.header
                        },
                        ..query_message
                    };
                }

                let size = answer_message.pack(&mut buffer);

                socket.send_to(&buffer[..size], &source).unwrap();
            }
        })
    }
}

