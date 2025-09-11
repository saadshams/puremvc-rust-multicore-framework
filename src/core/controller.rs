use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, Weak};
use crate::{ICommand, INotification, Observer, View};
use crate::interfaces::{IController, IView};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IController>>>> = LazyLock::new(|| Default::default());

pub struct Controller {
    key: String,
    view: Option<Weak<dyn IView>>,
    command_map: Mutex<HashMap<String, Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>>>,
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
        self.view = Some(Arc::downgrade(&(View::get_instance(&self.key, |k| Arc::new(View::new(k))))));    }
    
    pub fn remove_controller(key: &str) {
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IController for Controller {
    fn execute_command(&self, notification: &Arc<dyn INotification>) {
        let name = notification.name().to_string();
        if let Some(factory) = self.command_map.lock().unwrap().get(&name) {
            let mut command = factory();
            command.notifier().initialize_notifier(&self.key);
            command.execute(notification);
        }
    }

    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>) {
        let mut map = self.command_map.lock().unwrap();
        if !map.contains_key(notification_name) && let Some(view) = self.view.as_ref().unwrap().upgrade() {
            let controller = Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k)));

            let context: Arc<dyn Any + Send + Sync> = Arc::new(controller.clone());
            let weak = Arc::downgrade(&context); // Weak Controller to avoid reference cycle with the Observer.
            let notify = {
                let weak = weak.clone();
                Arc::new(move |notification: &Arc<dyn INotification>| {
                    if let Some(arc) = weak.upgrade() {
                        if let Some(controller) = arc.downcast_ref::<Arc<dyn IController>>() {
                            controller.execute_command(notification);
                        }
                    }
                })
            };

            let observer = Observer::new(Some(notify), Some(context));
            view.register_observer(notification_name, Arc::new(observer));
        }
        map.insert(notification_name.to_string(), factory);
    }

    fn has_command(&self, notification_name: &str) -> bool {
        self.command_map.lock().unwrap().contains_key(notification_name)
    }

    fn remove_command(&self, notification_name: &str) {
        let mut map = self.command_map.lock().unwrap();
        if map.remove(notification_name).is_some() && let Some(view) = self.view.as_ref().unwrap().upgrade(){
            let controller: Arc<dyn IController> = Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k)));
            let context: Arc<dyn Any + Send + Sync> = Arc::new(controller.clone());
            view.remove_observer(notification_name, context);
        }
    }
}
