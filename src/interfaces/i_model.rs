use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::interfaces::IProxy;

pub trait IModel: Any + Sync + Send {
    fn initialize_model(&self);

    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>);
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>>;
    fn has_proxy(&self, proxy_name: &str) -> bool;
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>>;
}
