use std::sync::{Arc, Mutex};
use puremvc::{Facade, ICommand, INotification, INotifier, Notification, SimpleCommand};

struct FacadeTestVO {
    input: i8,
    result: i8
}

struct FacadeTestCommand {
    command: SimpleCommand
}

impl FacadeTestCommand {
    fn new() -> Self {
        Self { command: SimpleCommand::new()}
    }
}

impl INotifier for FacadeTestCommand {}

impl ICommand for FacadeTestCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {

    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier()
    }
}

#[test]
fn test_get_instance() {
    let facade = Facade::get_instance("FacadeTestKey1", |k| Arc::new(Facade::new(k)));

    assert!(Arc::strong_count(&facade) > 0, "Expecting instance not null");
}

#[test]
fn test_register_command_and_send_notification() {
    let facade = Facade::get_instance("FacadeTestKey2", |k| Arc::new(Facade::new(k)));
    facade.register_command("FacadeTestNote", Arc::new(|| Box::new(FacadeTestCommand::new())));

    // let vo = Arc::new(Mutex::new(FacadeTestVO{input: 0, result: 0}));
    // facade.send_notification()
}
