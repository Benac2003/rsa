pub struct Key {
    pub n: u64,
    pub exp: u64,
}

impl Key {
    pub fn encrypt(&self, t: u64) -> u64 {
        let exp_table:[u64; 64] = self.gen_table(t);
        for i in 0..64 {
            println!("table[{}] = {}", i, exp_table[i]);
        }
        let mut acc: u64 = 1;
        let mut idx: u64 = self.exp;
        for i in 0..64 {
            if (idx & 1) == 1 {
                acc = (acc * exp_table[i]) % self.n;
                println!("idx: {} exp_table[{}]: {}", idx, i, exp_table[i]);
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
        KeyPair { skey: }
    }

}