/* 
    Key Pair Usage
*/


<<<<<<< Updated upstream
use core::panic;
use std::collections::btree_map::Range;

use rand::Rng;
=======
extern crate base64;

use base64::{encode};
use rand::{Rng, thread_rng, distributions::Uniform, seq::SliceRandom};

>>>>>>> Stashed changes

pub struct Key {
    pub n: u64,
    pub exp: u64,
}

impl Key {
    
    pub fn encrypt_str(&self, s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut result: Vec<u64> = Vec::new();
    
        let mut temp: u64;
        let mut buff: u64 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            //println!("{:#X}", *byte);
            temp = (*byte as u64) << ((3-(i%4))*8);
            //println!("{:#X}", temp);
            buff = temp | buff;
            //println!("buff: {:#X}", buff);
            if (3-(i%4)) == 0 {
                result.push(self.encrypt64(buff));
                buff = 0;
            }
        }

        let mut temp8: u8;
        for (i, word) in result.iter_mut().enumerate() {
            for n in 0..3 {
                temp8 = ((*word & (0xFF << ((3-n)*8))) >> ((3-n)*8)) as u8;
                println!("{:X}", temp8);
                bytes[i*4 + n] = temp8;
            }
        }

        println!("{:X?}", bytes);
        
        String::from_utf8(bytes).ok().expect("Failed to convert to UTF8.")
    }

    pub fn encrypt64(&self, t: u64) -> u64 {
        let exp_table:[u64; 64] = self.gen_table(t);
        let mut acc: u64 = 1;
        let mut idx: u64 = self.exp;
        for i in 0..64 {
            if (idx & 1) == 1 {
                acc = (acc * exp_table[i]) % self.n;
            }
            idx = idx >> 1;
        }

        acc
    }

    fn gen_table(&self, t: u64) -> [u64; 64] {
        let mut table: [u64; 64] = [0; 64];
        table[0] = t % self.n;
        for i in 1..64 {
            table[i] = (table[i-1].pow(2)) % self.n;
        }
        table
    }

}


pub struct KeyPair {
    pub skey: Key,
    pub pkey: Key
}


impl KeyPair {

    pub fn new () -> Self {
        KeyPair {skey: Key { n: 3233, exp: 17}, pkey: Key {n: 3233, exp: 2753}}

    }

<<<<<<< Updated upstream
    pub fn Generate(&self) -> Self {
        let (d, e, m, n) = self.calcExponents();
        KeyPair {skey: Key { n, exp: e}, pkey: Key {n, exp: d}}
    }

