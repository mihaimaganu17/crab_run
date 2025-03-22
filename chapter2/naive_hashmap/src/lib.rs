use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
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
    V: std::fmt::Debug,
{
    pub fn new() -> HashMap<K, V> {
        Self {
            hash_builder: RandomState::new(),
            data: Vec::new(),
        }
    }
}

pub fn make_hash<V: Hash, S: BuildHasher>(value: &V, hash_builder: &S) -> u64 {
    hash_builder.hash_one(value)
    /*
    // Create a new hasher
    let mut hasher = hash_builder.build_hasher();
    // Hash the given value
    value.hash(&mut hasher);
    // Finish and return the hash
    hasher.finish()
    */
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

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Hash + Eq,
    {
        // Compute the hash for the entry
        let wanted_hash = make_hash(&key, &self.hash_builder);

        for (hash, _key, value) in self.data.iter() {
            if &wanted_hash == hash {
                return Some(value);
            }
        }
        None
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
        self.data.push((hash, key, value));
        None
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

    #[test]
    fn get_what_you_give() {
        fn property(key: u16, value: u16) -> TestResult {
            let mut system_under_test = HashMap::new();

            assert_eq!(None, system_under_test.get(&key));
            assert_eq!(None, system_under_test.insert(key, value));
            assert_eq!(Some(&value), system_under_test.get(&key));

            TestResult::passed()
        }

        QuickCheck::new().quickcheck(property as fn(u16, u16) -> TestResult);
    }

    #[derive(Clone, Debug)]
    enum Action<T: Arbitrary> {
        Insert(T, u16),
        Lookup(T),
    }

    impl<T: Arbitrary> Arbitrary for Action<T> {
        // Required method
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            // Generate a random number
            let choice = g.gen_range(0, 100);

            match choice {
                0..50 => Self::Insert(T::arbitrary(g), u16::arbitrary(g)),
                _ => Self::Lookup(T::arbitrary(g)),
            }
        }
    }

    #[test]
    fn sut_vs_std_hashmap() {
        use crate::Hash;

        fn property<T>(actions: Vec<Action<T>>) -> TestResult
        where
            T: Arbitrary + Eq + Hash + Clone,
        {
            let mut model = std::collections::HashMap::new();
            let mut sys_under_test = HashMap::new();

            for action in actions {
                match action {
                    Action::Insert(key, value) => assert_eq!(
                        model.insert(key.clone(), value),
                        sys_under_test.insert(key.clone(), value)
                    ),
                    Action::Lookup(key) => assert_eq!(model.get(&key), sys_under_test.get(&key)),
                }
            }
            TestResult::passed()
        }

        QuickCheck::new().quickcheck(property as fn(Vec<Action<u8>>) -> TestResult);
    }
}
