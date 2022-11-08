mod keyGen;
use crate::keys::KeyPair;
pub mod keys;


fn main() {
    let pair: KeyPair = keyGen::Generate();

    let t: u64 = 855;
    let c: u64 = pair.pkey.encrypt(t);
    println!("{}", c);


}
