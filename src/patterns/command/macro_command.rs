use std::sync::{Arc, Mutex};
use crate::{ICommand, INotification, INotifier, Notifier};

pub struct MacroCommand {
    notifier: Box<dyn INotifier + Send + Sync>,
    sub_commands: Vec<Box<dyn Fn() -> Box<dyn ICommand> + Send + Sync>>,

}

impl MacroCommand {
    pub fn new() -> Self {
        Self {
            notifier: Box::new(Notifier::new()),
            sub_commands: Vec::new()
        }
    }
    
    pub fn initialize_macro_command(&mut self) {
        
    }

    pub fn add_sub_command(&mut self, factory: impl Fn() -> Box<dyn ICommand> + Send + Sync + 'static) {
        self.sub_commands.push(Box::new(factory));
    }
}

impl INotifier for MacroCommand {

}

impl ICommand for MacroCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        for factory in self.sub_commands.drain(..) {
            let mut command = factory();
            command.execute(&notification);
        }
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        &mut self.notifier
    }
}
