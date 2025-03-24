use rand::{rngs::SmallRng, SeedableRng, Rng};

fn insert_and_lookup_naive(mut count: u64) {
    // SmallRng is Xoshiro256++. Insecure but fast
    let mut rng = SmallRng::seed_from_u64(1981 | (1986 << 8) | (2003 << 16) | (2011 << 24));
    // Create a new hashmap
    let mut hash_map = naive_hashmap::HashMap::new();

    while count != 0 {
        // Get a random key
        let key = rng.random::<u32>();
        // Arbitrarily generate an insert or o lookup
        if rng.random::<bool>() {
            let value = rng.random::<u32>();
            hash_map.insert(key, value);
        } else {
            hash_map.get(&key);
        }
        count -= 1;
    }
}

fn insert_and_lookup_standard(mut count: u64) {
    // SmallRng is Xoshiro256++. Insecure but fast
    let mut rng = SmallRng::seed_from_u64(1981 | (1986 << 8) | (2003 << 16) | (2011 << 24));
    // Create a new hashmap
    let mut hash_map = std::collections::HashMap::new();

    while count != 0 {
        // Get a random key
        let key = rng.random::<u32>();
        // Arbitrarily generate an insert or o lookup
        if rng.random::<bool>() {
            let value = rng.random::<u32>();
            hash_map.insert(key, value);
        } else {
            hash_map.get(&key);
        }
        count -= 1;
    }
}

fn main() {}
