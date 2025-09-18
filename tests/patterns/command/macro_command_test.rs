use std::sync::{Arc, Mutex};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{MacroCommand, Notification, SimpleCommand};

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
    fn key(&self) -> &str {
        self.command.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }
}

impl ICommand for MacroCommandTestSub1Command {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<Mutex<MacroCommandTestVO>>())
            .and_then(|mutex| mutex.lock().ok())
            .map(|mut vo| {
                vo.result1 = 2 * vo.input;
            });
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
    fn key(&self) -> &str {
        self.command.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }
}

impl ICommand for MacroCommandTestSub2Command {
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<Mutex<MacroCommandTestVO>>())
            .and_then(|mutex| mutex.lock().ok())
            .map(|mut vo| {
                vo.result2 = vo.input * vo.input;
            });
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
        self.command.add_sub_command(|| MacroCommandTestSub1Command::new());
        self.command.add_sub_command(|| MacroCommandTestSub2Command::new());
    }
}

impl INotifier for MacroCommandTestCommand {
    fn key(&self) -> &str {
        self.command.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
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

    let notification = Arc::new(Notification::new("MacroCommandTest", Some(vo), None));

    let mut command = MacroCommandTestCommand::new();
    command.execute(&(notification.clone() as Arc<dyn INotification>));

    notification.body()
        .and_then(|body| body.downcast_ref::<Mutex<MacroCommandTestVO>>())
        .and_then(|mutex| mutex.lock().ok())
        .map(|vo| {
            assert_eq!(vo.result1, 10);
            assert_eq!(vo.result2, 25);
        });
}
