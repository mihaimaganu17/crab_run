use std::{
    hash::{Hash, Hasher, BuildHasher},
    collections::hash_map::RandomState,
};

#[derive(Default)]
pub struct HashMap<K, V, S = RandomState>
where
    K: Eq,
    V: std::fmt::Debug,
{
    // Creates `Hashers` used to generate hashes for values inserted in the hashmap
    hash_builder: S,
    // Data storage
    data: Vec<(u64, K, V)>,
}

impl<K, V> HashMap<K, V, RandomState>
where
    K: Eq + Hash,
    V: std::fmt::Debug
{
    pub fn new() -> HashMap<K, V> {
        Self {
            hash_builder: RandomState::new(),
            data: Vec::new(),
        }
    }

    pub fn make_hash<S: BuildHasher>(value: &K, hash_builder: S) -> u64 {
        // Create a new hasher
        let mut hasher = hash_builder.build_hasher();
        // Hash the given value
        value.hash(&mut hasher);
        // Finish and return the hash
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
}
