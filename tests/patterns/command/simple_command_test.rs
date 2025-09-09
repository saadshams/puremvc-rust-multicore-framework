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
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        let body = notification.body().cloned().unwrap();

        let mut guard = body.lock().unwrap();
        let vo = guard.downcast_mut::<SimpleCommandTestVO>().unwrap();

        vo.result = 2 * vo.input;
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier()
    }
}

#[test]
fn test_simple_command_execute() {
    let vo = Arc::new(Mutex::new(SimpleCommandTestVO { input: 5, result: 0 }));
    let note = Arc::new(Notification::new("SimpleCommandTestNote", Some(vo.clone()), None));

    let mut command = SimpleCommandTestCommand::new();
    command.execute(&(note as Arc<dyn INotification>));

    assert_eq!(vo.lock().unwrap().result, 10, "Expecting vo.result == 10");
}
