use std::sync::{Arc, Mutex};
use puremvc::interfaces::{ICommand, INotification, INotifier};
use puremvc::patterns::{Notification, SimpleCommand};

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

impl INotifier for SimpleCommandTestCommand {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        self.command.notifier()
    }
}

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<Mutex<SimpleCommandTestVO>>())
            .and_then(|mutex| mutex.lock().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
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
