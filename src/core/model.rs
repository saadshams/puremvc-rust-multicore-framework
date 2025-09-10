use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IModel, IProxy};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IModel>>>> = LazyLock::new(|| Default::default());

pub struct Model {
    key: String,
    proxy_map: Mutex<HashMap<String, Arc<Mutex<dyn IProxy>>>>,
}

impl Model {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            proxy_map: Mutex::new(HashMap::new())
        }
    }
    
    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IModel>) -> Arc<dyn IModel> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }
    
    pub fn remove_model(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl dyn IModel {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl IModel for Model {
    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) {
        {
            let mut map = self.proxy_map.lock().unwrap();
            map.insert(proxy.lock().unwrap().name().to_string(), Arc::clone(&proxy));
        }

        let mut guard = proxy.lock().unwrap();
        guard.notifier().initialize_notifier(&self.key);
        guard.on_register();
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        self.proxy_map.lock().unwrap().get(proxy_name).cloned()
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.proxy_map.lock().unwrap().contains_key(proxy_name)
    }

    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        let removed = {
            let mut map = self.proxy_map.lock().unwrap();
            map.remove(proxy_name)
        };

        if let Some(proxy) = &removed {
            proxy.lock().unwrap().on_remove();
        }

        removed
    }
}
