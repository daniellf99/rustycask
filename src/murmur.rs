use std::ops::BitAnd;

pub fn murmurhash3_x64_128(key: &[u8], seed: &u32) -> u128 {
    let mut h1 = *seed as u64;
    let mut h2 = *seed as u64;

    let c1: u64 = 0x87c37b91114253d5;
    let c2: u64 = 0x4cf5ad432745937f;

    // -------
    // Body

    // Block size = 128 bits
    // We subdivide into 2 blocks of 64 bits
    let key_length = key.len();
    let number_of_blocks = key_length / 16;

    for i in 0..number_of_blocks {
        let k1 = &key[i * 8..(i + 1) * 8];
        let k2 = &key[(i + 1) * 8..(i + 2) * 8];
        
        let k1: [u8; 8] = k1.try_into().unwrap();
        let k2: [u8; 8] = k2.try_into().unwrap();
        
        let k1 = u64::from_le_bytes(k1).wrapping_mul(c1);
        let k1 = k1.rotate_left(31);
        let k1 = k1.wrapping_mul(c2);
        h1 = h1 ^ k1;
        h1 = h1.rotate_left(27);
        h1 = h1 + h2;
        h1 = h1.wrapping_mul(5).wrapping_add(0x52dce729);
        
        let k2 = u64::from_le_bytes(k2).wrapping_mul(c2);
        let k2 = k2.rotate_left(33);
        let k2 = k2.wrapping_mul(c1);
        h2 = h2 ^ k2;
        h2 = h2.rotate_left(31);
        h2 = h2 + h1;
        h2 = h2.wrapping_mul(5).wrapping_add(0x38495ab5);
    }
    
    // -------
    // Tail

    let tail_start_pos = number_of_blocks * 16;

    let mut k1: u64 = 0;
    let mut k2: u64 = 0;

    let tail_size = key_length.bitand(0xF);

    if tail_size >= 15 {
        k2 ^= (key[tail_start_pos + 14] as u64).wrapping_shl(48);
    }
    if tail_size >= 14 {
        k2 ^= (key[tail_start_pos + 13] as u64).wrapping_shl(40);
    }
    if tail_size >= 13 {
        k2 ^= (key[tail_start_pos + 12] as u64).wrapping_shl(32);
    }
    if tail_size >= 12 {
        k2 ^= (key[tail_start_pos + 11] as u64).wrapping_shl(24);
    }
    if tail_size >= 11 {
        k2 ^= (key[tail_start_pos + 10] as u64).wrapping_shl(16);
    }
    if tail_size >= 10 {
        k2 ^= (key[tail_start_pos + 9] as u64).wrapping_shl(8);
    }
    if tail_size >= 9 {
        k2 ^= (key[tail_start_pos + 8] as u64).wrapping_shl(0);
        
        k2 = k2.wrapping_mul(c2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
    }
    
    if tail_size >= 8 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(56);
    }
    if tail_size >= 7 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(48);
    }
    if tail_size >= 6 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(40);
    }
    if tail_size >= 5 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(32);
    }
    if tail_size >= 4 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(24);
    }
    if tail_size >= 3 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(16);
    }
    if tail_size >= 2 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(8);
    }
    if tail_size >= 1 {
        k1 ^= (key[tail_start_pos + 7] as u64).wrapping_shl(0);

        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }
    
    // ------
    // Finalization
    
    h1 ^= key_length as u64;
    h2 ^= key_length as u64;
    
    h1 += h2;
    h2 += h1;
    
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    // Concat two u64 into u128
    ((h1 as u128) << 64) | (h2 as u128)
}

fn fmix64(k: u64) -> u64 {
    let mut result = k;
    
    result ^= result >> 33;
    result = result.wrapping_mul(0xff51afd7ed558ccd);
    result ^= result >> 33;
    result = result.wrapping_mul(0xc4ceb9fe1a85ec53);
    result >> 33
}