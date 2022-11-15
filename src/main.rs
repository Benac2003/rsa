use crate::keys::{KeyPair, Key};
pub mod keys;


fn main() {
<<<<<<< Updated upstream
    let pair: KeyPair = KeyPair::new().Generate();
    let t: u64 = 123;
  //  test(t);
=======
    let pair: KeyPair = KeyPair::new().generate(&32);
>>>>>>> Stashed changes

    let n: u32 = 121;
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
