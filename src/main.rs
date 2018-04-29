use std::net::UdpSocket;
use std::str;

fn main() {
    let buf = &[0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d,
                0x00, 0x00, 0x01, 0x00, 0x01];
    println!("going to bind to address");
    let socket = UdpSocket::bind("0.0.0.0:12345").expect("couldn't bind to address");
    println!("going to send data");
    socket.send_to(buf, "8.8.8.8:53").expect("couldn't send data");
    println!("going to receive");
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf).expect("Didn't receive data");
    println!("going to print");

    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("result: {}", s);
    println!("amt: {}", amt);
    println!("src: {}", src);
}
