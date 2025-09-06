use std::sync::{Arc, Mutex};
use crate::{INotification, INotifier, Notifier};
use crate::interfaces::ICommand;

pub struct SimpleCommand {
    notifier: Box<dyn INotifier + Send + Sync>,
}

impl SimpleCommand {
    pub fn new() -> Self {
        Self { notifier: Box::new(Notifier::new()) }
    }
}

impl Default for SimpleCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl INotifier for SimpleCommand {}

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: &Arc<Mutex<dyn INotification>>) {

    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        &mut self.notifier
    }
}
