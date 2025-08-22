use puremvc::{ICommand, INotification, Notification, SimpleCommand};

#[test]
fn test_simple_command_execute() {
    let vo = SimpleCommandTestVO::new(5);

    let mut note = Notification::new("SimpleCommandTestNote".to_string(), Some(Box::new(vo)), None);

    let mut command = SimpleCommandTestCommand::new();
    command.execute(&mut note);

    let body = note.body().unwrap();
    let vo = body.downcast_ref::<SimpleCommandTestVO>().unwrap();

    assert_eq!(vo.result, 10, "Expecting vo.result == 10");
}

pub struct SimpleCommandTestCommand(SimpleCommand);

impl SimpleCommandTestCommand {
    pub fn new() -> Self {
        Self(SimpleCommand)
    }
}

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: &mut dyn INotification) {
        if let Some(body) = notification.body() {
            if let Some(vo) = body.downcast_mut::<SimpleCommandTestVO>() {
                vo.result = 2 * vo.input;
            }
        }
    }
}

pub struct SimpleCommandTestVO {
    input: i8,
    result: i8,
}

impl SimpleCommandTestVO {
    pub fn new(input: i8) -> Self {
        Self { input, result: 0 }
    }
}
