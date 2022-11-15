use crate::keys::{KeyPair, Key};
pub mod keys;


fn main() {
    let pair: KeyPair = KeyPair::new().generate(&32);

    let t: u64 = 121;
//    test(t);
    
    let c: u64 = pair.skey.encrypt64(t);
    println!("{}", c);
    let p = pair.pkey.encrypt64(c);
    println!("{}", p);

}

// fn test(n: usize) {
   // prinln!("{}", size_of(usize));
    //prinln!("{}", size_of(n));
//}
