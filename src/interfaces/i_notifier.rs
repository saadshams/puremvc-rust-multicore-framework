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

    fn initialize_notifier(&mut self, key: &str) {
        let _ = key;
    }

    fn send_notification(&self, notification_name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        let _ = notification_name; let _ = body; let _ = type_;
    }
}
