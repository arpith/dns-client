extern crate itertools;
use itertools::join;
use std::net::UdpSocket;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_name: String = "example.com".to_string();
    let split_name = args.last().unwrap_or(&default_name).split(".");
    let mut buf = vec![0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for part in split_name {
        buf.push(part.len() as u8);
        buf.extend_from_slice(part.as_bytes());
    }
    buf.append(&mut vec![0x00, 0x00, 0x01, 0x00, 0x01]);
    let socket = UdpSocket::bind("0.0.0.0:12345").expect("couldn't bind to address");
    socket.send_to(&buf, "8.8.8.8:53").expect("couldn't send data");
    let mut buf = [0; 64];
    let (amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data");
    println!("{}", join(&buf[amt-4 .. amt], "."));
}
