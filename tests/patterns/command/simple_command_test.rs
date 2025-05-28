use puremvc::{ICommand, INotification, Notification, SimpleCommand};

#[test]
fn test_simple_command_execute() {
    let vo = SimpleCommandTestVO::new(5.0);

    let mut note = Notification::new("SimpleCommandTestNote".to_string(), Some(Box::new(vo)), None);

    let mut command = SimpleCommandTestCommand::new();
    command.execute(&mut note);

    let body = note.get_body().unwrap();
    let vo = body.downcast_ref::<SimpleCommandTestVO>().unwrap();

    assert_eq!(vo.result, 10.0, "Expecting vo.result == 10");
}

pub struct SimpleCommandTestCommand(SimpleCommand);

impl SimpleCommandTestCommand {
    pub fn new() -> Self {
        Self(SimpleCommand{})
    }
}

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: &mut dyn INotification) {
        if let Some(body) = notification.get_body() {
            if let Some(vo) = body.downcast_mut::<SimpleCommandTestVO>() {
                vo.result = 2.0 * vo.input;
            }
        }
    }
}

pub struct SimpleCommandTestVO {
    pub input: f64,
    pub result: f64,
}

impl SimpleCommandTestVO {
    pub fn new(input: f64) -> Self {
        Self { input, result: 0.0 }
    }
}
