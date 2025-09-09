use std::sync::{Arc, Mutex};
use puremvc::{Facade, ICommand, INotification, INotifier, Notifier, SimpleCommand};

struct NotifierTestVO {
    input: i8,
    result: i8
}

struct NotifierTestCommand {
    command: SimpleCommand
}

impl NotifierTestCommand {
    fn new() -> NotifierTestCommand {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for NotifierTestCommand {}

impl ICommand for NotifierTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        let body = notification.body().cloned().unwrap();
        let mut guard = body.lock().unwrap();
        let vo = guard.downcast_mut::<NotifierTestVO>().unwrap();

        vo.result = 2 * vo.input;
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier()
    }
}

#[test]
fn test_notifier() {
    let facade = Facade::get_instance("NotifierTestKey1", |k| Arc::new(Facade::new(k)));

    let vo = Arc::new(Mutex::new(NotifierTestVO{ input: 5, result: 0 }));
    facade.register_command("NotifierTestNote", Arc::new(|| Box::new(NotifierTestCommand::new())));

    let mut notifier = Notifier::new();
    notifier.initialize_notifier("NotifierTestKey1");
    notifier.send_notification("NotifierTestNote", Some(vo.clone()), None);

    assert_eq!(vo.lock().unwrap().result, 10);
}
