use crate::murmur::murmurhash3_x64_128;

const HASH_TABLE_SIZE: usize = 256;

pub struct Database {
    hash_table: [Option<i64>; HASH_TABLE_SIZE],
}


impl Database {
    pub fn new() -> Database {
        Self {
            hash_table: [None; HASH_TABLE_SIZE],
        }
    }

    pub fn add(&mut self, key: i64, value: i64) {
        let key_hash = murmurhash3_x64_128(&key.to_le_bytes(), &0);
        let index = (HASH_TABLE_SIZE - 1) & key_hash as usize;
        self.hash_table[index] = Some(value);
    }

    pub fn get(&self, key: i64) -> Option<i64> {
        let key_hash = murmurhash3_x64_128(&key.to_le_bytes(), &0);
        let index = (HASH_TABLE_SIZE - 1) & key_hash as usize;
        
        self.hash_table[index]
    }

    pub fn print_all(&self) {
        for i in 0..HASH_TABLE_SIZE {
            println!("{}: {:?}", i, self.hash_table[i])
        }
    }
}