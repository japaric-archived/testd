extern crate bincode;
extern crate byteorder;
extern crate testd;

use std::{env, io, process};
use std::io::{Read, Write};
use std::net::TcpStream;

use bincode::SizeLimit;
use bincode::serde;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use testd::{Executable, Output};

pub fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();

    let exec = Executable::open(env::args_os().nth(1).unwrap());

    let blob = serde::serialize(&exec, SizeLimit::Infinite).unwrap();
    stream.write_u64::<LittleEndian>(blob.len() as u64).unwrap();
    stream.write_all(&blob).unwrap();

    let size = stream.read_u64::<LittleEndian>().unwrap();
    let mut blob = vec![0u8; size as usize];
    stream.read_exact(&mut blob[..]).unwrap();

    let output: Output = serde::deserialize(&blob).unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    if !output.status.success {
        process::exit(output.status.code.unwrap_or(1))
    }
}
