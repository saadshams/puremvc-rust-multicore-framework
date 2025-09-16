use std::any::Any;
use std::sync::Arc;
use crate::interfaces::{INotifier, IProxy};
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
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        Some(self.notifier.as_mut())
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
