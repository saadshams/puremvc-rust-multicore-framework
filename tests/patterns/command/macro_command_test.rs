use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{MacroCommand, Notification, SimpleCommand};

/// A utility struct used by MacroCommand tests to hold input and result values.
struct MacroCommandTestVO {
    input: i8,
    result1: i8,
    result2: i8,
}

/// A SimpleCommand subclass used by MacroCommandTestCommand.
struct MacroCommandTestSub1Command {
    command: SimpleCommand
}

impl MacroCommandTestSub1Command {
    /// Constructor.
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for MacroCommandTestSub1Command {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.command.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}

impl ICommand for MacroCommandTestSub1Command {
    /// Fabricates a result by multiplying the input by 2.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the MacroCommandTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<MacroCommandTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result1 = 2 * vo.input;
            });
    }
}

/// A SimpleCommand subclass used by MacroCommandTestCommand.
struct MacroCommandTestSub2Command {
    command: SimpleCommand
}

impl MacroCommandTestSub2Command {
    /// Constructor.
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for MacroCommandTestSub2Command {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.command.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}

impl ICommand for MacroCommandTestSub2Command {
    /// Fabricates a result by multiplying the input by itself.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the MacroCommandTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<MacroCommandTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result2 = vo.input * vo.input;
            });
    }
}

/// A MacroCommand subclass used by MacroCommandTest.
struct MacroCommandTestCommand {
    command: MacroCommand,
}

impl MacroCommandTestCommand {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            command: MacroCommand::new()
        }
    }

    /// Initializes the MacroCommand by adding its subcommands.
    fn initialize_macro_command(&mut self) {
        self.command.add_sub_command(|| MacroCommandTestSub1Command::new());
        self.command.add_sub_command(|| MacroCommandTestSub2Command::new());
    }
}

impl INotifier for MacroCommandTestCommand {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.command.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}

impl ICommand for MacroCommandTestCommand {
    /// Executes the MacroCommand, initializing and running its subcommands.
    ///
    /// # Arguments
    /// * `notification` - The notification to pass to subcommands
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        self.initialize_macro_command();
        self.command.execute(&notification);
    }
}

/// Tests operation of a MacroCommand.
///
/// Creates a `MacroCommandTestVO` with an input value of 5, wraps it in a
/// `Notification`, and executes a `MacroCommandTestCommand`. The command
/// initializes two subcommands: `MacroCommandTestSub1Command` (multiplies input
/// by 2) and `MacroCommandTestSub2Command` (multiplies input by itself). Success
/// is verified by asserting that `result1` is 10 and `result2` is 25.
#[test]
fn test_macro_command_execute() {
    // Create a value object with input value 5
    let vo = Arc::new(RwLock::new(MacroCommandTestVO { input: 5, result1: 0, result2: 0 }));

    // Create a notification with the value object
    let notification = Arc::new(Notification::new("MacroCommandTest", Some(vo), None));

    // Create the MacroCommandTestCommand
    let mut command = MacroCommandTestCommand::new();
    // Execute the command, which runs its subcommands
    command.execute(&(notification.clone() as Arc<dyn INotification>));

    // Verify the results of the subcommands
    notification.body()
        .and_then(|body| body.downcast_ref::<RwLock<MacroCommandTestVO>>())
        .and_then(|mutex| mutex.read().ok())
        .map(|vo| {
            // Assert that result1 is input * 2 (5 * 2 = 10)
            assert_eq!(vo.result1, 10);
            // Assert that result2 is input * input (5 * 5 = 25)
            assert_eq!(vo.result2, 25);
        });
}
