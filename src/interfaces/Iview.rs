use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::{IMediator, INotification, IObserver};

pub trait IView: Sync + Send + 'static {
    fn register_observer(&self, observer: Arc<Mutex<dyn IObserver + Send>>);
    fn remove_observer(&self, context: Option<Arc<dyn Any + Send + Sync>>);
    fn notify_observers(&self, notification: Arc<dyn INotification>);

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator + Send>>);
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator + Send>>>;
    fn has_mediator(&self, mediator_name: &str) -> bool;
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator + Send>>>;
}
