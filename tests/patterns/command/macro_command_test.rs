use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, INotifier, MacroCommand, Notification, SimpleCommand};

struct MacroCommandTestVO {
    input: i8,
    result1: i8,
    result2: i8,
}

struct MacroCommandTestSub1Command {
    command: SimpleCommand,
}

impl MacroCommandTestSub1Command {
    fn new() -> Self {
        Self {
            command: SimpleCommand::new()
        }
    }
}

impl INotifier for MacroCommandTestSub1Command {}

impl ICommand for MacroCommandTestSub1Command {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let note = notification.lock().unwrap();
        let body = note.body().cloned().unwrap();
        let mut guard = body.lock().unwrap();
        let vo = guard.downcast_mut::<MacroCommandTestVO>().unwrap();

        vo.result1 = 2 * vo.input;
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier()
    }
}

struct MacroCommandTestSub2Command {
    command: SimpleCommand,
}

impl MacroCommandTestSub2Command {
    fn new() -> Self {
        Self {
            command: SimpleCommand::new()
        }
    }
}

impl INotifier for MacroCommandTestSub2Command {}

impl ICommand for MacroCommandTestSub2Command {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        let note = notification.lock().unwrap();
        let body = note.body().cloned().unwrap();

        let mut guard = body.lock().unwrap();
        let vo = guard.downcast_mut::<MacroCommandTestVO>().unwrap();

        vo.result2 = vo.input * vo.input;
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.command.notifier()
    }
}

struct MacroCommandTestCommand {
    command: MacroCommand,
}

impl MacroCommandTestCommand {
    pub fn new() -> Self {
        Self {
            command: MacroCommand::new()
        }
    }

    fn initialize_macro_command(&mut self) {
        self.command.add_sub_command(|| Box::new(MacroCommandTestSub1Command::new()));
        self.command.add_sub_command(|| Box::new(MacroCommandTestSub2Command::new()));
    }
}

impl INotifier for MacroCommandTestCommand {}

impl ICommand for MacroCommandTestCommand {
    fn execute(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        self.initialize_macro_command();
        self.command.execute(&notification);
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        todo!()
    }
}

#[test]
fn test_macro_command_execute() {
    let vo = Arc::new(Mutex::new(MacroCommandTestVO { input: 5, result1: 0, result2: 0 }));

    let notification: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new(
        "MacroCommandTest",
        Some(vo.clone()),
        None,
    )));

    let mut command = MacroCommandTestCommand::new();
    command.execute(&notification);

    assert_eq!(vo.lock().unwrap().result1, 10, "Expecting vo.result1 == 10");
    assert_eq!(vo.lock().unwrap().result2, 25, "Expecting vo.result2 == 25");
}