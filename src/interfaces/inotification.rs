use std::any::Any;

pub trait INotification {
    fn name(&self) -> &str;

    fn body(&mut self) -> Option<&mut Box<dyn Any>>;
    fn set_body(&mut self, body: Option<Box<dyn Any>>);

    fn get_type(&self) -> Option<&str>;
    fn set_type(&mut self, type_: Option<String>);

    fn to_string(&self) -> String;
}
