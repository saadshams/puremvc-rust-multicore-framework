use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::IMediator;
use crate::interfaces::IView;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(|| Default::default());

pub struct View {
    pub key: String,
    mediator_map: Mutex<HashMap<String, Arc<Mutex<dyn IMediator>>>>,
}

impl View {
    pub fn new(key: String) -> Self {
        Self {
            key,
            mediator_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance(key: String, factory: impl Fn(String) -> Box<dyn IView>) -> Arc<dyn IView> {
        INSTANCE_MAP
            .lock()
            .unwrap()
            .entry(key.clone())
            .or_insert_with(|| Arc::from(factory(key)))
            .clone()
    }
}

impl IView for View {
    fn key(&self) -> &str {
        &self.key
    }

    fn register_mediator(&self, mut mediator: Arc<dyn IMediator>) {
        let name = mediator.name().to_string();

        {
            // lock only for insertion
            let mut map = self.mediator_map.lock().unwrap();
            map.insert(name, Arc::clone(&mediator));
        }

        // call after releasing lock to avoid deadlocks
        mediator.on_register();
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<dyn IMediator>> {
        self.mediator_map.lock().unwrap().get(mediator_name).cloned()
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.mediator_map.lock().unwrap().contains_key(mediator_name)
    }

    // fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<dyn IMediator>> {
    //     let maybe_mediator = {
    //         self.mediator_map.lock().unwrap().get(mediator_name).cloned()
    //     };
    // 
    //     if let Some(mediator) = &maybe_mediator {
    //         let interests = mediator.list_notification_interests();
    //         for interest in interests {
    //             // self.remove_observer(&interest, mediator_name);
    //         }
    //     }
    // 
    //     if let Some(mediator) = &maybe_mediator {
    //         let interests = mediator.lock().unwrap().list_notification_interests();
    //         for interest in interests {
    //             // self.remove_observer(&interest, mediator_name);
    //         }
    //     }
    // }
}
