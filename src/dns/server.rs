use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::net::UdpSocket;
use std::str::FromStr;
use std::thread;

use crate::config::Config;
use crate::dns::answer::Answer;
use crate::dns::header::Header;
use crate::dns::message::Message;

pub struct Server {
    pub thread: thread::JoinHandle<u8>,
}

impl Server {
    pub fn create(config: &Config) -> Server {
        let thread = Server::create_thread(config.dns_port);

        Server { thread }
    }

    fn create_thread(port: u16) -> thread::JoinHandle<u8> {
        thread::spawn(move || {
            let socket = UdpSocket::bind(SocketAddrV4::new(
                Ipv4Addr::from_str("127.0.0.1").unwrap(),
                port,
            ))
            .unwrap();

            let mut buffer: [u8; 512] = [0; 512];

            loop {
                let (size, source) = socket.recv_from(&mut buffer).unwrap();

                let query_message = Message::unpack(&buffer[..size]);

                let mut answers = Vec::new();

                answers.push(Answer {
                    name: query_message.questions[0].name.clone(),
                    rrtype: 1,
                    class: 1,
                    ttl: 0,
                    length: 4,
                    data: vec![127, 0, 0, 1],
                });

                let answer_message = Message {
                    header: Header {
                        query_response: 1,
                        answer_count: 1,
                        ns_count: 0,
                        ar_count: 0,
                        ..query_message.header
                    },
                    answers,
                    ..query_message
                };

                let size = answer_message.pack(&mut buffer);

                socket.send_to(&buffer[..size], &source).unwrap();
            }
        })
    }
}
