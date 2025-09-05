use std::sync::{Arc, Mutex};
use crate::{ICommand, INotification};

pub struct MacroCommand {
    sub_commands: Vec<Box<dyn Fn() -> Box<dyn ICommand> + Send + Sync>>,
}

impl MacroCommand {
    pub fn new() -> Self {
        Self {
            sub_commands: Vec::new()
        }
    }
    
    pub fn initialize_macro_command(&mut self) {
        
    }

    pub fn add_sub_command(&mut self, factory: impl Fn() -> Box<dyn ICommand> + Send + Sync + 'static) {
        self.sub_commands.push(Box::new(factory));
    }
}

impl ICommand for MacroCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        for factory in self.sub_commands.drain(..) {
            let mut command = factory();
            command.execute(&notification.clone());
        }
    }
}
