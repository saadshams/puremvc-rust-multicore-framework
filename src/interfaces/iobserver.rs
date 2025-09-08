use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::INotification;

pub trait IObserver: Send + Sync {
    fn notify(&self) -> Option<Arc<dyn Fn(&Arc<Mutex<dyn INotification>>) + Send + Sync>>;

    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&Arc<Mutex<dyn INotification>>) + Send + Sync>>);

    fn context(&self) -> Option<Arc<dyn Any + Send + Sync>>;

    fn set_context(&mut self, context: Option<Arc<dyn Any + Send + Sync>>);

    fn notify_observer(&self, notification: &Arc<Mutex<dyn INotification>>);

    fn compare_notify_context(&self, object: &Arc<dyn Any + Send + Sync>) -> bool;
}
