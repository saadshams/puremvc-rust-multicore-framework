use std::sync::{Arc, Mutex};
use puremvc::core::{Controller, View};
use puremvc::interfaces::{ICommand, INotification, INotifier};
use puremvc::patterns::{Notification, SimpleCommand};

struct ControllerTestVO {
    input: i8,
    result: i8
}

struct ControllerTestCommand {
    command: SimpleCommand
}

impl ControllerTestCommand {
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for ControllerTestCommand {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self.command.notifier()
    }
}

impl ICommand for ControllerTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(body) = notification.body() {
            let mut vo = body.downcast_ref::<Mutex<ControllerTestVO>>().unwrap().lock().unwrap();
            vo.result = 2 * vo.input;
        }
    }
}

struct ControllerTestCommand2 {
    command: SimpleCommand
}

impl ControllerTestCommand2 {
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for ControllerTestCommand2 {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self.command.notifier()
    }
}

impl ICommand for ControllerTestCommand2 {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(body) = notification.body() {
            let mut vo = body.downcast_ref::<Mutex<ControllerTestVO>>().unwrap().lock().unwrap();
            vo.result = vo.result + (2 * vo.input);
        }
    }
}

#[test]
fn test_get_instance() {
    let controller = Controller::get_instance("ControllerTestKey1", |k| Controller::new(k));

    assert!(Arc::strong_count(&controller) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_execute_command() {
    let controller = Controller::get_instance("ControllerTestKey2", |k| Controller::new(k));

    controller.register_command("ControllerTest", || Box::new(ControllerTestCommand::new()));

    let vo = Arc::new(Mutex::new(ControllerTestVO { input: 12, result: 0 }));
    let notification = Arc::new(Notification::new("ControllerTest", Some(vo.clone()), None));

    controller.execute_command(&(notification as Arc<dyn INotification>));

    assert_eq!(vo.lock().unwrap().result, 24);
}

#[test]
fn test_register_and_remove_command() {
    let controller = Controller::get_instance("ControllerTestKey3", |k| Controller::new(k));

    controller.register_command("ControllerRemoveTest", || Box::new(ControllerTestCommand::new()));

    let vo = Arc::new(Mutex::new(ControllerTestVO { input: 12, result: 0 }));
    let notification: Arc<dyn INotification> = Arc::new(Notification::new("ControllerRemoveTest", Some(vo.clone()), None));

    controller.execute_command(&notification);

    assert_eq!(vo.lock().unwrap().result, 24);

    vo.lock().unwrap().result = 0;

    controller.remove_command("ControllerRemoveTest");

    controller.execute_command(&notification);

    assert_eq!(vo.lock().unwrap().result, 0);
}

#[test]
fn test_has_command() {
    let controller = Controller::get_instance("ControllerTestKey4", |k| Controller::new(k));

    controller.register_command("hasCommandTest", || Box::new(ControllerTestCommand::new()));

    assert_eq!(controller.has_command("hasCommandTest"), true, "Expecting controller.has_command('hasCommandTest')");

    controller.remove_command("hasCommandTest");

    assert_eq!(controller.has_command("hasCommandTest"), false, "Expecting controller.has_command('hasCommandTest')");
}

#[test]
fn test_reregister_and_execute_command() {
    let controller = Controller::get_instance("ControllerTestKey5", |k| Controller::new(k));

    controller.register_command("ControllerTest2", || Box::new(ControllerTestCommand2::new()));
    controller.remove_command("ControllerTest2");
    controller.register_command("ControllerTest2", || Box::new(ControllerTestCommand2::new()));

    let vo = Arc::new(Mutex::new(ControllerTestVO { input: 12, result: 0 }));
    let notification: Arc<dyn INotification> = Arc::new(Notification::new("ControllerTest2", Some(vo.clone()), None));

    let view = View::get_instance("ControllerTestKey5", |k| View::new(k));

    view.notify_observers(&notification);
    
    assert_eq!(vo.lock().unwrap().result, 24);
}
