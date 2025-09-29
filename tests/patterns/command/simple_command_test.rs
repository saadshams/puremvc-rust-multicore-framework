use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{Notification, SimpleCommand};

/// A utility struct used by SimpleCommand tests to hold input and result values.
struct SimpleCommandTestVO {
    input: i8,
    result: i8,
}

/// A SimpleCommand subclass used by SimpleCommandTest.
struct SimpleCommandTestCommand {
    command: SimpleCommand
}

impl SimpleCommandTestCommand {
    /// Constructor.
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for SimpleCommandTestCommand {
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

impl ICommand for SimpleCommandTestCommand {
    /// Fabricates a result by multiplying the input by 2.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the SimpleCommandTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<SimpleCommandTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
    }
}

/// Tests the `execute` method of a SimpleCommand.
///
/// Creates a `SimpleCommandTestVO` with an input value of 5, wraps it in a
/// `Notification`, and executes a `SimpleCommandTestCommand`. The command
/// multiplies the input by 2, and success is verified by asserting that `result`
/// is 10.
#[test]
fn test_simple_command_execute() {
    // Create a value object with input value 5
    let vo = Arc::new(RwLock::new(SimpleCommandTestVO { input: 5, result: 0 }));
    // Create a notification with the value object
    let note = Arc::new(Notification::new("SimpleCommandTestNote", Some(vo.clone()), None));

    // Create the SimpleCommandTestCommand
    let mut command = SimpleCommandTestCommand::new();
    // Execute the command, which multiplies the input by 2
    command.execute(&(note as Arc<dyn INotification>));

    // Assert that the result is input * 2 (5 * 2 = 10)
    assert_eq!(vo.read().unwrap().result, 10, "Expecting vo.result == 10");
}
