use std::sync::{Arc, Mutex};
use crate::IMediator;

pub trait IView: Sync + Send + 'static {
    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator + Send>>);
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator + Send>>>;
    fn has_mediator(&self, mediator_name: &str) -> bool;
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator + Send>>>;
}
