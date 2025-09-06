use std::sync::{Arc, Mutex};
use puremvc::{Controller, ICommand, INotification, INotifier, Notification, SimpleCommand, View};

struct ControllerTestVO {
    input: i8,
    result: i8
}

struct ControllerTestCommand {
    command: SimpleCommand,
}

impl ControllerTestCommand {
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for ControllerTestCommand {}

impl ICommand for ControllerTestCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("No body in notification");

        let vo = body.downcast_mut::<ControllerTestVO>()
            .expect("Body is not a ControllerTestVO");

        vo.result = 2 * vo.input;
    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier_mut()
    }
}

struct ControllerTestCommand2 {
    command: SimpleCommand
}

impl ControllerTestCommand2 {
    fn new() -> Self {
        Self{command: SimpleCommand::new()}
    }
}

impl INotifier for ControllerTestCommand2 {}

impl ICommand for ControllerTestCommand2 {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("No body in notification");

        let vo = body.downcast_mut::<ControllerTestVO>()
            .expect("Body is not a ControllerTestVO");

        vo.result = vo.result + (2 * vo.input);
    }

    fn notifier_mut(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier_mut()
    }
}

#[test]
fn test_get_instance() {
    let controller = Controller::get_instance("ControllerTestKey1", |k| Arc::new(Controller::new(k)));

    assert!(Arc::strong_count(&controller) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_execute_command() {
    let controller = Controller::get_instance("ControllerTestKey2", |k| Arc::new(Controller::new(k)));

    controller.register_command("ControllerTest", Arc::new(|| Box::new(ControllerTestCommand::new())));

    let vo = ControllerTestVO { input: 12, result: 0 };
    let notification = Notification::new("ControllerTest", Some(Box::new(vo)), None);
    let note_arc: Arc<Mutex<dyn INotification>>  = Arc::new(Mutex::new(notification));
    
    controller.execute_command(&note_arc);

    let guard = note_arc.lock().unwrap();
    let body = guard.body().expect("No body in notification");
    let vo_result = body.downcast_ref::<ControllerTestVO>()
        .expect("Body is not a ControllerTestVO");
    assert_eq!(vo_result.result, 24);
}

#[test]
fn test_register_and_remove_command() {
    let controller = Controller::get_instance("ControllerTestKey3", |k| Arc::new(Controller::new(k)));

    controller.register_command("ControllerRemoveTest", Arc::new(|| Box::new(ControllerTestCommand::new())));

    let notification: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(
        Notification::new("ControllerRemoveTest", Some(Box::new(ControllerTestVO { input: 12, result: 0 })), None)
    ));

    controller.execute_command(&notification);

    assert_eq!(notification.lock().unwrap().body()
                   .and_then(|b| b.downcast_ref::<ControllerTestVO>())
                   .map(|vo| vo.result), Some(24));

    if let Some(body) = notification.lock().unwrap().body_mut() {
        if let Some(vo) = body.downcast_mut::<ControllerTestVO>() {
            vo.result = 0;
        }
    }

    controller.remove_command("ControllerRemoveTest");

    controller.execute_command(&notification);

    assert_eq!(notification.lock().unwrap().body()
                   .and_then(|b| b.downcast_ref::<ControllerTestVO>())
                   .map(|vo| vo.result), Some(0));
}

#[test]
fn test_has_command() {
    let controller = Controller::get_instance("ControllerTestKey4", |k| Arc::new(Controller::new(k)));

    controller.register_command("hasCommandTest", Arc::new(|| Box::new(ControllerTestCommand::new())));

    assert_eq!(controller.has_command("hasCommandTest"), true, "Expecting controller.has_command('hasCommandTest')");

    controller.remove_command("hasCommandTest");

    assert_eq!(controller.has_command("hasCommandTest"), false, "Expecting controller.has_command('hasCommandTest')");
}

#[test]
fn test_reregister_and_execute_command() {
    let controller = Controller::get_instance("ControllerTestKey5", |k| Arc::new(Controller::new(k)));

    controller.register_command("ControllerTest2", Arc::new(|| Box::new(ControllerTestCommand2::new())));
    controller.remove_command("ControllerTest2");
    controller.register_command("ControllerTest2", Arc::new(|| Box::new(ControllerTestCommand2::new())));

    let vo = ControllerTestVO { input: 12, result: 0 };
    let notification = Notification::new("ControllerTest2", Some(Box::new(vo)), None);
    let note_arc: Arc<Mutex<dyn INotification>>  = Arc::new(Mutex::new(notification));

    let view = View::get_instance("ControllerTestKey5", |k| Arc::new(View::new(k)));

    view.notify_observers(&note_arc);

    if let Some(body) = note_arc.lock().unwrap().body_mut() {
        if let Some(vo) = body.downcast_mut::<ControllerTestVO>() {
            vo.result = vo.result;
        }
    }

    assert_eq!(note_arc.lock().unwrap().body()
                   .and_then(|b| b.downcast_ref::<ControllerTestVO>())
                   .map(|vo| vo.result), Some(24));

    view.notify_observers(&note_arc);

    assert_eq!(note_arc.lock().unwrap().body()
                   .and_then(|b| b.downcast_ref::<ControllerTestVO>())
                   .map(|vo| vo.result), Some(48));
}
