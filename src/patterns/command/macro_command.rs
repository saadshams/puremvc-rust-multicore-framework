use std::sync::{Arc};
use crate::{ICommand, IMacroCommand, INotification, INotifier, SimpleCommand};

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
}

impl INotifier for MacroCommand {
    fn notifier(&mut self) -> &mut dyn INotifier {
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

impl IMacroCommand for MacroCommand {
    fn initialize_macro_command(&mut self) {

    }

    fn add_sub_command(&mut self, factory: fn() -> Box<dyn ICommand + Send + Sync>) {
        self.sub_commands.push(factory);
    }
}