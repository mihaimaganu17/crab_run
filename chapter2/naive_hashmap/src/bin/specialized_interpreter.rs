use naive_hashmap::HashMapU8;
use std::io::{self, BufRead};

fn main() {
    // Get a handle to the standard input
    let stdin = io::stdin();

    // Create a new HashMap
    let mut hash_map = HashMapU8::new();

    for line in stdin.lock().lines().map_while(Result::ok) {
        let mut cmd = line.split(" ");
        match cmd.next() {
            // If we are requested a lookup
            Some("LOOKUP") => {
                // If we have a key
                if let Some(key) = cmd.next() {
                    if let Ok(key_u8) = key.parse::<u8>() {
                        // Get the key
                        let _ = hash_map.get(&key_u8);
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            Some("INSERT") => {
                // If we have a key and a value
                if let (Some(key), Some(value)) = (cmd.next(), cmd.next()) {
                    if let Ok(key_u8) = key.parse::<u8>() {
                        // Insert the value and key
                        let _ = hash_map.insert(key_u8, value.to_string());
                    }
                } else {
                    continue;
                }
            }
            Some(_) | None => continue,
        }
    }
}
