use std::sync::{Arc};
use crate::interfaces::{INotification, INotifier};

pub trait ICommand: INotifier {
    fn execute(&mut self, notification: &Arc<dyn INotification>);
}
