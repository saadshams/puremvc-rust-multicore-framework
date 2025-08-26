use std::sync::Arc;
use crate::IMediator;

pub trait IView: Sync + Send + 'static {
    fn key(&self) -> &str;
    fn register_mediator(&self, mediator: Arc<dyn IMediator>);
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<dyn IMediator>>;
    fn has_mediator(&self, mediator_name: &str) -> bool;
    // fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<dyn IMediator>>;
}
