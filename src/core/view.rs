use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IMediator, INotification, IObserver};
use crate::interfaces::IView;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(|| Default::default());

static MULTITON_MSG: &str = "View instance for this Multiton key already constructed!";

pub struct View {
    key: String,
    mediator_map: Mutex<HashMap<String, Arc<Mutex<dyn IMediator + Send>>>>,
}

impl View {
    pub fn new(key: &str) -> Self {
        if INSTANCE_MAP.lock().unwrap().contains_key(key) {
            panic!("{}", MULTITON_MSG);
        }

        Self {
            key: key.to_string(),
            mediator_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IView>) -> Arc<dyn IView> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }
}

impl IView for View {
    fn register_observer(&self, observer: Arc<Mutex<dyn IObserver + Send>>) {
        todo!()
    }

    fn remove_observer(&self, context: Option<Arc<dyn Any + Send + Sync>>) {
        todo!()
    }

    fn notify_observers(&self, notification: Arc<dyn INotification>) {
        todo!()
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator + Send>>) {
        let mut map = self.mediator_map.lock().unwrap();
        map.insert(mediator.lock().unwrap().name().to_string(), Arc::clone(&mediator));
        mediator.lock().unwrap().on_register();
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator + Send>>> {
        let map = self.mediator_map.lock().unwrap();
        map.get(mediator_name).cloned()
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        let map = self.mediator_map.lock().unwrap();
        map.contains_key(mediator_name)
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator + Send>>> {
        let mut map = self.mediator_map.lock().unwrap();
        let removed = map.remove(mediator_name);

        if let Some(mediator) = &removed {
            let interests = mediator.lock().unwrap().list_notification_interests();
            for interest in interests {
                // self.remove_observer(&interest, mediator_name);
            }
        }

        removed
    }
}
