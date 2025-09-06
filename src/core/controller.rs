use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{ICommand, INotification, IObserver, Observer, View};
use crate::interfaces::{IController, IView};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(Default::default);

pub struct Controller {
    key: String,
    view: Option<Arc<dyn IView>>,
    command_map: Mutex<HashMap<String, Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>>>,
    observer_contexts: Mutex<HashMap<String, Arc<Box<dyn std::any::Any + Send + Sync>>>>,
}

impl Controller {
    pub fn new(key: &str) -> Self {
        let mut instance = Self {
            key: key.to_string(),
            view: None,
            command_map: Mutex::new(HashMap::new()),
            observer_contexts: Mutex::new(HashMap::new()),
        };

        instance.initialize_controller();
        instance
    }

    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IController>) -> Arc<dyn IController> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }

    pub fn initialize_controller(&mut self) {
        self.view = Some(View::get_instance(&self.key, |k| Arc::new(View::new(k))));
    }
}

impl IController for Controller {
    fn execute_command(&self, notification: &Arc<Mutex<dyn INotification>>) {
        let factory = {
            let map = self.command_map.lock().unwrap();
            map.get(notification.lock().unwrap().name()).cloned()
        };

        if let Some(factory) = factory {
            let mut command = factory();
            command.notifier_mut().initialize_notifier(&self.key);
            command.execute(notification);
        }
    }

    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>) {
        let mut map = self.command_map.lock().unwrap();

        if !map.contains_key(notification_name) {
            let context = Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k)));
            let ctx_clone = context.clone();
            let notify = Arc::new(move |note: &Arc<Mutex<dyn INotification>>| {
                ctx_clone.execute_command(note);
            });

            let context_ref = Arc::new(Box::new(context.clone()) as Box<dyn std::any::Any + Send + Sync>);
            let observer = Observer::new(Some(notify), Some(context_ref.clone()));
            let observer2: Arc<Box<dyn IObserver>> = Arc::new(Box::new(observer));

            
            {
                let mut contexts = self.observer_contexts.lock().unwrap();
                contexts.insert(notification_name.to_string(), context_ref);
            }

            self.view.as_ref().unwrap().register_observer(notification_name, observer2);
        }

        map.insert(notification_name.to_string(), factory);
    }

    fn has_command(&self, notification_name: &str) -> bool {
        let map = self.command_map.lock().unwrap();
        map.contains_key(notification_name)
    }

    fn remove_command(&self, notification_name: &str) {
        let mut map = self.command_map.lock().unwrap();
        let removed = map.remove(notification_name);

        if removed.is_some() {
            if let Some(view) = &self.view {
                // Use the stored context reference for removal
                let context_ref = {
                    let mut contexts = self.observer_contexts.lock().unwrap();
                    contexts.remove(notification_name)
                };
                
                if let Some(context) = context_ref {
                    view.remove_observer(notification_name, &context);
                }
            }
        }
    }
}
