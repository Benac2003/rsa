/*
Generate key-pair for RSA
*/

use crate::keys::KeyPair;
use crate::keys::Key;

pub struct 

fn Gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn findE(m: u64) -> u64 {
    // let e: Vec<u64> = range(2, m);
    for n in 2..m {
        if Gcd(n, m) == 1 {
            return n;
        }
    }
    return 1;
}

pub fn Generate() -> KeyPair {
    let p: u64 = 61;
    let q: u64 = 53;
    let n: u64 = q * p;
    let m: u64 = (p - 1) * (q - 1);
    let e: u64 = findE(m);
    let d: u64 = (1 + n * m) / e;

    println!("{:?}", (d, e, m, n));
  //  e: u64 = factor::PrimeFactors::value();
   // d: u64 = (1 + n * m)/e;
//    println!("d:{d}, n: {n}, e: {e}");

    KeyPair {
        skey: Key { n, exp: d },
        pkey: Key { n, exp: e }
    }

}

