use std::sync::{Arc, Mutex};
use crate::{ICommand, IMediator, INotification, IProxy};
use crate::interfaces::INotifier;

pub trait IFacade: INotifier + Sync + Send {
    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>);
    fn has_command(&self, notification_name: &str) -> bool;
    fn remove_command(&self, notification_name: &str);

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>);
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>>;
    fn has_proxy(&self, proxy_name: &str) -> bool;
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>>;

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>);
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>>;
    fn has_mediator(&self, mediator_name: &str) -> bool;
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>>;

    fn notify_observers(&self, notification: &Arc<Mutex<dyn INotification>>);
}
