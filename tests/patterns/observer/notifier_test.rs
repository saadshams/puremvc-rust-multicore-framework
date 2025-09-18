use std::any::Any;
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

impl ICommand for NotifierTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|arc| arc.downcast_ref::<Mutex<NotifierTestVO>>())
            .and_then(|mutex| mutex.lock().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
    }
}

#[test]
fn test_notifier() {
    let facade= Facade::get_instance("NotifierTestKey1", |k| Facade::new(k));
    facade.register_command("NotifierTestNote", || Box::new(NotifierTestCommand::new()));

    let vo = Arc::new(Mutex::new(NotifierTestVO{ input: 5, result: 0 }));

    let mut notifier = Notifier::new();
    notifier.initialize_notifier("NotifierTestKey1");
    notifier.send_notification("NotifierTestNote", Some(vo.clone()), None);

    assert_eq!(vo.lock().unwrap().result, 10);
}
