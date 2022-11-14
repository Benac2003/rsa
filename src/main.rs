pub mod keys;
use crate::keys::KeyPair;
use std::io::{self, Read, Write};

const BUFF_SIZE: usize = 16;


fn main() {
    let mut buffer = [0; BUFF_SIZE];
    let bytes_read = io::stdin().read(&mut buffer).unwrap();
    io::stdout().write_all(&buffer[0..bytes_read]).unwrap();
    let encrypted = KeyPair::new().generate()
        .pkey.encrypt_str(String::from(std::str::from_utf8(&buffer[0..bytes_read]).unwrap()));
    println!("{}", encrypted);
}

