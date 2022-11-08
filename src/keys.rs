/* 
    Key Pair Usage
*/


pub struct Key {
    pub n: u64,
    pub exp: u64,
}

impl Key {
    pub fn encrypt(&self, t: u64) -> u64 {
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
    pub pkey: Key,
}

impl KeyPair {
    fn new () -> Self {
    let p: u64 = 61;
    let q: u64 = 53;
    let n: u64 = q * p;
    let m: u64 = (p - 1) * (q - 1);
    let e: u64 = findE(m);
    let d: u64 = (1 + n * m) / e;

    println!("{:?}", (d, e, m, n));
    }

    fn RandExponents() {

    }

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

}
