pub mod keys;
use crate::keys::{KeyPair, Key};
use std::io::{self, BufRead};

fn main() {
    let mut pair = KeyPair::new();
    
    let hex: u64 = 0x61616161;
    println!("{:X}", hex);
    println!("{:X}", pair.pkey.encrypt64(hex));
    println!("{:X}", pair.skey.encrypt64(pair.pkey.encrypt64(hex)));

    pair.skey.write_to_file("rsa.pub");

    print!("> ");
    //let mut bytes_read = io::stdin().read(&mut buffer).unwrap();
    //let mut input = String::from_utf8(buffer[0..bytes_read].to_vec()).unwrap();
    for line in io::stdin().lock().lines() {
        println!("{:?}", line);
        let encrypted = pair.pkey.encrypt_str(line.unwrap());
        println!("{}", encrypted);
        break;
    }

    pair.pkey = Key::from_file("rsa.pub");

    for line in io::stdin().lock().lines() {
        println!("{:?}", line);
        let encrypted = pair.pkey.decrypt_str(line.unwrap());
        println!("{}", encrypted);
        break;
    }

}

