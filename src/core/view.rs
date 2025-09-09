use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IMediator, INotification, IObserver, Observer};
use crate::interfaces::IView;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(|| Default::default());

pub struct View {
    key: String,
    observer_map: Mutex<HashMap<String, Vec<Arc<dyn IObserver>>>>,
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

impl dyn IView {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl IView for View {
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>) {
        let mut map = self.observer_map.lock().unwrap();
        map.entry(notification_name.to_string()).or_default().push(observer);
    }

    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>) {
        let mut map = self.observer_map.lock().unwrap();

        if let Some(observers) = map.get_mut(notification_name) {
            observers.retain(|observer| observer.compare_notify_context(&context) == false);

            if observers.is_empty() {
                map.remove(notification_name);
            }
        }
    }

    fn notify_observers(&self, notification: &Arc<Mutex<dyn INotification>>) {
        let observers_ref = {
            let notification_name = notification.lock().unwrap().name().to_string();
            let map = self.observer_map.lock().unwrap();
            map.get(&notification_name).map(|list| list.iter().cloned().collect::<Vec<_>>())
        };

        if let Some(observers) = observers_ref {
            for observer in observers {
                observer.notify_observer(notification);
            }
        }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        {
            let mut map = self.mediator_map.lock().unwrap();
            let mut guard = mediator.lock().unwrap();
            map.insert(guard.name().to_string(), mediator.clone());
            guard.notifier().initialize_notifier(&self.key);
        }

        let context: Arc<dyn Any + Send + Sync> = Arc::new(mediator.clone());
        let notify = Arc::new({
            let ctx = mediator.clone();
            move |notification: &Arc<Mutex<dyn INotification>>| {
                ctx.lock().unwrap().handle_notification(&notification);
            }
        });

        let mut guard = mediator.lock().unwrap();
        for interest in guard.list_notification_interests() {
            let observer = Arc::new(Observer::new(Some(notify.clone()), Some(context.clone())));
            self.register_observer(&interest, observer.clone());
        }

        guard.on_register();
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
        let removed = {
            let mut map = self.mediator_map.lock().unwrap();
            map.remove(mediator_name)
        };

        if let Some(mediator) = &removed {
            let interests = {
                mediator.lock().unwrap().list_notification_interests()
            };

            let context: Arc<dyn Any + Send + Sync> = Arc::new(mediator.clone());
            for interest in interests {
                self.remove_observer(&interest, context.clone());
            }
            mediator.lock().unwrap().on_remove();
            removed
        } else {
            None
        }
    }
}
