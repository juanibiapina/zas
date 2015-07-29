use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:12043").unwrap();

    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();

    let buf = &mut buf[..amt];
    socket.send_to(buf, &src).unwrap();

    drop(socket);
}
