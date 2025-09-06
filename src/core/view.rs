use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IMediator, INotification, IObserver, Observer};
use crate::interfaces::IView;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IView>>>> = LazyLock::new(Default::default);

pub struct View {
    key: String,
    observer_map: Mutex<HashMap<String, Vec<Arc<Box<dyn IObserver>>>>>,
    mediator_map: Mutex<HashMap<String, Arc<Mutex<dyn IMediator>>>>,
    mediator_contexts: Mutex<HashMap<String, Arc<Box<dyn Any + Send + Sync>>>>,
}

impl View {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            observer_map: Mutex::new(HashMap::new()),
            mediator_map: Mutex::new(HashMap::new()),
            mediator_contexts: Mutex::new(HashMap::new()),
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
            observers.retain(|observer| !observer.compare_notify_context(context));

            if observers.is_empty() {
                map.remove(notification_name);
            }
        }
    }

    fn notify_observers(&self, notification: &Arc<Mutex<dyn INotification>>) {
        let observers_ref = {
            let notification_name = notification.lock().unwrap().name().to_string();
            let map = self.observer_map.lock().unwrap();
            map.get(&notification_name).map(|list| list.to_vec())
        };

        if let Some(observers) = observers_ref {
            for observer in observers {
                observer.notify_observer(notification);
            }
        }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        let mediator_name = mediator.lock().unwrap().name().to_string();
        
        // Register mediator in map
        {
            let mut map = self.mediator_map.lock().unwrap();
            map.insert(mediator_name.clone(), mediator.clone());
        }

        // Initialize and register mediator
        let mut guard = mediator.lock().unwrap();
        guard.notifier_mut().initialize_notifier(&self.key);
        
        // Get notification interests and register observers
        let interests = guard.list_notification_interests();
        if !interests.is_empty() {
            let context_ref = Arc::new(Box::new(mediator.clone()) as Box<dyn Any + Send + Sync>);
            
            // Store the context reference for later removal
            {
                let mut contexts = self.mediator_contexts.lock().unwrap();
                contexts.insert(mediator_name, context_ref.clone());
            }
            
            // Register observers for each notification interest
            for interest in interests {
                let med_clone = mediator.clone();
                let notify_fn = Arc::new(move |notification: &Arc<Mutex<dyn INotification>>| {
                    if let Ok(mut med) = med_clone.try_lock() {
                        med.handle_notification(notification);
                    }
                });
                
                let observer = Observer::new(Some(notify_fn), Some(context_ref.clone()));
                self.register_observer(&interest, Arc::new(Box::new(observer)));
            }
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
            let interests = mediator.lock().unwrap().list_notification_interests();
            let context = Arc::new(Box::new(mediator.clone()) as Box<dyn Any + Send + Sync>);
            for interest in interests {
                self.remove_observer(&interest, &context);
            }
            
            mediator.lock().unwrap().on_remove();
        }
        removed
    }
}
