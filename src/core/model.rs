use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IModel, IProxy};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IModel>>>> = LazyLock::new(|| Default::default());

static MULTITON_MSG: &str = "Model instance for this Multiton key already constructed!";

pub struct Model {
    key: String,
    proxy_map: Mutex<HashMap<String, Arc<Mutex<dyn IProxy + Send>>>>,
}

impl Model {
    pub fn new(key: &str) -> Self {
        // if INSTANCE_MAP.lock().unwrap().contains_key(key) {
        //     panic!("{}", MULTITON_MSG);
        // }
        //
        Self {
            key: key.to_string(),
            proxy_map: Mutex::new(HashMap::new())
        }
    }
    
    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IModel>) -> Arc<dyn IModel> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }

}

impl IModel for Model {
    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy + Send>>) {
        let mut map = self.proxy_map.lock().unwrap();
        map.insert(proxy.lock().unwrap().name().to_string(), Arc::clone(&proxy));
        proxy.lock().unwrap().on_register();
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy + Send>>> {
        let map = self.proxy_map.lock().unwrap();
        map.get(proxy_name).cloned()
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        let map = self.proxy_map.lock().unwrap();
        map.contains_key(proxy_name)
    }

    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy + Send>>> {
        let mut map = self.proxy_map.lock().unwrap();
        let removed = map.remove(proxy_name);

        if let Some(proxy) = &removed {
            proxy.lock().unwrap().on_remove();
        }

        removed
    }
}
