use std::any::Any;
use std::rc::{Rc};
use crate::INotification;

pub trait IObserver {
    fn notify(&self) -> Option<Rc<dyn Fn(&mut dyn INotification)>>;

    fn set_notify(&mut self, notify: Option<Rc<dyn Fn(&mut dyn INotification)>>);

    fn context(&self) -> Option<Rc<dyn Any>>;

    fn set_context(&mut self, context: Option<Rc<dyn Any>>);

    fn notify_observer(&self, notification: &mut dyn INotification);

    fn compare_notify_context(&self, object: &Rc<dyn Any>) -> bool;
}
