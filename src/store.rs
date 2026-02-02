use std::{collections::HashMap, sync::RwLock};
pub enum Command {
    Get(String),
    Set(String, String),
    Del(String),
    Unknown,
}

impl Command {
    pub fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["GET", key] => Command::Get(key.to_string()),
            ["SET", key, val] => Command::Set(key.to_string(), val.to_string()),
            ["DEL", key] => Command::Del(key.to_string()),
            _ => Command::Unknown,
        }
    }
}

pub trait Storage<T> {
    fn get(&self, key: &str) -> Option<T>;
    fn set(&self, key: String, val: T);
    fn delete(&self, key: &str) -> bool;
}

pub struct Store<T> {
    inner: RwLock<HashMap<String, T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

impl<T> Storage<T> for Store<T>
where
    T: Clone + Send + Sync,
{
    fn get(&self, key: &str) -> Option<T> {
        let val = &self.inner.read().unwrap();
        val.get(key).cloned()
    }

    fn set(&self, key: String, val: T) {
        let mut map = self.inner.write().unwrap();
        map.insert(key, val);
    }

    fn delete(&self, key: &str) -> bool {
        let mut map = self.inner.write().unwrap();
        map.remove(key).is_some()
    }
}

