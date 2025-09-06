use std::sync::{Arc, Mutex};
use puremvc::{Controller, ICommand, INotification, INotifier, Notification, SimpleCommand};

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
