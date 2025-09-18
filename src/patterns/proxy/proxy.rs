use std::any::Any;
use std::sync::Arc;
use crate::interfaces::{IFacade, INotifier, IProxy};
use crate::patterns::Notifier;

pub struct Proxy {
    notifier: Box<dyn INotifier + Send + Sync>,
    name: String,
    data: Option<Arc<dyn Any + Send + Sync>>
}

impl Proxy {
    pub const NAME: &'static str = "Proxy";

    pub fn new(name: Option<&str>, data: Option<Arc<dyn Any + Send + Sync>>) -> Self {
        Self {
            notifier: Box::new(Notifier::new()),
            name: name.unwrap_or(Self::NAME).to_string(),
            data
        }
    }
}

impl INotifier for Proxy {
    fn key(&self) -> &str {
        ""
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.notifier.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.notifier.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.notifier.send_notification(name, body, type_);
    }
}

impl IProxy for Proxy {
    fn name(&self) -> &str {
        &self.name
    }

    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.data.as_ref()
    }

    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        self.data = data;
    }

    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {

    }
}
