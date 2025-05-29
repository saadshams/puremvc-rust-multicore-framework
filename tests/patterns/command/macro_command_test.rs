use puremvc::{ICommand, INotification, MacroCommand, Notification, SimpleCommand};

#[test]
fn test_macro_command_execute() {
    let vo = MacroCommandTestVO::new(5);

    let mut note = Notification::new("MacroCommandTest".to_string(), Some(Box::new(vo)), None);

    let mut command = MacroCommandTestCommand::new();
    command.execute(&mut note);

    let vo = note.get_body().unwrap().downcast_ref::<MacroCommandTestVO>().unwrap();
    assert_eq!(vo.result1, 10, "Expecting vo.result == 10");
    assert_eq!(vo.result2, 25, "Expecting vo.result == 25");
}

pub struct MacroCommandTestCommand(MacroCommand);

impl MacroCommandTestCommand {
    pub fn new() -> Self {
        Self(MacroCommand::new())
    }

    pub fn initialize_macro_command(&mut self) {
        self.0.add_sub_command(|| Box::new(MacroCommandTestSub1Command::new()));
        self.0.add_sub_command(|| Box::new(MacroCommandTestSub2Command::new()));
    }
}

impl ICommand for MacroCommandTestCommand {
    fn execute(&mut self, notification: &mut dyn INotification) {
        self.initialize_macro_command();
        self.0.execute(notification);
    }
}

pub struct MacroCommandTestSub1Command(SimpleCommand);

impl MacroCommandTestSub1Command {
    pub fn new() -> Self {
        Self(SimpleCommand{})
    }
}

impl ICommand for MacroCommandTestSub1Command {
    fn execute(&mut self, notification: &mut dyn INotification) {
        if let Some(body) = notification.get_body() {
            if let Some(vo) = body.downcast_mut::<MacroCommandTestVO>() {
                vo.result1 = 2 * vo.input;
            }
        }
    }
}

pub struct MacroCommandTestSub2Command(SimpleCommand);

impl MacroCommandTestSub2Command {
    pub fn new() -> Self {
        Self(SimpleCommand{})
    }
}

impl ICommand for MacroCommandTestSub2Command {
    fn execute(&mut self, notification: &mut dyn INotification) {
        if let Some(body) = notification.get_body() {
            if let Some(vo) = body.downcast_mut::<MacroCommandTestVO>() {
                vo.result2 = vo.input * vo.input;
            }
        }
    }
}

pub struct MacroCommandTestVO {
    input: i8,
    result1: i8,
    result2: i8
}

impl MacroCommandTestVO {
    pub fn new(input: i8) -> Self {
        Self { input, result1: 0, result2: 0 }
    }
}
