use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::interfaces::{IModel, IProxy};

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

    pub fn get_instance<T: IModel>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IModel> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.to_string())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_model();
                Arc::new(instance)
            })
            .clone()
    }
    
    pub fn remove_model(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IModel for Model {
    fn initialize_model(&self) {

    }

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) {
        self.proxy_map.lock().ok()
            .map(|mut map| {
                let mut guard = proxy.lock().unwrap();
                map.insert(guard.name().to_string(), Arc::clone(&proxy));
                guard.initialize_notifier(&self.key);
                guard.on_register();
            });
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        self.proxy_map.lock().ok()
            .map(|map| map.get(proxy_name).cloned())
            .unwrap()
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.proxy_map.lock().ok()
            .map(|map| map.contains_key(proxy_name))
            .unwrap()
    }

    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        self.proxy_map.lock().ok()
            .and_then(|mut map| map.remove(proxy_name))
            .map(|proxy| {
                proxy.lock().unwrap().on_remove();
                proxy
            })
    }
}
