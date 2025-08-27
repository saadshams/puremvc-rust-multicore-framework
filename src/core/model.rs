use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IModel, IProxy};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IModel>>>> = LazyLock::new(|| Default::default());

pub struct Model {
    pub key: String,
    proxy_map: Mutex<HashMap<String, Arc<Mutex<dyn IProxy + Send>>>>,
}

impl Model {
    pub fn new(key: String) -> Self {
        Self {
            key,
            proxy_map: Mutex::new(HashMap::new())
        }
    }
    
    pub fn get_instance(key: String, factory: impl Fn(String) -> Box<dyn IModel>) -> Arc<dyn IModel> {
        INSTANCE_MAP
            .lock()
            .unwrap()
            .entry(key.clone())
            .or_insert_with(|| Arc::from(factory(key)))
            .clone()
    }
}

impl IModel for Model {
    fn key(&self) -> &str {
        &self.key
    }

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy + Send>>) {
        // Lock the proxy to get a reference to call `name()`
        let name = proxy.lock().unwrap().name().to_string();

        // Insert into the map while holding the proxy_map lock
        {
            let mut map = self.proxy_map.lock().unwrap();
            map.insert(name, Arc::clone(&proxy));
        }

        // Call mutable method after releasing the map lock
        proxy.lock().unwrap().on_register();
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy + Send>>> {
        self.proxy_map.lock().unwrap().get(proxy_name).cloned()
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.proxy_map.lock().unwrap().contains_key(proxy_name)
    }

    fn remove_proxy(&self, name: &str) -> Option<Arc<Mutex<dyn IProxy + Send>>> {
        let removed = self.proxy_map.lock().unwrap().remove(name);

        if let Some(proxy) = &removed {
            // if proxy is Arc<dyn IProxy>, you can call the method via deref:
            proxy.lock().unwrap().on_remove();
        }

        removed
    }
}
