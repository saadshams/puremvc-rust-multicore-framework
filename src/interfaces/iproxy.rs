use std::any::Any;

pub trait IProxy: Send + Sync {
    fn name(&self) -> &str;

    fn data(&self) -> Option<&(dyn Any + Send + Sync)>;
    fn data_mut(&mut self) -> Option<&mut (dyn Any + Send + Sync)>;
    fn set_data(&mut self, data: Option<Box<dyn Any + Send + Sync>>);

    fn on_register(&mut self);
    fn on_remove(&mut self);
}
