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
}

pub fn make_hash<V: Hash, S: BuildHasher>(value: &V, hash_builder: &S) -> u64 {
    // Create a new hasher
    let mut hasher = hash_builder.build_hasher();
    // Hash the given value
    value.hash(&mut hasher);
    // Finish and return the hash
    hasher.finish()
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    V: std::fmt::Debug,
    S: BuildHasher,
{
    pub fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        Self {
            hash_builder,
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Compute the hash for the entry
        let hash = make_hash(&key, &self.hash_builder);
        // Get total count of elements in the current map
        let count = self.data.len();

        // We are inserting elements in order of their hashes value, ascending, so for each element
        for idx in 0..count {
            use std::cmp::Ordering;
            // Compare the new hash with the current elements hash
            match self.data[idx].0.cmp(&hash) {
                // If the current element's hash is greater in value, the new hash takes its place
                // moving all the elements from here (including current) to the right by one
                Ordering::Greater => {
                    self.data.insert(idx, (hash, key, value));
                    return None;
                }
                // If the current element's hash is less in value, we move forward
                Ordering::Less => continue,
                // If the current element's hash has the same value, we got the same key and we
                // swap the elements
                Ordering::Equal => {
                    let element = std::mem::replace(&mut self.data[idx], (hash, key, value));
                    // Return the previous value at this position
                    return Some(element.2);
                }
            }
        }

        // If we reached this point, we have gone through all the map's data and could not find
        // a spot to insert the element, so we push it at the end
        self.data.push((hash,key,value));
        None
    }
}

#[cfg(test)]
mod tests {
}
