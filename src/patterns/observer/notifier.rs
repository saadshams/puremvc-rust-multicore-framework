use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{IFacade, INotifier};
use crate::patterns::Facade;

const MULTITON_MSG: &str = "multitonKey for this Notifier not yet initialized!";

pub struct Notifier {
    pub key: Option<String>
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            key: None
        }
    }
}

impl INotifier for Notifier {
    fn key(&self) -> &str {
        self.key.as_ref().map(String::as_str).unwrap_or("")
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        let key = self.key.as_ref().expect(MULTITON_MSG);
        Facade::get_instance(key, |k| Facade::new(k))
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.key = Some(key.to_string());
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.facade().send_notification(notification_name, body, type_);
    }
}
