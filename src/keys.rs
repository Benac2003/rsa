/* 
    Key Pair Usage
*/


extern crate base64;
use base64::{encode};
use rand::Rng;

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

        encode(&bytes)
    }

    pub fn decrypt64(&self, t: u64) -> u64 {
        self.encrypt64(t)
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

    pub fn generate(&self) -> Self {
        let (d, e, _m, n) = self.calc_exponents();
        KeyPair {skey: Key { n, exp: e}, pkey: Key {n, exp: d}}
    }

    fn calc_exponents(&self) -> (u64, u64, u64, u64) {
        //print!("Calculating Exponents of KeyPair...");
        let (p, q) = (self.rand_prime(), self.rand_prime());
        let n: u64 = q * p;
        let m: u64 = (q -1) * (p -1);
        let e: u64 = self.findE(m);
        let d = self.ext_gcd(e, m);
        println!("\nCalculated Exponents:\nd: {}\ne: {}\nm: {}\nn: {}", d, e, m, n);
        (d, e, m, n)
    }

    fn ext_gcd(&self, a: u64, b: u64) -> u64 {
        let (mut a, mut b) = (i64::try_from(a).unwrap(), i64::try_from(b).unwrap());
        let mut x: [i64; 2] = [0, 1];
        let mut y: [i64; 2] = [1, 0];
        let mut q: i64;
        let  old_b: i64 = b;    

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

    fn rand_prime(&self) -> u64 {
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
    
    fn is_prime(&self, n: u64) -> bool {
        if n == 2 || n == 3 {
            return true;
        }
        if n <= 1 || n % 2 == 0 || n % 3 == 0 {
            return false;  
        }
        let mut i = 5;
        while  i*i <=n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i = i+6;
        }
        return true;
    }

    fn gcd(&self, first: u64, second: u64) -> u64 {
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
        

    fn find_e(&self, m: u64) -> u64 {
        // let e: Vec<u64> = range(2, m);
        for n in 2..m {
            if self.gcd(n, m) == 1 {
                return n;
            }
        }
        panic!("Can't Find E!!");
    }
}
