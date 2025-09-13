use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::interfaces::IProxy;

pub trait IModel: Any + Sync + Send {
    fn initialize_model(&mut self);

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>);
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>>;
    fn has_proxy(&self, proxy_name: &str) -> bool;
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>>;
}
