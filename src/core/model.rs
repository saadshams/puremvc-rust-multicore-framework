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

    // pub fn get_instance<T>(key: &str, factory: fn(&str) -> T) -> Arc<dyn IModel> where T: IModel + 'static {
    //     INSTANCE_MAP.lock().unwrap()
    //         .entry(key.to_string())
    //         .or_insert_with(|| {
    //             let mut instance = factory(key);
    //             instance.initialize_model();
    //             Arc::new(instance)
    //         })
    //         .clone()
    // }
    //
    // pub fn get_instance2(key: &str, factory: fn(&str) -> Box<dyn IModel>) -> Arc<dyn IModel> {
    //     INSTANCE_MAP.lock().unwrap()
    //         .entry(key.to_string())
    //         .or_insert_with(|| {
    //             let mut instance = factory(key);
    //             instance.initialize_model();
    //             Arc::from(instance)
    //         }).clone()
    // }

    pub fn get_instance<T: IModel>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IModel> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.to_string())
            .or_insert_with(|| {
                let mut instance = factory(key);
                instance.initialize_model();
                Arc::new(instance)
            }).clone()
    }
    
    pub fn remove_model(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IModel for Model {
    fn initialize_model(&mut self) {

    }

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) {
        let mut guard = proxy.lock().unwrap();
        {
            let mut map = self.proxy_map.lock().unwrap();
            map.insert(guard.name().to_string(), Arc::clone(&proxy));
        }

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
            let mut guard = proxy.lock().unwrap();
            guard.on_remove();
        }

        removed
    }
}
