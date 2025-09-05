use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, Notification};

struct SimpleCommandTestVO {
    input: i8,
    result: i8,
}

struct SimpleCommandTestCommand;

impl ICommand for SimpleCommandTestCommand {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>) {
        let note = notification.lock().unwrap();
        let body = note.body().expect("No body in notification");
        let mut vo = body.lock().unwrap();

        let vo = vo.downcast_mut::<SimpleCommandTestVO>()
            .expect("Body is not a ControllerTestVO");

        vo.result = 2 * vo.input;
    }
}

#[test]
fn test_simple_command_execute() {
    let note = Arc::new(Mutex::new(Notification::new(
        "SimpleCommandTestNote",
        Some(Arc::new(Mutex::new(SimpleCommandTestVO{input: 5, result: 0}))),
        None
    )));

    let mut command = SimpleCommandTestCommand;
    command.execute(note.clone());

    let note = note.lock().unwrap();
    let body = note.body().expect("No body in notification");
    let mut vo = body.lock().unwrap();

    let vo = vo.downcast_mut::<SimpleCommandTestVO>()
        .expect("Body is not a ControllerTestVO");

    assert_eq!(vo.result, 10, "Expecting vo.result == 10");
}
