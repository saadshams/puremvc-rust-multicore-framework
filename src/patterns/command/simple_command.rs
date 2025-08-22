use crate::INotification;
use crate::interfaces::ICommand;

pub struct SimpleCommand;

impl SimpleCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICommand for SimpleCommand {
    fn execute(&mut self, _notification: &mut dyn INotification) {

    }
}
