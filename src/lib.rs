#![deny(missing_docs)]
//! A simple key-value storage implementation.

/// The key-value pair stores in KvStore.
struct KvPair {
    _key: String,
    _value: String,
}

/// Implementation of KvPair
impl KvPair {
    pub fn set(&mut self, key: String, value: String) {
        self._key = key;
        self._value = value;
    }

    pub fn get(&self) -> String {
        self._value.to_owned()
    }
}

/// The simple key-value pair container.
pub struct KvStore {
    store: Vec<KvPair>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Initialize the key-value pair container.
    /// This struct use Vec as the actual container,
    /// after initialization, the Vec is empty.
    pub fn new() -> Self {
        KvStore { store: Vec::new() }
    }

    /// Set the value of specified key-value pair.
    /// If the key is in the container, just modify its value;
    /// Or a new key-value pair will be added to the container.
    pub fn set(&mut self, key: String, value: String) {
        for kv_pair in self.store.iter_mut() {
            if kv_pair._key.eq(&key) {
                (*kv_pair).set(key, value);
                return;
            }
        }

        self.store.push(KvPair {
            _key: key,
            _value: value,
        });
    }

    /// Get the value of the key-value pair, given a specific key.
    /// If there is a pair, the function will return an Option that contains the value;
    /// Or the function will return None.
    pub fn get(&self, key: String) -> Option<String> {
        for kv_pair in self.store.iter() {
            if kv_pair._key.eq(&key) {
                let result = kv_pair.get();
                return Some(result);
            }
        }

        None
    }

    /// Remove a key-value pair, given a specific key.
    /// If there is a pair, the function will return the index the pair was in;
    /// Or the function will return None.
    pub fn remove(&mut self, key: String) -> Option<usize> {
        let mut index = self.store.len() + 1;
        // TODO: Support enumerate through KvStore
        for i in 0..self.store.len() {
            if self.store[i]._key.eq(&key) {
                index = i;
            }
        }

        if index < self.store.len() {
            self.store.remove(index);
            return Some(index);
        }

        None
    }
}
