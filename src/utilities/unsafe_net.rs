use std::{net::TcpStream, io::{Read, Write}};

pub const MAGIC_BYTE_PLACE: u8 = 1;
pub const MAGIC_BYTE_SHOOT: u8 = 2;
pub const MAGIC_BYTE_GET_SHOT: u8 = 2;

pub fn send(stream: &mut TcpStream, bytes: &[u8]) {
    stream.write(bytes).unwrap();
}

pub fn read_blocking(stream: &mut TcpStream, buf: &mut [u8]) {
    loop {
        let count = stream.read(buf).unwrap();
        if count == 0 {
            continue;
        } else {
            break;
        }
    }
}
