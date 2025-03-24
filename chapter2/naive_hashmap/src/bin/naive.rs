use naive_hashmap::HashMap;
use rand::{Rng, SeedableRng, rngs::SmallRng};

fn main() {
    // SmallRng is Xoshiro256++. Insecure but fast
    let mut rng = SmallRng::seed_from_u64(1981 | (1986 << 8) | (2003 << 16) | (2011 << 24));
    // Create a new hashmap
    let mut hash_map = HashMap::new();
    // Keep track of the status of the actions over the HashMap
    let mut insert_empty = 0;
    let mut insert_exists = 0;
    let mut get_some = 0;
    let mut get_none = 0;

    for _ in 0..100_000 {
        // Get a random key
        let key = rng.random::<u32>();
        // Arbitrarily generate an insert or o lookup
        if rng.random::<bool>() {
            let value = rng.random::<u32>();
            if hash_map.insert(key, value).is_some() {
                insert_exists += 1;
            } else {
                insert_empty += 1;
            }
        } else if hash_map.get(&key).is_some() {
            get_some += 1;
        } else {
            get_none += 1;
        }
    }

    // Print stats
    println!("INSERT");
    println!("  exists: {}", insert_exists);
    println!("  empty: {}", insert_empty);
    println!("LOOKUP");
    println!("  some: {}", get_some);
    println!("  none: {}", get_none);
}
