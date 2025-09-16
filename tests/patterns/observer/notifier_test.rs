use std::sync::{Arc, Mutex};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{Facade, Notifier, SimpleCommand};

struct NotifierTestVO {
    input: i8,
    result: i8
}

struct NotifierTestCommand {
    command: SimpleCommand
}

impl NotifierTestCommand {
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for NotifierTestCommand {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        self.command.notifier()
    }
}

impl ICommand for NotifierTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(body) = notification.body() {
            let mut vo = body.downcast_ref::<Mutex<NotifierTestVO>>().unwrap().lock().unwrap();
            vo.result = 2 * vo.input;
        }
    }
}

#[test]
fn test_notifier() {
    // let facade = Facade::get_instance("NotifierTestKey1", |k| Facade::new(k));

    let arc= Facade::get_instance("NotifierTestKey1", |k| Facade::new(k));
    {
        let facade = arc.lock().unwrap(); // notifier.send_notification will re-lock facade
        facade.register_command("NotifierTestNote", || Box::new(NotifierTestCommand::new()));
    }

    let vo = Arc::new(Mutex::new(NotifierTestVO{ input: 5, result: 0 }));
    // facade.register_command("NotifierTestNote", || Box::new(NotifierTestCommand::new()));

    let mut notifier = Notifier::new();
    notifier.initialize_notifier("NotifierTestKey1");
    notifier.send_notification("NotifierTestNote", Some(vo.clone()), None);

    assert_eq!(vo.lock().unwrap().result, 10);
}
