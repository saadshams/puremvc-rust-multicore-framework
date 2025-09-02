use std::any::Any;
use std::sync::{Arc};
use crate::{IMediator, INotification, IObserver};

pub trait IView: Sync + Send + 'static {
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver + Send + Sync>);
    fn remove_observer(&self, notification_name: &str, context: &Arc<dyn Any + Send + Sync>);
    fn notify_observers(&self, notification: &mut dyn INotification);

    fn register_mediator(&self, mediator: Arc<dyn IMediator + Send + Sync>);
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<dyn IMediator + Send + Sync>>;
    fn has_mediator(&self, mediator_name: &str) -> bool;
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<dyn IMediator + Send + Sync>>;
}
