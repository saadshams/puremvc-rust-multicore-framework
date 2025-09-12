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

    pub fn get_instance(key: &str, factory: fn(&str) -> View) -> Arc<dyn IView> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.to_string())
            .or_insert_with(|| {
                let mut instance = factory(key);
                instance.initialize_view();
                Arc::new(instance)
            }).clone()
    }

    pub fn remove_view(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IView for View {
    fn initialize_view(&mut self){
        
    }
    
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

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        if let Some(observers) = self.observer_map.lock().unwrap().get(&notification.name().to_string()) {
            observers.iter().for_each(|observer| {
                observer.notify_observer(notification)
            });
        }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        let mut guard = mediator.lock().unwrap();
        {
            let mut map = self.mediator_map.lock().unwrap();
            if map.contains_key(guard.name()) { return }

            guard.notifier().initialize_notifier(&self.key);
            map.insert(guard.name().to_string(), Arc::clone(&mediator));
        }

        let notify = {
            let mediator = Arc::clone(&mediator);
            Arc::new(move |notification: &Arc<dyn INotification>| {
                mediator.lock().unwrap().handle_notification(notification);
            })
        };

        for interest in guard.list_notification_interests() {
            let observer = Observer::new(Some(notify.clone()), Some(Arc::new(Arc::clone(&mediator))));
            self.register_observer(&interest, Arc::new(observer));
        }

        guard.on_register();
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.mediator_map.lock().unwrap().get(mediator_name).cloned()
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.mediator_map.lock().unwrap().contains_key(mediator_name)
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        let mediator = {
            let mut map = self.mediator_map.lock().unwrap();
            map.remove(mediator_name)
        };

        if let Some(mediator) = &mediator {
            let mut guard = mediator.lock().unwrap();
            for interest in guard.list_notification_interests() {
                self.remove_observer(&interest, Arc::new(Arc::clone(mediator)));
            }
            guard.on_remove();
        }

        mediator
    }
}
