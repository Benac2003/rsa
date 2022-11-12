use crate::keys::{KeyPair};
pub mod keys;


fn main() {
    let pair: KeyPair = KeyPair::new().generate();

    let a: u64 = 855;
    print!("{} -> ", a);

    let b: u64 = pair.pkey.encrypt64(a);
    print!("{} -> ", b);

    let c: u64 = pair.skey.encrypt64(b);
    print!("{}\n", c);

}