    fn calcExponents(&self) -> (u64, u64, u64, u64) {
        print!("Calculating Exponents of KeyPair...\n");
        let (p, q) = (self.RandPrime(), self.RandPrime());
        let n: u64 = q * p;
        let m: u64 = (q -1) * (p -1);
        let e: u64 = self.findE(m);
        let d = self.ExtGcd(e, m);
=======
    pub fn generate(&self, k: &u32) -> Self {
        let (d, e, _m, n) = self.calc_exponents(k);
        KeyPair {skey: Key { n, exp: e}, pkey: Key {n, exp: d}}
    }

    fn calc_exponents(&self, k: &u32) -> (u64, u64, u64, u64) {
        //print!("Calculating Exponents of KeyPair...");
        let e: u64 = self.rand_e();
        let (p, q) = self.rand_pq(*k, e);
        let n: u64 = q * p;
        let m: u64 = (p -1) * (q -1);
        let d = self.ext_gcd(e, m);
>>>>>>> Stashed changes
        println!("\nCalculated Exponents:\nd: {}\ne: {}\nm: {}\nn: {}", d, e, m, n);
        (d, e, m, n)
    }

<<<<<<< Updated upstream
    fn ExtGcd(&self, a: u64, b: u64) -> u64 {
        let (mut a, mut b) = (i64::try_from(a).unwrap(), i64::try_from(b).unwrap());
        let mut x: [i64; 2] = [0, 1];
        let mut y: [i64; 2] = [1, 0];
        let mut q: i64;
        let  old_b: i64 = b;    
=======
    fn ext_gcd(&self, a: u64, b: u64) -> u64 {
        let (mut a, mut b) = (i128::try_from(a).unwrap(), i128::try_from(b).unwrap());
        let mut x: [i128; 2] = [0, 1];
        let mut y: [i128; 2] = [1, 0];
        let mut q: i128;
        let  old_b: i128 = b;    
>>>>>>> Stashed changes

        while a != 0 {
            ((q, a), b) = ((b / a, b % a), a);
            (y[0], y[1]) = (y[1], y[0] - q * y[1]);
            (x[0], x[1]) = (x[1], x[0] - q * x[1]);    
        }
        if b != 1 {
            panic!("gcd(a, b) != 1");
        }
        if x[0] < 0 {
            x[0] = x[0] + old_b;
        }
        u64::try_from(x[0]).unwrap()
    }

<<<<<<< Updated upstream
    fn RandPrime(&self) -> u64 {
        let mut u:u64 = 0;
        let mut n: u64 = 0;

        while !self.isPrime(u) {
            u = rand::thread_rng().gen_range(1000..3000);
        }
        
        for i in (2..1000).step_by(2) {
            n = i* u + 1;
            if self.isPrime(n) {
                return n;
            }
        }
        panic!("RandPrime: Iteration has reached limit!!")
    }
    
    fn isPrime(&self, n:u64) -> bool {
        if n == 2 || n == 3 {
            return true;
=======

    fn rand_pq(&self, mut k: u32, e: u64) -> (u64, u64) {
        k = k/2;
        let max: u64 = u64::try_from(2_u128.pow(k) - 1).unwrap();
        let min: u64 = ((max >> 1)^max)>>2;
        let mut p: [u64;2] = [0,0];
    
        for n in 0..2{
            p[n] = rand::thread_rng().gen_range(min..max);
            loop {
                println!("{}", p[n]);
                if !self.is_prime(p[n]) {
                    if p[n] & 1 == 0 {
                        p[n]+=1;
                        continue;
                    }
                    p[n]+=2;
                    continue;
                }
                if p[n] % e != 1 {
                    break;
                }
                p[n]+=2;
            }  
        } 
        (p[0], p[1])
    }
    
    fn is_prime(&self, mut u: u64) -> bool {
        let mut n: i64 = u as i64;
        let mut u: usize = u as usize;
        let mut s: i64;
        let mut d: i64;
        let mut x: i64;
        let mut y: i64 = 1;
        let mut r = 0;
        let mut samp: Vec<i64>;
        let mut k: i64 = 9999;
        let mut rng = rand::thread_rng();
        println!("n: {}", n);
        if n < 6 {
            print!("n < 6");
            return [false, false, true, true, false, true] [u];
>>>>>>> Stashed changes
        }
        else if n & 1 == 0 {
            print!("even");
            return false
        }

        (s, d) = (0, n - 1);
        while d & 1 == 0 {
            (s, d) = (s + 1, d >> 1);
        }
        samp = (2..std::cmp::min(n - 2, i64::MAX as i64)).collect();
        samp.shuffle(&mut thread_rng());
        // ..&samp[0..std::cmp::min(n-4, k) as usize
        
        for i in 0..k {
            
            let a = thread_rng().gen_range(2..n-2);
            println!("\na:{} d: {}", a, d);
            x = self.mod_pow(a, d, n);
            if x == 1 && x + 1 == n{
                continue;
            }
            for r in 1..s {
                x  = x * x % n;
                println!("\nx: {}", x);
                if x != 1 {
                    println!("x != 1");
                    return false; 
                }
                else if x != n-1 {
                    println!("x != -1");
                    break;
                }
            }
            if  r==s {
                print!("loop end");
                return false;
            }   
        }
        print!("true");
        print!("n: {n}");
        return true;
    }

<<<<<<< Updated upstream
    fn Gcd(&self, first: u64, second: u64) -> u64 {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
=======
    fn mod_pow(&self, mut a: i64, mut n: i64,  r: i64) -> i64 {
        let mut res: i64;
        let mut res2: i64;

        if r == 1 {
            return 0;
>>>>>>> Stashed changes
        }

        res = a % n;

        for i in 64..0 {
            if res < (0 << 63) {
                res <<=1;
                if res >=1 {res -=n};
            } else {
                res <<= 1;
                if res >= n {res -= n};
                res2 = res + r;
                if res2 < res {res2 -= n};

                res = res2;
            }
        }
        res
    }
<<<<<<< Updated upstream
        
    fn findE(&self, m: u64) -> u64 {
        let mut e:u64;
        for n in 2..m {
            e = rand::thread_rng().gen_range(2..m);
            if self.Gcd(e, m) == 1 {
                return e;
            }
=======

    fn rand_e(&self) -> u64 {
        let e: u64;
        let p: [u64; 5] = [3, 5, 17, 257, 65537];
 
        for _n in 0..4 {
            e = p[rand::thread_rng().gen_range(0..4)];
            return e;
>>>>>>> Stashed changes
        }
        panic!("Can't Find E!!");
    }
}
