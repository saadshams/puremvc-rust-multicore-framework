use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};
use crate::core::{Controller, Model, View};
use crate::interfaces::{ICommand, IController, IFacade, IMediator, IModel, INotification, INotifier, IProxy, IView};
use crate::patterns::Notification;

static INSTANCE_MAP: LazyLock<RwLock<HashMap<String, Arc<dyn IFacade>>>> = LazyLock::new(|| Default::default());

pub struct Facade {
    key: String,
    controller: Arc<dyn IController>,
    model: Arc<dyn IModel>,
    view: Arc<dyn IView>
}

impl Facade {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            controller: Controller::get_instance(key, |k| Controller::new(k)),
            model: Model::get_instance(key, |k| Model::new(k)),
            view: View::get_instance(key, |k| View::new(k))
        }
    }

    pub fn get_instance<T: IFacade>(key: &str, factory: impl Fn(&str) -> T) -> Arc<dyn IFacade> {
        INSTANCE_MAP.write().unwrap()
            .entry(key.into())
            .or_insert_with(|| {
                let instance = factory(key);
                instance.initialize_facade();
                Arc::new(instance)
            })
            .clone()
    }

    pub fn has_core(key: &str) -> bool {
        INSTANCE_MAP.read().unwrap().contains_key(key)
    }

    pub fn remove_core(key: &str) {
        Model::remove_model(key);
        View::remove_view(key);
        Controller::remove_controller(key);
        INSTANCE_MAP.write().unwrap().remove(key);
    }
}

impl IFacade for Facade {
    fn initialize_facade(&self) {
        self.initialize_model();
        self.initialize_controller();
        self.initialize_view();
    }

    fn initialize_controller(&self) {

    }

    fn initialize_model(&self) {

    }

    fn initialize_view(&self) {

    }

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.controller.register_command(notification_name, factory);
    }

    fn has_command(&self, notification_name: &str) -> bool {
        self.controller.has_command(notification_name)
    }

    fn remove_command(&self, notification_name: &str) {
        self.controller.remove_command(notification_name);
    }

    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>) {
        self.model.register_proxy(proxy);
    }

    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        self.model.retrieve_proxy(proxy_name)
    }

    fn has_proxy(&self, proxy_name: &str) -> bool {
        self.model.has_proxy(proxy_name)
    }

    fn remove_proxy(&self, name: &str) -> Option<Arc<RwLock<dyn IProxy>>> {
        self.model.remove_proxy(name)
    }

    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>) {
        self.view.register_mediator(mediator);
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.view.retrieve_mediator(mediator_name)
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.view.has_mediator(mediator_name)
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.view.remove_mediator(mediator_name)
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        self.view.notify_observers(notification);
    }
}

impl INotifier for Facade {
    fn key(&self) -> &str {
        self.key.as_str()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        Facade::get_instance(&self.key, |k| Facade::new(k))
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.key = key.into();
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        let notification = Notification::new(notification_name, body, type_);
        self.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    }
}
