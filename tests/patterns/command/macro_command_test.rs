use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, MacroCommand, SimpleCommand, Notification};

struct MacroCommandTestVO {
    input: i8,
    result1: i8,
    result2: i8
}

impl MacroCommandTestVO {
    pub fn new(input: i8) -> Self {
        Self { input, result1: 0, result2: 0 }
    }
}

struct MacroCommandTestSub1Command;

impl ICommand for MacroCommandTestSub1Command {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>)  {
        if let Some(body) = notification.lock().unwrap().body() {
            if let Some(vo) = body.downcast_mut::<MacroCommandTestVO>() {
                vo.result1 = 2 * vo.input;
            }
        }
    }
}

struct MacroCommandTestSub2Command;

impl ICommand for MacroCommandTestSub2Command {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>)  {
        if let Some(body) = notification.lock().unwrap().body() {
            if let Some(vo) = body.downcast_mut::<MacroCommandTestVO>() {
                vo.result2 = vo.input * vo.input;
            }
        }
    }
}

struct MacroCommandTestCommand(MacroCommand);

impl MacroCommandTestCommand {
    pub fn new() -> Self {
        Self(MacroCommand::new())
    }

    pub fn initialize_macro_command(&mut self) {
        self.0.add_sub_command(|| Box::new(MacroCommandTestSub1Command));
        self.0.add_sub_command(|| Box::new(MacroCommandTestSub2Command));
    }
}

impl ICommand for MacroCommandTestCommand {
    fn execute(&mut self, notification: Arc<Mutex<dyn INotification>>)  {
        self.initialize_macro_command();
        self.0.execute(notification);
    }
}

#[test]
fn test_macro_command_execute() {
    let vo = MacroCommandTestVO::new(5);

    let note = Arc::new(Mutex::new(
        Notification::new("MacroCommandTest", Some(Box::new(vo)), None)
    ));

    let mut command = MacroCommandTestCommand::new();
    command.execute(note.clone());

    let mut note_guard = note.lock().unwrap();
    let vo = note_guard.body().unwrap().downcast_ref::<MacroCommandTestVO>().unwrap();

    assert_eq!(vo.result1, 10, "Expecting vo.result1 == 10");
    assert_eq!(vo.result2, 25, "Expecting vo.result2 == 25");
}
