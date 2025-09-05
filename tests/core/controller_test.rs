use std::sync::{Arc, Mutex};
use puremvc::{Controller, ICommand, INotification, Notification};

struct ControllerTestVO {
    input: i8,
    result: i8
}

struct ControllerTestCommand;

impl ICommand for ControllerTestCommand {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>) {
        let note = notification.lock().unwrap();
        let body = note.body().expect("No body in notification");
        let mut vo = body.lock().unwrap();

        let vo = vo.downcast_mut::<ControllerTestVO>()
            .expect("Body is not a ControllerTestVO");

        vo.result = 2 * vo.input;
    }
}

struct ControllerTestCommand2;

impl ICommand for ControllerTestCommand2 {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>) {
        let note = notification.lock().unwrap();
        let body = note.body().expect("No body in notification");
        let mut vo = body.lock().unwrap();

        let vo = vo.downcast_mut::<ControllerTestVO>()
            .expect("Body is not a ControllerTestVO");

        vo.result = vo.result + (2 * vo.input);
    }
}

#[test]
fn test_get_instance() {
    let controller = Controller::get_instance("ControllerTestKey1", |k| Arc::new(Controller::new(k)));

    assert!(Arc::strong_count(&controller) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_execute_command() {
    let controller = Controller::get_instance("ControllerTestKey2", |k| { Arc::new(Controller::new(k))});

    controller.register_command("MyNotification", Arc::new(|| Arc::new(Mutex::new(ControllerTestCommand))));

    let vo = Arc::new(Mutex::new(ControllerTestVO { input: 0, result: 0 }));

    let notification = Notification::new("ControllerTest", Some(vo.clone()), None);
    controller.execute_command(Arc::new(Mutex::new(notification)));

    assert_eq!(vo.lock().unwrap().result, 24, "");
}
