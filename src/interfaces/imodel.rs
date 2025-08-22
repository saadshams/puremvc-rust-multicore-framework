use std::sync::{Arc};
use crate::IProxy;

pub trait IModel: Sync + Send + 'static {
    fn key(&self) -> &str;
    fn register_proxy(&self, proxy: Arc<dyn IProxy>);
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<dyn IProxy>>;
    fn has_proxy(&self, name: &str) -> bool;
    fn remove_proxy(&self, name: &str) -> Option<Arc<dyn IProxy>>;
}
