use std::sync::{Arc, Mutex};
use puremvc::{ICommand, INotification, INotifier, MacroCommand, Notification, SimpleCommand};

struct MacroCommandTestVO {
    input: i8,
    result1: i8,
    result2: i8,
}

struct MacroCommandTestSub1Command {
    command: SimpleCommand
}

impl MacroCommandTestSub1Command {
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for MacroCommandTestSub1Command {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self.command.notifier()
    }
}

impl ICommand for MacroCommandTestSub1Command {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(body) = notification.body() {
            let mut vo = body.downcast_ref::<Mutex<MacroCommandTestVO>>().unwrap().lock().unwrap();
            
            vo.result1 = 2 * vo.input;
        }
    }
}

struct MacroCommandTestSub2Command {
    command: SimpleCommand
}

impl MacroCommandTestSub2Command {
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for MacroCommandTestSub2Command {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self.command.notifier()
    }
}

impl ICommand for MacroCommandTestSub2Command {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        if let Some(body) = notification.body() {
            let mut vo = body.downcast_ref::<Mutex<MacroCommandTestVO>>().unwrap().lock().unwrap();
            vo.result2 = vo.input * vo.input;
        }
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

impl INotifier for MacroCommandTestCommand {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self as &mut dyn INotifier
    }
}

impl ICommand for MacroCommandTestCommand {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        self.initialize_macro_command();
        self.command.execute(&notification);
    }
}

#[test]
fn test_macro_command_execute() {
    let vo = Arc::new(Mutex::new(MacroCommandTestVO { input: 5, result1: 0, result2: 0 }));

    let notification = Arc::new(Notification::new("MacroCommandTest", Some(vo.clone()), None));

    let mut command = MacroCommandTestCommand::new();
    command.execute(&(notification as Arc<dyn INotification>));

    assert_eq!(vo.lock().unwrap().result1, 10, "Expecting vo.result1 == 10");
    assert_eq!(vo.lock().unwrap().result2, 25, "Expecting vo.result2 == 25");
}