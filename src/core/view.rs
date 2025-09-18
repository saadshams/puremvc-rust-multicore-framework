use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::interfaces::{IMediator, INotification, IObserver, IView};
use crate::patterns::Observer;

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

    pub fn get_instance<T: IView>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IView> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.to_string())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_view();
                Arc::new(instance)
            })
            .clone()
    }

    pub fn remove_view(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IView for View {
    fn initialize_view(&self){
        
    }
    
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>) {
        self.observer_map.lock().ok()
            .map(|mut map| {
                map.entry(notification_name.to_string())
                    .or_default()
                    .push(observer);
            });
    }

    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>) {
        self.observer_map.lock().ok()
            .and_then(|mut map| map.get_mut(notification_name).cloned())
            .map(|mut observers| {
                observers.retain(|observer| !observer.compare_notify_context(&context));

                if observers.is_empty() {
                    self.observer_map.lock().ok()
                        .map(|mut map| map.remove(notification_name));
                }
            });
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        self.observer_map.lock().ok()
            .and_then(|map| map.get(notification.name()).cloned())
            .map(|observers| {
                observers.iter().for_each(|observer| {
                    observer.notify_observer(notification);
                });
            });
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        let mut guard = mediator.lock().unwrap();
        {
            let mut map = self.mediator_map.lock().unwrap();
            if map.contains_key(guard.name()) { return }
            map.insert(guard.name().to_string(), Arc::clone(&mediator));
        }

        let notify = {
            let mediator = Arc::clone(&mediator);
            Arc::new(move |notification: &Arc<dyn INotification>| {
                mediator.lock().unwrap().handle_notification(notification);
            })
        };

        for interest in guard.list_notification_interests() {
            let context = Arc::new(Arc::clone(&mediator));
            let observer = Observer::new(Some(notify.clone()), Some(context));
            self.register_observer(&interest, Arc::new(observer));
        }

        guard.initialize_notifier(&self.key);
        guard.on_register();
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.mediator_map.lock().ok()
            .map(|map| map.get(mediator_name).cloned())
            .unwrap()
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.mediator_map.lock().ok()
            .map(|map| map.contains_key(mediator_name))
            .unwrap()
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.mediator_map.lock().ok()
            .and_then(|mut map| map.remove(mediator_name))
            .map(|mediator| {
                let mut guard = mediator.lock().unwrap();
                for interest in guard.list_notification_interests() {
                    self.remove_observer(&interest, Arc::new(Arc::clone(&mediator)));
                }
                guard.on_remove();
                mediator.clone()
            })
    }
}
