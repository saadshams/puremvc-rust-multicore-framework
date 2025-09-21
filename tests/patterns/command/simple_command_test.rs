use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
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
    fn key(&self) -> &str {
        self.command.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<SimpleCommandTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
    }
}

#[test]
fn test_simple_command_execute() {
    let vo = Arc::new(RwLock::new(SimpleCommandTestVO { input: 5, result: 0 }));
    let note = Arc::new(Notification::new("SimpleCommandTestNote", Some(vo.clone()), None));

    let mut command = SimpleCommandTestCommand::new();
    command.execute(&(note as Arc<dyn INotification>));

    assert_eq!(vo.read().unwrap().result, 10, "Expecting vo.result == 10");
}
