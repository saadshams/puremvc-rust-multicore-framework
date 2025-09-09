use std::any::Any;
use std::sync::{Arc};
use crate::{INotification, INotifier, Notifier};
use crate::interfaces::ICommand;

pub struct SimpleCommand {
    notifier: Box<dyn INotifier + Send + Sync>,
}

impl SimpleCommand {
    pub fn new() -> Self {
        Self {
            notifier: Box::new(Notifier::new())
        }
    }
}

impl dyn ICommand {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl INotifier for SimpleCommand {

}

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {

    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        &mut self.notifier
    }
}
