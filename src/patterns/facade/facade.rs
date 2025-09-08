use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{Controller, ICommand, IController, IMediator, IModel, INotification, IProxy, Model, Notification, View};
use crate::interfaces::{IFacade, INotifier, IView};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IFacade>>>> = LazyLock::new(|| Default::default());

pub struct Facade {
    key: String,
    controller: Option<Arc<dyn IController>>,
    model: Option<Arc<dyn IModel>>,
    view: Option<Arc<dyn IView>>
}

impl Facade {
    pub fn new(key: &str) -> Self {
        let mut instance = Self {
            key: key.to_string(),
            controller: None,
            model: None,
            view: None
        };

        instance.initialize_notifier(key);
        instance.initialize_facade();
        instance
    }

    pub fn get_instance(key: &str, factory: impl FnOnce(&str) -> Arc<dyn IFacade>) -> Arc<dyn IFacade> {
        let mut map = INSTANCE_MAP.lock().unwrap();
        map.entry(key.to_string()).or_insert_with(|| factory(key)).clone()
    }

    fn initialize_facade(&mut self) {
        self.initialize_model();
        self.initialize_controller();
        self.initialize_view();
    }

    fn initialize_controller(&mut self) {
        self.controller = Some(Controller::get_instance(&self.key, |k| Arc::new(Controller::new(k))))
    }

    fn initialize_model(&mut self) {
        self.model = Some(Model::get_instance(&self.key, |k| Arc::new(Model::new(k))))
    }

    fn initialize_view(&mut self) {
        self.view = Some(View::get_instance(&self.key, |k| Arc::new(View::new(k))))
    }
}

impl IFacade for Facade {
    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>) {
        if let Some(controller) = &self.controller {
            controller.register_command(notification_name, factory);
        }
    }

    fn has_command(&self, notification_name: &str) -> bool {
        if let Some(controller) = &self.controller {
            controller.has_command(notification_name)
        } else {
            false
        }
    }

    fn remove_command(&self, notification_name: &str) {
        if let Some(controller) = &self.controller {
            controller.remove_command(notification_name);
        }
    }

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) {
        if let Some(model) = &self.model {
            model.register_proxy(proxy.clone());
        }
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        if let Some(model) = &self.model {
            model.retrieve_proxy(proxy_name)
        } else {
            None
        }
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        if let Some(model) = &self.model {
            model.has_proxy(proxy_name)
        } else {
            false
        }
    }

    fn remove_proxy(&self, name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        if let Some(model) = &self.model {
            model.remove_proxy(name)
        } else {
            None
        }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        if let Some(view) = &self.view {
            view.register_mediator(mediator);
        }
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        if let Some(view) = &self.view {
            view.retrieve_mediator(mediator_name)
        } else {
            None
        }
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        if let Some(view) = &self.view {
            view.has_mediator(mediator_name)
        } else {
            false
        }
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        if let Some(view) = &self.view {
            view.remove_mediator(mediator_name)
        } else {
            None
        }
    }

    fn notify_observers(&self, notification: &Arc<Mutex<dyn INotification>>) {
        if let Some(view) = &self.view {
            view.notify_observers(notification);
        }
    }
}

impl INotifier for Facade {
    fn initialize_notifier(&mut self, key: &str) {
        self.key = key.to_string();
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<Mutex<dyn Any+ Send + Sync>>>, type_: Option<&str>) {
        let notification: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new(notification_name, body, type_)));
        self.notify_observers(&notification);
    }
}
