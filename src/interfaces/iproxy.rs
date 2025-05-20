use std::any::Any;

pub trait IProxy {
    fn get_proxy_name(&self) -> &str;
    fn get_data(&self) -> Option<&dyn Any>;
    fn set_data(&mut self, data: Option<Box<dyn Any>>);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}
