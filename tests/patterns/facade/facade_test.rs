use std::sync::{Arc, Mutex};
use puremvc::{Facade, ICommand, INotification, INotifier, Proxy, SimpleCommand};

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
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        let body = notification.body().cloned().unwrap();
        let mut guard = body.lock().unwrap();
        let vo = guard.downcast_mut::<FacadeTestVO>().unwrap();

        vo.result = 2 * vo.input;
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

    let vo = Arc::new(Mutex::new(FacadeTestVO{input: 32, result: 0}));
    facade.send_notification("FacadeTestNote", Some(vo.clone()), None);

    assert_eq!(vo.lock().unwrap().result, 64);
}

#[test]
fn test_register_and_remove_command_and_send_notification() {
    let facade = Facade::get_instance("FacadeTestKey3", |k| Arc::new(Facade::new(k)));
    facade.register_command( "FacadeTestNote", Arc::new(|| Box::new(FacadeTestCommand::new())));
    facade.remove_command("FacadeTestNote");

    let vo = Arc::new(Mutex::new(FacadeTestVO{input: 32, result: 0}));
    facade.send_notification("FacadeTestNote", Some(vo.clone()), None);

    assert_ne!(vo.lock().unwrap().result, 64);
}

#[test]
fn test_register_and_retrieve_proxy() {
    let facade = Facade::get_instance("FacadeTestKey4", |k| Arc::new(Facade::new(k)));
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors"), Some(Box::new(colors)));
    facade.register_proxy(Arc::new(Mutex::new(proxy)));

    let proxy = facade.retrieve_proxy("colors").unwrap();
}
