use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct DataAccessLayer {
    pub db: Arc<Mutex<HashMap<String, String>>>,
}

impl DataAccessLayer {
    pub fn new() -> Self {
        DataAccessLayer {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        let store = match self.db.lock() {
            Ok(db) => db,
            Err(e) => {
                println!("[error] Failed to lock db with error: {err}", err = e);
                return None;
            }
        };
        match store.get(&key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        let mut store = match self.db.lock() {
            Ok(db) => db,
            Err(e) => {
                println!("[error] Failed to lock db with error: {err}", err = e);
                return;
            }
        };
        store.insert(key, value);
    }

    pub fn del(&mut self, key: String) {
        let mut store = match self.db.lock() {
            Ok(db) => db,
            Err(e) => {
                println!("[error] Failed to lock db with error: {err}", err = e);
                return;
            }
        };
        store.remove(&key);
    }
}
