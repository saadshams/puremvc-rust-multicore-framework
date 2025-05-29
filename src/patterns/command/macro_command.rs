use crate::{ICommand, INotification};

pub struct MacroCommand {
    sub_commands: Vec<Box<dyn Fn() -> Box<dyn ICommand>>>
}

impl MacroCommand {
    pub fn new() -> Self {
        Self { sub_commands: Vec::new() }
    }
    
    fn initialize_macro_command(&mut self) {
        
    }

    pub fn add_sub_command(&mut self, factory: impl Fn() -> Box<dyn ICommand> + 'static) {
        self.sub_commands.push(Box::new(factory));
    }

    pub fn execute(&mut self, notification: &mut dyn INotification) {
        self.initialize_macro_command();
        while !self.sub_commands.is_empty() {
            let factory = self.sub_commands.remove(0); // FIFO
            let mut command = factory();
            // command.initialize_notifier(&self.multiton_key);
            command.execute(notification);
        }
    }
}
