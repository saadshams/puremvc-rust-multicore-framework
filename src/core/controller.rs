use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{ICommand, INotification};
use crate::interfaces::IController;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

static MULTITON_MSG: &str = "Controller instance for this Multiton key already constructed!";

pub struct Controller {
    key: String,
    command_map: Mutex<HashMap<String, Arc<dyn Fn() -> Arc<Mutex<dyn ICommand>> + Send + Sync>>>,
}

impl Controller {
    pub fn new(key: &str) -> Self {
        // if INSTANCE_MAP.lock().unwrap().contains_key(key) {
        //     panic!("{}", MULTITON_MSG);
        // }

        Self {
            key: key.to_string(),
            command_map: Mutex::new(HashMap::new())
        }
    }

    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IController>) -> Arc<dyn IController> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }
}

impl IController for Controller {
    fn execute_command(&self, notification: Arc<Mutex<dyn INotification>>) {
        let map = self.command_map.lock().unwrap();
        if let Some(factory) = map.get(notification.lock().unwrap().name()) {
            // instance.initialize_notifier(&self.key);
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

        if let Some(_factory) = map.get(notification_name) {
            // The command exists, do whatever extra logic you need here
            println!("Removing command for notification: {}", notification_name);

            // For example, you could notify a view or cleanup something
        }

        // Now remove it
        map.remove(notification_name);
    }
}
