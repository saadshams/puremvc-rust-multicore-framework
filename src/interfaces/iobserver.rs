use std::any::Any;
use std::sync::Arc;
use crate::INotification;

pub trait IObserver: Send + Sync {
    fn notify(&self) -> Option<Arc<dyn Fn(&mut dyn INotification) + Send + Sync>>;

    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&mut dyn INotification) + Send + Sync>>);

    fn context(&self) -> Option<Arc<Box<dyn Any + Send + Sync>>>;

    fn set_context(&mut self, context: Option<Arc<Box<dyn Any + Send + Sync>>>);

    fn notify_observer(&self, notification: &mut dyn INotification);

    fn compare_notify_context(&self, object: &Arc<Box<dyn Any + Send + Sync>>) -> bool;
}
