use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, INotifier, Notification, SimpleCommand};

struct SimpleCommandTestVO {
    input: i8,
    result: i8,
}

struct SimpleCommandTestCommand {
    command: SimpleCommand
}

impl SimpleCommandTestCommand {
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for SimpleCommandTestCommand {}

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("No body in notification");
        
        let vo = body.downcast_mut::<SimpleCommandTestVO>()
            .expect("Body is not a SimpleCommandTestVO");

        vo.result = 2 * vo.input;
    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier_mut()
    }
}

#[test]
fn test_simple_command_execute() {
    let note: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new(
        "SimpleCommandTestNote",
        Some(Box::new(SimpleCommandTestVO{input: 5, result: 0})),
        None
    )));

    let mut command = SimpleCommandTestCommand::new();
    command.execute(&note);

    let note = note.lock().unwrap();
    let body = note.body().expect("No body in notification");

    let vo = body.downcast_ref::<SimpleCommandTestVO>()
        .expect("Body is not a SimpleCommandTestVO");

    assert_eq!(vo.result, 10, "Expecting vo.result == 10");
}
