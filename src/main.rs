pub mod keys;
use crate::keys::{KeyPair, Key};
use std::io::{self, Write, BufRead};

fn main() {
    let mut pair = KeyPair::new();
    
    // Test number below 3233
    let mut hex: u64 = 0x00000C9F;
    print!("Num Test 1: {:X} -> ", hex);
    print!("{:X} -> ", pair.pkey.encrypt64(hex));
    print!("{:X}    PASS\n", pair.skey.encrypt64(pair.pkey.encrypt64(hex)));

    // Test number above 3233
    hex = 0x61616161;
    print!("Num Test 2: {:X} -> ", hex);
    print!("{:X} -> ", pair.pkey.encrypt64(hex));
    print!("{:X}    FAIL\n", pair.skey.encrypt64(pair.pkey.encrypt64(hex)));


    pair.skey.write_to_file("rsa.pub");

    print!("Enter text to encrypt: ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        println!("{:?}", line);
        let encrypted = pair.pkey.encrypt_str(line.unwrap());
        println!("Result: {}", encrypted);
        break;
    }

    pair.pkey = Key::from_file("rsa.pub");

    print!("Enter text to decrypt: ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        println!("{:?}", line);
        let encrypted = pair.pkey.decrypt_str(line.unwrap());
        println!("{}", encrypted);
        break;
    }

}

