use std::sync::{Arc, Mutex};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{Facade, Mediator, Proxy, SimpleCommand};

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
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for FacadeTestCommand {
    fn notifier(&mut self) -> Option<&mut dyn INotifier> {
        self.command.notifier()
    }
}

impl ICommand for FacadeTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(body) = notification.body() {
            let mut vo = body.downcast_ref::<Mutex<FacadeTestVO>>().unwrap().lock().unwrap();

            vo.result = 2 * vo.input;
        }
    }
}

#[test]
fn test_get_instance() {
    let facade = Facade::get_instance("FacadeTestKey1", |k| Facade::new(k));

    assert!(Arc::strong_count(&facade) > 0, "Expecting instance not null");
}

#[test]
fn test_register_command_and_send_notification() {
    let arc = Facade::get_instance("FacadeTestKey2", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    facade.register_command("FacadeTestNote", || Box::new(FacadeTestCommand::new()));

    let vo = Arc::new(Mutex::new(FacadeTestVO{input: 32, result: 0}));
    facade.send_notification("FacadeTestNote", Some(vo.clone()), None);

    assert_eq!(vo.lock().unwrap().result, 64);
}

#[test]
fn test_register_and_remove_command_and_send_notification() {
    let arc = Facade::get_instance("FacadeTestKey3", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    facade.register_command( "FacadeTestNote", || Box::new(FacadeTestCommand::new()));
    facade.remove_command("FacadeTestNote");

    let vo = Arc::new(Mutex::new(FacadeTestVO{input: 32, result: 0}));
    facade.send_notification("FacadeTestNote", Some(vo.clone()), None);

    assert_ne!(vo.lock().unwrap().result, 64);
}

#[test]
fn test_register_and_retrieve_proxy() {
    let arc = Facade::get_instance("FacadeTestKey4", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors"), Some(Arc::new(Mutex::new(colors))));
    facade.register_proxy(Arc::new(Mutex::new(proxy)));

    let proxy = facade.retrieve_proxy("colors").unwrap();

    if let Some(data) = proxy.lock().unwrap().data() {
        let mut colors = data.downcast_ref::<Mutex<Vec<String>>>().unwrap().lock().unwrap();

        assert_eq!(&*colors, &["red", "green", "blue"]);
        
        colors.push("yellow".to_string());
    }

    if let Some(data) = proxy.lock().unwrap().data() {
        let colors = data.downcast_ref::<Mutex<Vec<String>>>().unwrap().lock().unwrap();
        assert_eq!(&*colors, &["red", "green", "blue", "yellow"]);
    }
}

#[test]
fn test_register_and_remove_proxy() {
    let arc = Facade::get_instance("FacadeTestKey5", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    let sizes = vec![7, 13, 21];
    let proxy = Proxy::new(Some("sizes"), Some(Arc::new(sizes)));
    facade.register_proxy(Arc::new(Mutex::new(proxy)));

    let removed_proxy = facade.remove_proxy("sizes").unwrap();

    assert_eq!(removed_proxy.lock().unwrap().name(), "sizes");

    assert!(facade.retrieve_proxy("sizes").is_none());
}

#[test]
fn test_register_retrieve_and_remove_mediator() {
    let arc = Facade::get_instance("FacadeTestKey6", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
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
    let arc = Facade::get_instance("FacadeTestKey7", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    let proxy = Proxy::new(Some("hasProxyTest"), Some(Arc::new(vec![1, 2, 3])));
    facade.register_proxy(Arc::new(Mutex::new(proxy)));

    assert!(facade.has_proxy("hasProxyTest"));
}

#[test]
fn test_has_mediator() {
    let arc = Facade::get_instance("FacadeTestKey8", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    let component = Arc::new(Mutex::new(Sprite::default()));
    let mediator = Mediator::new(Some("facadeHasMediatorTest"), Some(Arc::downgrade(&component).clone()));
    facade.register_mediator(Arc::new(Mutex::new(mediator)));

    assert!(facade.has_mediator("facadeHasMediatorTest"));

    facade.remove_mediator("facadeHasMediatorTest");

    assert!(!facade.has_mediator("facadeHasMediatorTest"));
}

#[test]
fn test_has_command() {
    let arc = Facade::get_instance("FacadeTestKey9", |k| Facade::new(k));
    let facade = arc.lock().unwrap();
    facade.register_command("FacadeTestCommand", || Box::new(FacadeTestCommand::new()));

    assert!(facade.has_command("FacadeTestCommand"));

    facade.remove_command("FacadeTestCommand");

    assert!(!facade.has_command("FacadeTestCommand"));
}

#[test]
fn test_has_core_and_remove_core() {
    assert!(!Facade::has_core("FacadeTestKey10"));

    Facade::get_instance("FacadeTestKey10", |k| Facade::new(k));

    assert!(Facade::has_core("FacadeTestKey10"));

    Facade::remove_core("FacadeTestKey10");

    assert!(!Facade::has_core("FacadeTestKey10"));
}
