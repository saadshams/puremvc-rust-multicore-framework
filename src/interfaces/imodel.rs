use std::sync::{Arc, Mutex};
use crate::{IProxy};

pub trait IModel: Sync + Send {
    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>);
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>>;
    fn has_proxy(&self, proxy_name: &str) -> bool;
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>>;
}
