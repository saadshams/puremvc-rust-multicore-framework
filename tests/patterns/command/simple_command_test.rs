use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, Notification, SimpleCommand};

struct SimpleCommandTestVO {
    input: i8,
    result: i8,
}

impl SimpleCommandTestVO {
    pub fn new(input: i8) -> Self {
        Self { input, result: 0 }
    }
}

struct SimpleCommandTestCommand(SimpleCommand);

impl SimpleCommandTestCommand {
    pub fn new() -> Self {
        Self(SimpleCommand)
    }
}

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>) {
        if let Some(body) = notification.lock().unwrap().body() {
            if let Some(vo) = body.downcast_mut::<SimpleCommandTestVO>() {
                vo.result = 2 * vo.input;
            } else {
                panic!("Incorrect type for SimpleCommandTestVO");
            }
        } else {
            panic!("Incorrect type for body for SimpleCommandTestVO");
        }
    }
}

#[test]
fn test_simple_command_execute() {
    let note = Arc::new(Mutex::new(Notification::new(
        "SimpleCommandTestNote",
        Some(Box::new(SimpleCommandTestVO::new(5))),
        None
    )));

    let mut command = SimpleCommandTestCommand::new();
    command.execute(note.clone());

    let mut note_guard = note.lock().unwrap();
    let vo = note_guard.body().unwrap().downcast_ref::<SimpleCommandTestVO>().unwrap();

    assert_eq!(vo.result, 10, "Expecting vo.result == 10");
}
