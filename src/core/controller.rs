use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{ICommand, INotification, View};
use crate::interfaces::{IController, IView};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

pub struct Controller {
    key: String,
    view: Option<Arc<dyn IView>>,
    command_map: Mutex<HashMap<String, Arc<dyn Fn() -> Arc<Mutex<dyn ICommand>> + Send + Sync>>>,
}

impl Controller {
    pub fn new(key: &str) -> Self {
        let mut instance = Self {
            key: key.to_string(),
            view: None,
            command_map: Mutex::new(HashMap::new()),
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
    fn execute_command(&self, notification: Arc<Mutex<dyn INotification>>) {
        let map = self.command_map.lock().unwrap();

        if let Some(factory) = map.get(notification.lock().unwrap().name()) {
            let instance = factory();
            let mut command = instance.lock().unwrap();
            command.execute(notification.clone());
        }
    }

    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Arc<Mutex<dyn ICommand>> + Send + Sync>) {
        let mut map = self.command_map.lock().unwrap();
        map.insert(notification_name.to_string(), factory);
    }

    fn has_command(&self, notification_name: &str) -> bool {
        let map = self.command_map.lock().unwrap();
        map.contains_key(notification_name)
    }

    fn remove_command(&self, notification_name: &str) {
        let mut map = self.command_map.lock().unwrap();

        if let Some(factory) = map.get(notification_name) {
            if let Some(view) = self.view.clone() {
                let this = Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k)));
                view.remove_observer(notification_name, &Arc::new(Box::new(this)));
            }

            println!("Removing command for notification: {}", notification_name);
        }

        map.remove(notification_name);
    }
}
