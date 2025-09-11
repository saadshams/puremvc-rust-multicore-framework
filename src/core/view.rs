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

impl Drop for View {
    fn drop(&mut self) {
        println!("View Dropped");
    }
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

    pub fn remove_view(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
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
            // let weak = Arc::downgrade(&context); // todo
            observers.retain(|observer| observer.compare_notify_context(&context) == false);

            if observers.is_empty() {
                map.remove(notification_name);
            }
        }
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        let notification_name = notification.name().to_string();
        if let Some(observers) = self.observer_map.lock().unwrap().get(&notification_name) {
            observers.iter().for_each(|observer| {
                observer.notify_observer(notification)
            });
        }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        {
            let mut map = self.mediator_map.lock().unwrap();
            if map.contains_key(mediator.lock().unwrap().name()) { return }

            let mut guard = mediator.lock().unwrap();
            guard.notifier().initialize_notifier(&self.key);
            let _name = guard.name().to_string();
            map.insert(guard.name().to_string(), Arc::clone(&mediator));
        }

        let context : Arc<Arc<Mutex<dyn IMediator>>> = Arc::new(mediator.clone());
        let weak = Arc::downgrade(&context);
        let notify = {
            let weak = weak.clone();
            Arc::new(move |notification: &Arc<dyn INotification>| {
                if let Some(arc) = weak.upgrade() {
                    let mediator: Arc<Mutex<dyn IMediator>> = Arc::clone(&*arc);
                    mediator.lock().unwrap().handle_notification(notification);
                }
            })
        };

        let interests = {
            mediator.lock().unwrap().list_notification_interests()
        };

        for interest in interests {
            let observer = Arc::new(Observer::new(Some(notify.clone()), Some(context.clone())));
            self.register_observer(&interest, observer.clone());
        }

        mediator.lock().unwrap().on_register();
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.mediator_map.lock().unwrap().get(mediator_name).cloned()
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.mediator_map.lock().unwrap().contains_key(mediator_name)
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        let mut map = self.mediator_map.lock().unwrap();

        if let Some(mediator) = map.remove(mediator_name) {
            let interests = {
                mediator.lock().unwrap().list_notification_interests()
            };

            let context: Arc<dyn Any + Send + Sync> = Arc::new(mediator.clone());
            for interest in interests {
                self.remove_observer(&interest, context.clone());
            }

            mediator.lock().unwrap().on_remove();
            Some(mediator)
        } else {
            None
        }
    }
}
