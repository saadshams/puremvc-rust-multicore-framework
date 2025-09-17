use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::IFacade;

pub trait INotifier: Any + Send + Sync {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        None
    }

    fn facade(&self) -> Option<Arc<dyn IFacade>> {
        None
    }

    fn initialize_notifier(&mut self, _key: &str) {

    }

    fn send_notification(&self, _notification_name: &str, _body: Option<Arc<dyn Any + Send + Sync>>, _type_: Option<&str>) {

    }
}
