use std::any::Any;
use std::sync::{Arc, Weak};
use crate::interfaces::{IFacade, INotifier};
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

impl INotifier for Notifier {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        Some(self)
    }

    // fn notifier(&mut self) -> &mut dyn INotifier {
    //     self as &mut dyn INotifier
    // }

    fn facade(&self) -> Option<Weak<dyn IFacade>> {
        let key = self.key.as_ref().expect(MULTITON_MSG);
        let arc_facade: Arc<dyn IFacade> = Facade::get_instance(key, |k| Facade::new(k));
        Some(Arc::downgrade(&arc_facade))
    }

    // fn facade(&self) -> Option<Weak<dyn IFacade>> {
    //     let key = self.key.as_ref().expect(MULTITON_MSG);
    //     let arc_facade: Arc<dyn IFacade> = Facade::get_instance(key, |k| Facade::new(k));
    //     Some(Arc::downgrade(&(arc_facade)))
    // }

    fn initialize_notifier(&mut self, key: &str) {
        self.key = Some(key.to_string());
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        if let Some(facade) = self.facade().unwrap().upgrade() {
            facade.send_notification(notification_name, body, type_);
        }
    }
}
