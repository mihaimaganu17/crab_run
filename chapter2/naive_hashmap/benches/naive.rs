use criterion::{Criterion, criterion_group, criterion_main};
use rand::{Rng, SeedableRng, rngs::SmallRng};

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

macro_rules! insert_lookup {
    // Creates a function that compares the `naive` and `standard` `HashMap` implementations using
    // a criterion group and given parameters
    ($bench_fn:ident, $input:expr) => {
        fn $bench_fn(c: &mut Criterion) {
            // Create a new criterion group
            let mut group = c.benchmark_group(format!("HashMap/{}", $input));
            // Add the naive function
            group.bench_with_input("naive", $input, |b, count| {
                b.iter(|| insert_and_lookup_naive(*count))
            });
            // Add the standard function
            group.bench_with_input("std", $input, |b, count| {
                b.iter(|| insert_and_lookup_standard(*count))
            });
            // Consume the benchmakr group and generate the reports.
            group.finish();
        }
    };
}

insert_lookup!(insert_lookup_100000, &100_000);
insert_lookup!(insert_lookup_10000, &10_000);
insert_lookup!(insert_lookup_1000, &1000);
insert_lookup!(insert_lookup_100, &100);
insert_lookup!(insert_lookup_10, &10);
insert_lookup!(insert_lookup_1, &1);

criterion_group! {
    name = insert_lookup_benches;
    config = Criterion::default();
    targets = insert_lookup_100000, insert_lookup_10000, insert_lookup_1000, insert_lookup_100,
                insert_lookup_10, insert_lookup_1,
}

criterion_main!(insert_lookup_benches);
