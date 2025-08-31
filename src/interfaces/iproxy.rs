use std::any::Any;
use std::sync::Arc;

pub trait IProxy: Sync + Send + 'static {
    fn name(&self) -> &str;
    fn data(&mut self) -> Option<Arc<dyn Any + Sync + Send>>;
    fn set_data(&mut self, data: Option<Arc<dyn Any + Sync + Send>>);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
