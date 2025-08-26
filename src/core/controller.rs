use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{ICommand, INotification};
use crate::interfaces::IController;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

pub struct Controller {
    pub key: String,
    command_map: Mutex<HashMap<String, Box<dyn Fn() -> Box<dyn ICommand> + Send + Sync> >>,
}

impl Controller {
    pub fn new(key: String) -> Self {
        Self {
            key,
            command_map: Mutex::new(HashMap::new())
        }
    }

    pub fn get_instance(key: String, factory: impl Fn(String) -> Box<dyn IController>) -> Arc<dyn IController> {
        INSTANCE_MAP.
            lock()
            .unwrap()
            .entry(key.clone())
            .or_insert_with(|| Arc::from(factory(key)))
            .clone()
    }
}

impl IController for Controller {
    fn key(&self) -> &str {
        &self.key
    }

    fn execute_command(&self, notification: &mut dyn INotification) {
        if let Some(factory) = self.command_map.lock().unwrap().get(notification.name()) {
            let mut instance = factory();
            // instance.initialize_notifier(&self.multiton_key);
            instance.execute(notification);
        }
    }

    fn register_command(&self, notification_name: String, factory: Box<dyn Fn() -> Box<dyn ICommand> + Send + Sync>) {
        // todo register with view
        self.command_map.lock().unwrap().insert(notification_name, factory);
    }

    fn has_command(&self, notification_name: &str) -> bool {
        self.command_map.lock().unwrap().contains_key(notification_name)
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
