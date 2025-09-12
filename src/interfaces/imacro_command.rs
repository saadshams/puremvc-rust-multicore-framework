use crate::ICommand;

pub trait IMacroCommand: ICommand {
    fn initialize_macro_command(&mut self);

    fn add_sub_command(&mut self, _factory: fn() -> Box<dyn ICommand + Send + Sync>) {

    }
}
