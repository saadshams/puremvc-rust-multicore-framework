use std::sync::{Arc};
use crate::interfaces::{ICommand, INotification, INotifier};
use crate::patterns::SimpleCommand;

pub struct MacroCommand {
    command: SimpleCommand,
    sub_commands: Vec<fn() -> Box<dyn ICommand + Send + Sync>>
}

impl MacroCommand {
    pub fn new() -> Self {
        Self {
            command: SimpleCommand::new(),
            sub_commands: Vec::new()
        }
    }

    pub fn initialize_macro_command(&mut self) {

    }

    pub fn add_sub_command(&mut self, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.sub_commands.push(factory);
    }
}

impl INotifier for MacroCommand {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        self.command.notifier()
    }
}

impl ICommand for MacroCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        for factory in self.sub_commands.drain(..) {
            let mut command = factory();
            command.execute(&notification);
        }
    }
}
