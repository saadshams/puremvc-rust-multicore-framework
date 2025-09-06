use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, MacroCommand, Notification};

struct MacroCommandTestVO {
    input: i8,
    result1: i8,
    result2: i8,
}

struct MacroCommandTestSub1Command;

impl ICommand for MacroCommandTestSub1Command {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("Notification body missing");
        let vo = body.downcast_mut::<MacroCommandTestVO>()
            .expect("Body is not a MacroCommandTestVO");

        vo.result1 = 2 * vo.input;
    }
}

struct MacroCommandTestSub2Command;

impl ICommand for MacroCommandTestSub2Command {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let mut note = notification.lock().unwrap();
        let body = note.body_mut().expect("Notification body missing");
        let vo = body.downcast_mut::<MacroCommandTestVO>()
            .expect("Body is not a MacroCommandTestVO");

        vo.result2 = vo.input * vo.input;
    }
}

struct MacroCommandTestCommand {
    macro_command: MacroCommand,
}

impl MacroCommandTestCommand {
    pub fn new() -> Self {
        Self {
            macro_command: MacroCommand::new(),
        }
    }

    fn initialize_macro_command(&mut self) {
        // Add sub-commands - note the syntax for creating boxed commands
        self.macro_command.add_sub_command(|| Box::new(MacroCommandTestSub1Command));
        self.macro_command.add_sub_command(|| Box::new(MacroCommandTestSub2Command));
    }
}

impl ICommand for MacroCommandTestCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        self.initialize_macro_command();
        self.macro_command.execute(&notification);
    }
}

#[test]
fn test_macro_command_execute() {
    let note: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new(
        "MacroCommandTest",
        Some(Box::new(MacroCommandTestVO { input: 5, result1: 0, result2: 0 })),
        None,
    )));

    // Execute the command
    let mut command = MacroCommandTestCommand::new();
    command.execute(&note);

    {
        let guard = note.lock().unwrap();
        let body = guard.body().expect("Notification body missing");
        let vo = body.downcast_ref::<MacroCommandTestVO>()
            .expect("Body is not a MacroCommandTestVO");
        
        assert_eq!(vo.result1, 10, "Expecting vo.result1 == 10");
        assert_eq!(vo.result2, 25, "Expecting vo.result2 == 25");
    }
}