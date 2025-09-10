use std::any::Any;
use std::sync::{Arc, Weak};
use crate::{INotifier};
use crate::interfaces::IFacade;
use crate::patterns::Facade;

const MULTITON_MSG: &str = "multitonKey for this Notifier not yet initialized!";

pub struct Notifier {
    key: Option<String>
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            key: None
        }
    }
}

impl dyn INotifier {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl INotifier for Notifier {
    fn facade(&self) -> Option<Weak<dyn IFacade>> {
        let key = self.key.as_ref().expect(MULTITON_MSG);
        Some(Arc::downgrade(&Facade::get_instance(key, |k| Arc::new(Facade::new(k)))))
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.key = Some(key.to_string());
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        if let Some(facade) = self.facade().unwrap().upgrade() {
            facade.send_notification(notification_name, body, type_);
        }
    }
}
