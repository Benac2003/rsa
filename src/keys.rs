/* 
    Key Pair Usage
*/

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
        KeyPair {skey: Key { n: 0, exp: 0}, pkey: Key {n: 0, exp: 0}}

    }

    pub fn generate(&self) -> Self {
        let (d, e, _m, n) = self.calc_exponents();
        KeyPair {skey: Key { n, exp: e}, pkey: Key {n, exp: d}}
    }

    fn calc_exponents(&self) -> (u64, u64, u64, u64) {
        print!("Calculating Exponents of KeyPair...");
        let (p, q) = self.rand_init_exponents();
        let n: u64 = q * p;
        let m: u64 = (p -1) * (q -1);
        let e: u64 = self.find_e(m);
        let d: u64 = (1 + n * m) / e;
        println!("\nCalculated Exponents:\nd: {}\ne: {}\nm: {}\nn: {}", d, e, m, n);
        (d, e, m, n)
    }

    fn rand_init_exponents(&self) -> (u64, u64) {
        let mut exps: [u64; 2] = [0 , 0];
        let mut rng = rand::thread_rng();
        let mut n:u64;
        let mut i = 0;
        

        while i < 2 {
            n = rng.gen_range(1..9999);
            if self.is_prime(n) {
                exps[i] = n;
                i+=1;
            }
        }        
        (exps[0] , exps[1])
    }

    fn is_prime(&self, n:u64) -> bool {
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
        return 1;
    }
}
