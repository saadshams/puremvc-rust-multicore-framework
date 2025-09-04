use std::sync::{Arc, Mutex};
use crate::INotification;
use crate::interfaces::ICommand;

pub struct SimpleCommand;

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: Arc<Mutex<dyn INotification>>) {

    }
}
