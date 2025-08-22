use std::any::Any;

pub trait IProxy: Sync + Send + 'static {
    fn name(&self) -> &str;
    fn data(&self) -> Option<&(dyn Any + Sync + Send)>;
    fn data_mut(&mut self) -> Option<&mut (dyn Any + Sync + Send)>;
    fn set_data(&mut self, data: Option<Box<dyn Any + Sync + Send>>);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
