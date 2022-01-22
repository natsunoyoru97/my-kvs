#![deny(missing_docs)]
//! A simple key-value storage implementation.

use std::{
    collections::HashMap,
    fs::File,
    io,
    io::prelude::*,
    path::PathBuf
};
use serde::{Deserialize, Serialize};

/// Self-defined key-value errors.
#[derive(Debug)]
pub enum KvError {
    /// The key is invalid.
    InvalidKey,
    /// The key cannot be read.
    KeyReadError,
    /// The key cannot be written.
    KeyWriteError,
}

/// Self-defined key-value result.
pub type KvResult<T> = Result<T, KvError>;

/// The key-value pair stores in KvStore.
#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct KvPair {
    _key: String,
    _value: String,
}

/// The simple key-value pair container.
pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Initialize the key-value pair HashMap container.
    pub fn new() -> Self {
        KvStore { store: HashMap::new() }
    }

    /// Open a key-value store from a specified path.
    pub fn open(path: impl Into<PathBuf> + std::convert::AsRef<std::path::Path>) -> KvResult<KvStore> {
        let mut file = File::open(&path);
        Ok(KvStore::new())
    }

    /// Set the value of specified key-value pair.
    /// If the key is in the container, just modify its value;
    /// Or a new key-value pair will be inserted to the container.
    pub fn set(&mut self, key: String, value: String) -> KvResult<()> {
        let new_pair =(&key, &value);
        match self.store.insert(key.to_owned(), value.to_owned()) {
            Some(v) => {
                // There is an existing key.
            },
            None => {
                // There is no existing key.
            },
        }

        let json = serde_json::to_string(&new_pair).unwrap();
        println!("{}", json);

        Ok(())
    }

    /// Get the value of the key-value pair, given a specific key.
    /// If there is a pair, the function will return an Option that contains the value;
    /// Or the function will return None.
    /// It returns an error if the value is not read successfully.
    pub fn get(&self, key: String) -> KvResult<Option<String>> {
        match self.store.get(&key) {
            Some(v) => Ok(Some(v.to_owned())),
            None => Ok(None),
        }
    }

    /// Remove a key-value pair, given a specific key.
    /// If there is a pair, the function will return Ok(());
    /// Or the function will throw InvalidKey Error.
    pub fn remove(&mut self, key: String) -> KvResult<()> {
        match self.store.remove_entry(&key) {
            Some(_) => Ok(()),
            None => Err(KvError::InvalidKey),
        }
    }
}
