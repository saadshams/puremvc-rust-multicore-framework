use std::sync::{Arc, Mutex};
use crate::IProxy;

pub trait IModel: Sync + Send + 'static {
    fn key(&self) -> &str;
    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy + Send>>);
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy + Send>>>;
    fn has_proxy(&self, name: &str) -> bool;
    fn remove_proxy(&self, name: &str) -> Option<Arc<Mutex<dyn IProxy + Send>>>;
}
