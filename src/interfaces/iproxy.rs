use std::any::Any;

pub trait IProxy: Sync + Send + 'static {
    fn get_proxy_name(&self) -> &str;
    fn get_data(&self) -> Option<&(dyn Any + Sync + Send)>;
    fn set_data(&mut self, data: Option<Box<dyn Any + Sync + Send>>);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
