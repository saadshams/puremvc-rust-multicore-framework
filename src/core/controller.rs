use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{ICommand, INotification, Observer, View};
use crate::interfaces::{IController, IView};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

pub struct Controller {
    key: String,
    view: Option<Arc<dyn IView>>,
    command_map: Mutex<HashMap<String, Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>>>,
}

impl dyn IController {
    pub(crate) fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn execute_command(&self, notification: &Arc<Mutex<dyn INotification>>) {
        let factory = {
            let map = self.command_map.lock().unwrap();
            map.get(notification.lock().unwrap().name()).cloned()
        };

        if let Some(factory) = factory {
            let mut command = factory();
            command.notifier().initialize_notifier(&self.key);
            command.execute(notification);
        }
    }

    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>) {
        let mut map = self.command_map.lock().unwrap();

        if !map.contains_key(notification_name) {
            let context = Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k)));
            let notify = {
                let controller = context.clone();
                move |notification: &Arc<Mutex<dyn INotification>>| {
                    controller.execute_command(&notification);
                }
            };

            let observer = Observer::new(Some(Arc::new(notify)), Some(Arc::new(context)));
            self.view.as_ref().unwrap().register_observer(notification_name, Arc::new(observer));
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
                let controller: Arc<dyn IController> = Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k)));
                let controller_any: Arc<dyn Any + Send + Sync> = Arc::new(controller.clone());

                view.remove_observer(notification_name, controller_any.clone());
            }
        }
    }
}
