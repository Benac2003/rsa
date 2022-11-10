use crate::keys::{KeyPair};
pub mod keys;


fn main() {
    let pair: KeyPair = KeyPair::new().Generate();
    let t: u64 = 855;
  //  test(t);

    let n: u32 = 121;
//    test(t);
    
    //let c: u64 = pair.pkey.encrypt64(t);
  //  println!("{}", c);


}

// fn test(n: usize) {
   // prinln!("{}", size_of(usize));
    //prinln!("{}", size_of(n));
//}
