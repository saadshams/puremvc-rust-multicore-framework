use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::core::{Controller, Model, View};
use crate::interfaces::{ICommand, IController, IFacade, IMediator, IModel, INotification, INotifier, IProxy, IView};
use crate::patterns::Notification;

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IFacade>>>> = LazyLock::new(|| Default::default());
// static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>> = LazyLock::new(|| Default::default());

pub struct Facade {
    key: String,
    controller: Arc<dyn IController>,
    model: Arc<dyn IModel>,
    view: Arc<dyn IView>
}

impl Facade {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            controller: Controller::get_instance(key, |k| Controller::new(k)),
            model: Model::get_instance(key, |k| Model::new(k)),
            view: View::get_instance(key, |v| View::new(v))
        }
    }

    pub fn get_instance<T: IFacade>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IFacade> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.to_string())
            .or_insert_with(|| {
                let mut instance = factory(key);
                instance.initialize_facade();
                Arc::new(instance)
            })
            .clone()
    }

    pub fn has_core(key: &str) -> bool {
        INSTANCE_MAP.lock().unwrap().contains_key(key)
    }

    pub fn remove_core(key: &str) {
        Model::remove_model(key);
        View::remove_view(key);
        Controller::remove_controller(key);
        INSTANCE_MAP.lock().unwrap().remove(key);
    }
}

impl IFacade for Facade {
    fn initialize_facade(&mut self) {
        self.initialize_model();
        self.initialize_controller();
        self.initialize_view();
    }

    fn initialize_controller(&mut self) {
        // self.controller = Some(Controller::get_instance(&self.key, |k| Controller::new(k)))
    }

    fn initialize_model(&mut self) {
        // self.model = Some(Model::get_instance(&self.key, |k| Model::new(k)))
    }

    fn initialize_view(&mut self) {
        // self.view = Some(View::get_instance(&self.key, |k| View::new(k)))
    }

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.controller.register_command(notification_name, factory);
        // self.controller.as_ref().unwrap().register_command(notification_name, factory);
    }

    fn has_command(&self, notification_name: &str) -> bool {
        self.controller.has_command(notification_name)
        // self.controller.as_ref().unwrap().has_command(notification_name)
    }

    fn remove_command(&self, notification_name: &str) {
        self.controller.remove_command(notification_name);
        // self.controller.as_ref().unwrap().remove_command(notification_name);
    }

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) {
        self.model.register_proxy(proxy);
        // self.model.as_ref().unwrap().register_proxy(proxy);
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        self.model.retrieve_proxy(proxy_name)
        // self.model.as_ref().unwrap().retrieve_proxy(proxy_name)
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.model.has_proxy(proxy_name)
        // self.model.as_ref().unwrap().has_proxy(proxy_name)
    }

    fn remove_proxy(&self, name: &str) -> Option<Arc<Mutex<dyn IProxy>>> {
        self.model.remove_proxy(name)
        // self.model.as_ref().unwrap().remove_proxy(name)
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        self.view.register_mediator(mediator);
        // self.view.as_ref().unwrap().register_mediator(mediator);
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.view.retrieve_mediator(mediator_name)
        // self.view.as_ref().unwrap().retrieve_mediator(mediator_name)
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.view.has_mediator(mediator_name)
        // self.view.as_ref().unwrap().has_mediator(mediator_name)
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.view.remove_mediator(mediator_name)
        // self.view.as_ref().unwrap().remove_mediator(mediator_name)
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        self.view.notify_observers(notification);
        // self.view.as_ref().unwrap().notify_observers(notification);
    }
}

impl INotifier for Facade {
    // fn notifier(&mut self) -> Option<&mut dyn INotifier> {
    //     None
    //     // self as &mut dyn INotifier
    // }

    fn initialize_notifier(&mut self, key: &str) {
        self.key = key.to_string();
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        let notification = Notification::new(notification_name, body, type_);
        self.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    }
}
