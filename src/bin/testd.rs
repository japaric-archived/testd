extern crate bincode;
extern crate byteorder;
extern crate tempdir;
extern crate testd;

use std::fs::{File, Permissions};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use bincode::SizeLimit;
use bincode::serde;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use tempdir::TempDir;
use testd::{Executable, ExitStatus, Output};

const PORT: u16 = 12345;

pub fn main() {
    let listener = TcpListener::bind(("127.0.0.1", PORT)).unwrap();

    println!("Listening on :{}", PORT);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let size = stream.read_u64::<LittleEndian>().unwrap();
        let mut blob = vec![0; size as usize];
        stream.read_exact(&mut blob[..]).unwrap();

        let exec: Executable = serde::deserialize(&blob).unwrap();
        println!("{:?}", exec);

        let td = TempDir::new("testd").unwrap();
        let tfile = td.path().join("test");
        File::create(&tfile).unwrap().write_all(&exec.contents).unwrap();
        fs::set_permissions(&tfile, Permissions::from_mode(0o755)).unwrap();
        let coutput = Command::new(&tfile).output().unwrap();

        let output = Output {
            stdout: coutput.stdout,
            stderr: coutput.stderr,
            status: ExitStatus {
                success: coutput.status.success(),
                code: coutput.status.code(),
            },
        };
        println!("{:?}", output.status);

        let blob = serde::serialize(&output, SizeLimit::Infinite).unwrap();
        stream.write_u64::<LittleEndian>(blob.len() as u64).unwrap();
        stream.write_all(&blob).unwrap();
    }
}
