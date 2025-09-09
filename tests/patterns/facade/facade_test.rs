use std::sync::{Arc, Mutex};
use puremvc::{Facade, ICommand, INotification, INotifier, Mediator, Proxy, SimpleCommand};

struct Sprite{}
impl Default for Sprite {
    fn default() -> Self { Self {} }
}

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

    let mut guard = proxy.lock().unwrap();
    let data = guard.data_mut()  // <-- a method returning &mut Box<dyn Any>
        .unwrap()
        .downcast_mut::<Vec<String>>()
        .unwrap();

    assert_eq!(data, &["red", "green", "blue"]);

    data.push("yellow".to_string());

    let data2 = guard.data_mut()  // <-- a method returning &mut Box<dyn Any>
        .unwrap()
        .downcast_mut::<Vec<String>>()
        .unwrap();

    assert_eq!(data2, &["red", "green", "blue", "yellow"]);
}

#[test]
fn test_register_and_remove_proxy() {
    let facade = Facade::get_instance("FacadeTestKey5", |k| Arc::new(Facade::new(k)));
    let sizes = vec![7, 13, 21];
    let proxy = Proxy::new(Some("sizes"), Some(Box::new(sizes)));
    facade.register_proxy(Arc::new(Mutex::new(proxy)));

    let removed_proxy = facade.remove_proxy("sizes").unwrap();

    assert_eq!(removed_proxy.lock().unwrap().name(), "sizes");

    assert!(facade.retrieve_proxy("sizes").is_none());
}

#[test]
fn test_register_retrieve_and_remove_mediator() {
    let facade = Facade::get_instance("FacadeTestKey6", |k| Arc::new(Facade::new(k)));
    let component = Arc::new(Mutex::new(Sprite::default()));
    let mediator = Mediator::new(Some(Mediator::NAME), Some(Arc::downgrade(&component).clone()));

    facade.register_mediator(Arc::new(Mutex::new(mediator)));

    assert!(facade.retrieve_mediator(Mediator::NAME).is_some());

    let removed_mediator = facade.remove_mediator(Mediator::NAME).unwrap();

    assert_eq!(removed_mediator.lock().unwrap().name(), Mediator::NAME);

    assert!(facade.retrieve_mediator(Mediator::NAME).is_none());
}

#[test]
fn test_has_proxy() {
    let facade = Facade::get_instance("FacadeTestKey7", |k| Arc::new(Facade::new(k)));
    let proxy = Proxy::new(Some("hasProxyTest"), Some(Box::new(vec![1, 2, 3])));
    facade.register_proxy(Arc::new(Mutex::new(proxy)));

    assert!(facade.has_proxy("hasProxyTest"));
}

#[test]
fn test_has_mediator() {
    let facade = Facade::get_instance("FacadeTestKey8", |k| Arc::new(Facade::new(k)));
    let component = Arc::new(Mutex::new(Sprite::default()));
    let mediator = Mediator::new(Some("facadeHasMediatorTest"), Some(Arc::downgrade(&component).clone()));
    facade.register_mediator(Arc::new(Mutex::new(mediator)));

    assert!(facade.has_mediator("facadeHasMediatorTest"));

    facade.remove_mediator("facadeHasMediatorTest");

    assert!(!facade.has_mediator("facadeHasMediatorTest"));
}

#[test]
fn test_has_command() {
    let facade = Facade::get_instance("FacadeTestKey9", |k| Arc::new(Facade::new(k)));
    facade.register_command("FacadeTestCommand", Arc::new(|| {Box::new(FacadeTestCommand::new())}));

    assert!(facade.has_command("FacadeTestCommand"));

    facade.remove_command("FacadeTestCommand");

    assert!(!facade.has_command("FacadeTestCommand"));
}

#[test]
fn test_has_core_and_remove_core() {
    assert!(!Facade::has_core("FacadeTestKey10"));

    Facade::get_instance("FacadeTestKey10", |k| Arc::new(Facade::new(k)));

    assert!(Facade::has_core("FacadeTestKey10"));

    Facade::remove_core("FacadeTestKey10");

    assert!(!Facade::has_core("FacadeTestKey10"));
}
