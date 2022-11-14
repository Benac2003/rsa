use crate::keys::{KeyPair, Key};
pub mod keys;


fn main() {
    let pair: KeyPair = KeyPair::new().Generate();

    let a: u64 = 855;
    
    let b: u64 = pair.pkey.encrypt64(a);

    let c: u64 = pair.skey.decrypt64(b);

    print!(" {}\n", a == c);
}

