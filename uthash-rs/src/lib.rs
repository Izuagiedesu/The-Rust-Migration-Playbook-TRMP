// uthash-rs/src/lib.rs
use std::collections::HashMap;

pub struct UserDatabase {
    // We replace the C "global pointer" with a safe, owned HashMap
    users: HashMap<i32, String>,
}

impl UserDatabase {
    /// Create a new, pure-Rust database
    pub fn new() -> Self {
        UserDatabase {
            users: HashMap::new(),
        }
    }

    /// Add a user (Replaces HASH_ADD_INT)
    pub fn add(&mut self, id: i32, name: &str) -> Result<(), String> {
        // In C, we had to check for existence, malloc, and macro-add.
        // In Rust, we just insert.
        self.users.insert(id, name.to_string());
        Ok(())
    }

    /// Find a user (Replaces HASH_FIND_INT)
    pub fn find(&self, id: i32) -> Option<String> {
        // In C, we handled pointers and null checks.
        // In Rust, we get an Option reference and clone the string if found.
        self.users.get(&id).cloned()
    }
}

// NOTE: We do NOT need a `Drop` implementation.
// Rust automatically frees the HashMap memory when it goes out of scope.
// This prevents the "memory leaks" common in the C version.

// --- THE TEST (UNCHANGED) ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_manages_users_via_pure_rust() {
        let mut db = UserDatabase::new();

        // 1. Add Users
        db.add(1, "Izuagie").expect("Failed to add");
        db.add(100, "Rust").expect("Failed to add");

        // 2. Find Users
        assert_eq!(db.find(1), Some("Izuagie".to_string()));
        assert_eq!(db.find(100), Some("Rust".to_string()));
        
        // 3. Find Missing User
        assert_eq!(db.find(999), None);
    }
}