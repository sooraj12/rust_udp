use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").unwrap();

    let data = b"Hello, world!";

    socket.send_to(data, "127.0.0.1:3000").unwrap();

    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        println!("Received {} bytes from {:?}", amt, src);
        println!("{}", std::str::from_utf8(&buf[0..amt]).unwrap());
    }
}
