use std::any::Any;

pub trait IMediator {
    fn get_mediator_name(&self) -> &str;
    fn get_view_component(&self) -> Option<&dyn Any>;
    fn set_view(&mut self, view: Option<Box<dyn Any>>);
    fn on_register(&mut self);
    fn on_remove(&mut self);
}