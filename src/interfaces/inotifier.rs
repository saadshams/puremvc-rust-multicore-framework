use std::any::Any;
use std::sync::{Arc, Weak};
use crate::interfaces::IFacade;

pub trait INotifier: Any + Send + Sync {
    fn facade(&self) -> Option<Weak<dyn IFacade>> {
        None
    }

    fn initialize_notifier(&mut self, _key: &str) {

    }

    fn send_notification(&self, _notification_name: &str, _body: Option<Arc<dyn Any+ Send + Sync>>, _type_: Option<&str>) {

    }
}
