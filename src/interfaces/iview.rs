use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::{IMediator, INotification, IObserver};

pub trait IView: Any + Sync + Send {
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>);
    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>);
    fn notify_observers(&self, notification: &Arc<dyn INotification>);
    
    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>);
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>>;
    fn has_mediator(&self, mediator_name: &str) -> bool;
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>>;
}
