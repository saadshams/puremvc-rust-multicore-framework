use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, Weak};
use crate::core::View;
use crate::interfaces::{ICommand, IController, INotification, IView};
use crate::patterns::Observer;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

pub struct Controller {
    key: String,
    view: Weak<dyn IView>,
    command_map: Mutex<HashMap<String, fn() -> Box<dyn ICommand + Send + Sync>>>
}

impl Controller {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            view: Arc::downgrade(&(View::get_instance(&key, |k| View::new(k)))),
            command_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_instance<T: IController>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IController> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.to_string())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_controller();
                Arc::new(instance)
            })
            .clone()
    }

    pub fn remove_controller(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IController for Controller {
    fn initialize_controller(&self) {

    }

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.command_map.lock().ok()
            .map(|mut map| {
                if !map.contains_key(notification_name) && let Some(view) = self.view.upgrade() {
                    let context = Controller::get_instance(&self.key, |k| Controller::new(k));
                    let notify = {
                        let controller = Arc::clone(&context);
                        Arc::new(move |notification: &Arc<dyn INotification>| {
                            controller.execute_command(&notification);
                        })
                    };

                    let observer = Observer::new(Some(notify), Some(Arc::new(context)));
                    view.register_observer(notification_name, Arc::new(observer));
                }
                map.insert(notification_name.to_string(), factory)
            });
    }

    fn execute_command(&self, notification: &Arc<dyn INotification>) {
        let factory = self.command_map.lock().ok()
            .and_then(|map| map.get(notification.name()).cloned());

        if let Some(factory) = factory {
            let mut command = Box::new(factory());
            command.notifier().map(|notifier| {
                notifier.initialize_notifier(&self.key)
            });
            command.execute(notification);
        }
    }

    fn has_command(&self, notification_name: &str) -> bool {
        self.command_map.lock().ok()
            .map(|map| map.contains_key(notification_name))
            .unwrap_or(false)
    }

    fn remove_command(&self, notification_name: &str) {
        let existed = self.command_map.lock().ok()
            .and_then(|mut map| map.remove(notification_name))
            .is_some();

        if existed && let Some(view) = self.view.upgrade() {
            let context: Arc<dyn IController> = Controller::get_instance(&self.key, |k| Controller::new(k));
            view.remove_observer(notification_name, Arc::new(context));
        }
    }
}
