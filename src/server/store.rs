use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type Store = HashMap<String, String>;
type StoreRecords = Arc<RwLock<Store>>;

pub struct StoreRegistry {
    pub cache: StoreRecords,
}

impl StoreRegistry {
    pub fn new() -> StoreRegistry {
        return StoreRegistry {
            cache: Arc::new(RwLock::new(Store::new())),
        };
    }

    pub fn add(&mut self, key: String, value: String) -> bool {
        if self.exists(key.clone()) {
            return false;
        } else {
            let _ = &self
                .cache
                .write()
                .ok()
                .and_then(|mut g| g.insert(key, value));
        }

        return true;
    }

    pub fn get(&self, key: String) -> Option<String> {
        let v = self
            .cache
            .read()
            .ok()
            .and_then(|g| g.get::<String>(&key.into()).cloned());

        return v;
    }

    pub fn exists<S: Into<String>>(&self, key: S) -> bool {
        return self
            .cache
            .read()
            .ok()
            .map(|g| g.contains_key(&key.into()))
            .unwrap();
    }

    pub fn delete<S: Into<String>>(&self, key: S) -> bool {
        let k = key.into();

        if self.exists(k.clone()) {
            let _ = self.cache.write().ok().and_then(|mut g| g.remove(&k));

            return true;
        } else {
            return false;
        }
    }
}
