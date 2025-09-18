use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::IFacade;

pub trait INotifier: Any + Send + Sync {
    fn key(&self) -> &str {
        ""
    }

    fn facade(&self) -> Arc<dyn IFacade>;

    fn initialize_notifier(&mut self, key: &str);

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>);
}
