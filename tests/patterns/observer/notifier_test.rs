use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{Facade, Notifier, SimpleCommand};

/// A utility struct used by Notifier tests to hold input and result values.
struct NotifierTestVO {
    input: i8,
    result: i8
}

/// A SimpleCommand subclass used by NotifierTest.
struct NotifierTestCommand {
    command: SimpleCommand
}

impl NotifierTestCommand {
    /// Constructor.
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for NotifierTestCommand {
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

impl ICommand for NotifierTestCommand {
    /// Fabricates a result by multiplying the input by 2.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the NotifierTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|arc| arc.downcast_ref::<RwLock<NotifierTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
    }
}

/// Tests the Notifier’s ability to send a notification that triggers a command.
///
/// Gets a Facade instance, registers a `NotifierTestCommand` for
/// 'NotifierTestNote' notifications, and uses a `Notifier` to send a
/// notification with a `NotifierTestVO`. The command multiplies the VO’s input
/// by 2, and success is verified by asserting that `result` is 10 for an input
/// of 5.
#[test]
fn test_notifier() {
    // Get a Multiton Facade instance
    let facade= Facade::get_instance("NotifierTestKey1", |k| Facade::new(k));
    // Register the NotifierTestCommand for 'NotifierTestNote' notifications
    facade.register_command("NotifierTestNote", || Box::new(NotifierTestCommand::new()));

    // Create a value object with input value 5
    let vo = Arc::new(RwLock::new(NotifierTestVO{ input: 5, result: 0 }));

    // Create and initialize a Notifier
    let mut notifier = Notifier::new();
    notifier.initialize_notifier("NotifierTestKey1");
    // Send the notification to trigger the command
    notifier.send_notification("NotifierTestNote", Some(vo.clone()), None);

    // Assert that the result is input * 2 (5 * 2 = 10)
    assert_eq!(vo.read().unwrap().result, 10);
}
