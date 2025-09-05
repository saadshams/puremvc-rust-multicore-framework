use std::sync::{Arc, Mutex};
use puremvc::{Controller, ICommand, INotification, Notification};
#[derive(Clone)]
struct ControllerTestVO {
    input: i8,
    result: i8
}

struct ControllerTestCommand;

impl ICommand for ControllerTestCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("No body in notification");

        let vo = body.downcast_mut::<ControllerTestVO>()
            .expect("Body is not a ControllerTestVO");

        vo.result = 2 * vo.input;
    }
}

struct ControllerTestCommand2;

impl ICommand for ControllerTestCommand2 {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("No body in notification");

        let vo = body.downcast_mut::<ControllerTestVO>()
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
    let controller = Controller::get_instance("ControllerTestKey2", |k| { 
        Arc::new(Controller::new(k))
    });

    controller.register_command(
        "ControllerTest", 
        Arc::new(|| Arc::new(Mutex::new(ControllerTestCommand)))
    );

    // Create the VO and notification
    let vo = Box::new(ControllerTestVO {
        input: 12, 
        result: 0
    });
    
    let notification = Notification::new("ControllerTest", Some(vo), None);
    let note_arc = Arc::new(Mutex::new(notification));
    
    // Execute the command
    controller.execute_command(note_arc.clone());

    // Alternative: if you need to access the result multiple times
    fn check_result(note: &Arc<Mutex<Notification>>) {
        let guard = note.lock().unwrap();
        let body = guard.body().expect("No body in notification");
        let vo_result = body.downcast_ref::<ControllerTestVO>()
            .expect("Body is not a ControllerTestVO");
        assert_eq!(vo_result.result, 24);
    }
    
    check_result(&note_arc);
}
