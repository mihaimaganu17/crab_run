use naive_hashmap::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Get a handle to the standard input
    let stdin = io::stdin();

    // Create a new HashMap
    let mut hash_map = HashMap::new();

    for line in stdin.lock().lines().map_while(Result::ok) {
        let mut cmd = line.split(" ");
        match cmd.next() {
            // If we are requested a lookup
            Some("LOOKUP") => {
                // If we have a key
                if let Some(key) = cmd.next() {
                    // Get the key
                    let _ = hash_map.get(key);
                } else {
                    continue;
                }
            }
            Some("INSERT") => {
                // If we have a key and a value
                if let (Some(key), Some(value)) = (cmd.next(), cmd.next()) {
                    // Insert the value and key
                    let _ = hash_map.insert(key.to_string(), value.to_string());
                } else {
                    continue;
                }
            }
            Some(_) | None => continue,
        }
    }
}
