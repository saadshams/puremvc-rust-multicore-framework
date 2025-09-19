use std::any::Any;
use std::sync::{Arc};
use crate::interfaces::{ICommand, IFacade, INotification, INotifier};
use crate::patterns::SimpleCommand;

pub struct MacroCommand {
    command: SimpleCommand,
    sub_commands: Vec<Box<dyn Fn() -> Box<dyn ICommand + Send + Sync> + Send + Sync>>,
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

    pub fn add_sub_command<T: ICommand + Send + Sync>(&mut self, factory: fn() -> T) {
        self.sub_commands.push(Box::new(move || Box::new(factory())));
    }
}

impl ICommand for MacroCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        for factory in self.sub_commands.drain(..) {
            let mut command = factory();
            command.initialize_notifier(self.command.key());
            command.execute(&notification);
        }
    }
}

impl INotifier for MacroCommand {
    fn key(&self) -> &str {
        self.command.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}
