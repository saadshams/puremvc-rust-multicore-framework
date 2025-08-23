use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IModel, IProxy};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IModel>>>> = LazyLock::new(|| Default::default());

pub struct Model {
    pub key: String,
    proxy_map: Mutex<HashMap<String, Arc<dyn IProxy>>>,
}

impl Model {
    pub fn new(key: String) -> Self {
        Self {
            key,
            proxy_map: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn get_instance(key: String, factory: impl Fn(String) -> Box<dyn IModel>) -> Arc<dyn IModel> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.clone())
            .or_insert_with(|| Arc::from(factory(key)))
            .clone()
    }
}

impl IModel for Model {
    fn key(&self) -> &str {
        &self.key
    }

    fn register_proxy(&self, proxy: Arc<dyn IProxy>) {
        self.proxy_map.lock().unwrap().insert(proxy.name().to_string(), proxy);
    }

    fn retrieve_proxy(&self, name: &str) -> Option<Arc<dyn IProxy>> {
        self.proxy_map.lock().unwrap().get(name).cloned()
    }

    fn has_proxy(&self, name: &str) -> bool {
        self.proxy_map.lock().unwrap().contains_key(name)
    }

    fn remove_proxy(&self, name: &str) -> Option<Arc<dyn IProxy>> {
        self.proxy_map.lock().unwrap().remove(name)
    }
}
