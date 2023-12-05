use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Calculates the hash of a string
pub(crate) fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    format!("{:x}", s.finish())
}