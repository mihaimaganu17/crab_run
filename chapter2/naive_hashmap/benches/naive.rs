use rand::SmallRng;

fn insert_and_lookup_naive(mut count: u64) {
    // SmallRng is Xoshiro256++. Insecure but fast
    let mut rng = SmallRng::from_seed([1981, 1986, 2003, 2011]);
    // Create a new hashmap
    let mut hashmap = naive_hashmap::HashMap::new();

    while count != 0 {
        // Get a random key
        let key = rng.gen::<u32>();
        // Arbitrarily generate an insert or o lookup
        if rng.get::<bool>() {
            let value = rng.get::<u32>();
            hash_map.insert(key, value);
        } else {
            hash_map.get(key);
        }
        count -= 1;
    }
}

fn insert_and_lookup_standard(mut count: u64) {
    // SmallRng is Xoshiro256++. Insecure but fast
    let mut rng = SmallRng::from_seed([1981, 1986, 2003, 2011]);
    // Create a new hashmap
    let mut hashmap = std::collections::HashMap::new();

    while count != 0 {
        // Get a random key
        let key = rng.gen::<u32>();
        // Arbitrarily generate an insert or o lookup
        if rng.get::<bool>() {
            let value = rng.get::<u32>();
            hash_map.insert(key, value);
        } else {
            hash_map.get(key);
        }
        count -= 1;
    }
}
