use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IMediator, INotification, IObserver};
use crate::interfaces::IView;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(|| Default::default());

pub struct View {
    key: String,
    observer_map: Mutex<HashMap<String, Vec<Arc<Box<dyn IObserver>>>>>,
    mediator_map: Mutex<HashMap<String, Arc<Mutex<dyn IMediator>>>>,
}

impl View {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            observer_map: Mutex::new(HashMap::new()),
            mediator_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IView>) -> Arc<dyn IView> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }
}

impl IView for View {
    fn register_observer(&self, notification_name: &str, observer: Arc<Box<dyn IObserver>>) {
        let mut map = self.observer_map.lock().unwrap();
        map.entry(notification_name.to_string()).or_default().push(observer);
    }

    fn remove_observer(&self, notification_name: &str, context: &Arc<Box<dyn Any + Send + Sync>>) {
        let mut map = self.observer_map.lock().unwrap();

        if let Some(observers) = map.get_mut(notification_name) {
            if let Some(position) = observers.iter().position(|observer| {
                observer.compare_notify_context(context)
            }) {
                observers.remove(position);
            }

            if observers.is_empty() {
                map.remove(notification_name);
            }
        }
    }

    fn notify_observers(&self, notification: &Arc<Mutex<dyn INotification>>) {
        let map = self.observer_map.lock().unwrap();

        let notification_name = {
            let note = notification.lock().unwrap();
            note.name().to_string()
        };

        if let Some(observers_ref) = map.get(&notification_name) {
            // Copy observers to a working array to avoid holding the lock while notifying
            let observers: Vec<Arc<Box<dyn IObserver>>> = observers_ref.iter().cloned().collect();

            for observer in observers {
                observer.notify_observer(notification);
            }
        }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        let mut map = self.mediator_map.lock().unwrap();
        map.insert(mediator.lock().unwrap().name().to_string(), mediator.clone());
        mediator.lock().unwrap().on_register();
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        let map = self.mediator_map.lock().unwrap();
        map.get(mediator_name).cloned()
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        let map = self.mediator_map.lock().unwrap();
        map.contains_key(mediator_name)
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        let mut map = self.mediator_map.lock().unwrap();
        let removed = map.remove(mediator_name);

        if let Some(mediator) = removed.clone() {
            let interests = mediator.lock().unwrap().list_notification_interests();
            let context = Arc::new(Box::new(mediator.clone()) as Box<dyn Any + Send + Sync>);
            for interest in interests {
                self.remove_observer(&interest, &context);
            }
            removed
        } else {
            None
        }
    }
}
