use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use crate::interfaces::{IMediator, INotification, IObserver, IView};
use crate::patterns::Observer;

static INSTANCE_MAP: LazyLock<RwLock<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(|| Default::default());

pub struct View {
    key: String,
    observer_map: RwLock<HashMap<String, Vec<Arc<dyn IObserver>>>>,
    mediator_map: RwLock<HashMap<String, Arc<RwLock<dyn IMediator>>>>
}

impl View {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            observer_map: RwLock::new(HashMap::new()),
            mediator_map: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_instance<T: IView>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IView> {
        INSTANCE_MAP.write().unwrap()
            .entry(key.into())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_view();
                Arc::new(instance)
            })
            .clone()
    }

    pub fn remove_view(key: &str) {
        INSTANCE_MAP.write().unwrap().remove(key);
    }
}

impl IView for View {
    fn initialize_view(&self){
        
    }
    
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>) {
        self.observer_map.write().ok()
            .map(|mut map| {
                map.entry(notification_name.into())
                    .or_default()
                    .push(observer);
            });
    }

    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>) {
        self.observer_map.write().ok()
            .and_then(|mut map| map.get_mut(notification_name).cloned())
            .map(|mut observers| {
                observers.retain(|observer| !observer.compare_notify_context(&context));

                if observers.is_empty() {
                    self.observer_map.write().ok()
                        .map(|mut map| map.remove(notification_name));
                }
            });
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        self.observer_map.read().ok()
            .and_then(|map| map.get(notification.name()).cloned())
            .map(|observers| {
                observers.iter().for_each(|observer| {
                    observer.notify_observer(notification);
                });
            });
    }

    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>) {
        {
            let name = mediator.read().unwrap().name().to_string();
            let mut map = self.mediator_map.write().unwrap();
            if map.contains_key(&name) { return }
            map.insert(name, Arc::clone(&mediator));
        }

        let notify = {
            let mediator = Arc::clone(&mediator);
            Arc::new(move |notification: &Arc<dyn INotification>| {
                mediator.write().unwrap().handle_notification(notification);
            })
        };

        for interest in mediator.read().unwrap().list_notification_interests() {
            let context = Arc::new(Arc::clone(&mediator));
            let observer = Observer::new(Some(notify.clone()), Some(context));
            self.register_observer(&interest, Arc::new(observer));
        }

        {
            let mut guard = mediator.write().unwrap();
            guard.initialize_notifier(&self.key);
            guard.on_register();
        }
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.mediator_map.read().ok()
            .and_then(|map| map.get(mediator_name).cloned())
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.mediator_map.read().ok()
            .map(|map| map.contains_key(mediator_name))
            .unwrap()
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.mediator_map.write().ok()
            .and_then(|mut map| map.remove(mediator_name))
            .map(|mediator| {
                for interest in mediator.read().unwrap().list_notification_interests() {
                    self.remove_observer(&interest, Arc::new(Arc::clone(&mediator)));
                }
                mediator.write().unwrap().on_remove(); mediator })
    }
}
