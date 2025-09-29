use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::core::{Controller, View};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{Notification, SimpleCommand};

/// A utility class used by Controller tests.
///
/// This value object holds an input number and a result, used to
/// verify the effects of command execution in the Controller tests.
struct ControllerTestVO {
    /// The number to be fed to the ControllerTestCommand or ControllerTestCommand2.
    input: i8,
    result: i8
}

/// A SimpleCommand subclass used by Controller tests.
struct ControllerTestCommand {
    command: SimpleCommand
}

impl ControllerTestCommand {
    /// Constructor.
    pub fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for ControllerTestCommand {
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

impl ICommand for ControllerTestCommand {
    /// Fabricate a result by multiplying the input by 2.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the ControllerTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<ControllerTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
    }
}

/// A SimpleCommand subclass used by Controller tests.
struct ControllerTestCommand2 {
    command: SimpleCommand
}

impl ControllerTestCommand2 {
    /// Constructor.
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for ControllerTestCommand2 {
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

impl ICommand for ControllerTestCommand2 {
    /// Fabricate a result by multiplying the input by 2 and adding to the existing result.
    ///
    /// This tests the accumulation effect that would show if the command were executed more than once.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the ControllerTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<ControllerTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result = vo.result + (2 * vo.input);
            });
    }
}

/// Tests the Controller Multiton Factory Method
#[test]
fn test_get_instance() {
    // Get a Multiton Controller instance
    let controller = Controller::get_instance("ControllerTestKey1", |k| Controller::new(k));

    // Assert that the instance is not null
    assert!(Arc::strong_count(&controller) > 0, "Expecting instance not null");
}

/// Tests Command registration and execution.
///
/// This test gets a Multiton Controller instance and registers the
/// `ControllerTestCommand` to handle `ControllerTest` notifications.
/// It then constructs such a notification and tells the Controller
/// to execute the associated Command. Success is determined by
/// evaluating a property on an object passed to the Command, which
/// will be modified when the Command executes.
#[test]
fn test_register_and_execute_command() {
    // Get a Multiton Controller instance
    let controller = Controller::get_instance("ControllerTestKey2", |k| Controller::new(k));

    // Register the ControllerTestCommand for 'ControllerTest' notifications
    controller.register_command("ControllerTest", || Box::new(ControllerTestCommand::new()));

    // Create a value object with input value 12
    let vo = Arc::new(RwLock::new(ControllerTestVO { input: 12, result: 0 }));
    // Create a notification with the value object
    let notification = Arc::new(Notification::new("ControllerTest", Some(vo.clone()), None));

    // Execute the command via the controller
    controller.execute_command(&(notification as Arc<dyn INotification>));

    // Assert that the result is input * 2 (12 * 2 = 24)
    assert_eq!(vo.write().unwrap().result, 24);
}

/// Tests Command registration and removal.
///
/// Tests that once a Command is registered and verified working,
/// it can be removed from the Controller.
#[test]
fn test_register_and_remove_command() {
    // Get a Multiton Controller instance
    let controller = Controller::get_instance("ControllerTestKey3", |k| Controller::new(k));

    // Register the ControllerTestCommand for 'ControllerRemoveTest' notifications
    controller.register_command("ControllerRemoveTest", || Box::new(ControllerTestCommand::new()));

    // Create a value object with input value 12
    let vo = Arc::new(RwLock::new(ControllerTestVO { input: 12, result: 0 }));
    // Create a notification with the value object
    let notification: Arc<dyn INotification> = Arc::new(Notification::new("ControllerRemoveTest", Some(vo.clone()), None));

    // Execute the command to verify it works
    controller.execute_command(&notification);

    // Assert that the result is input * 2 (12 * 2 = 24)
    assert_eq!(vo.read().unwrap().result, 24);

    // Reset the result to 0
    vo.write().unwrap().result = 0;

    // Remove the command
    controller.remove_command("ControllerRemoveTest");

    // Attempt to execute the command again
    controller.execute_command(&notification);

    // Assert that the result remains 0, confirming the command was removed
    assert_eq!(vo.read().unwrap().result, 0);
}

/// Tests the `has_command` method.
#[test]
fn test_has_command() {
    // Get a Multiton Controller instance
    let controller = Controller::get_instance("ControllerTestKey4", |k| Controller::new(k));

    // Register the ControllerTestCommand for 'hasCommandTest' notifications
    controller.register_command("hasCommandTest", || Box::new(ControllerTestCommand::new()));

    // Assert that has_command returns true for the registered command
    assert_eq!(controller.has_command("hasCommandTest"), true, "Expecting controller.has_command('hasCommandTest')");

    // Remove the command
    controller.remove_command("hasCommandTest");

    // Assert that has_command returns false after removal
    assert_eq!(controller.has_command("hasCommandTest"), false, "Expecting controller.has_command('hasCommandTest')");
}

/// Tests removing and re-registering a Command.
///
/// Tests that when a Command is re-registered, it isn't fired twice.
/// This involves registration with the controller and notification
/// via the View, rather than direct execution of the Controller's
/// `execute_command` method. The test ensures that the command
/// executes correctly after re-registration.
#[test]
fn test_reregister_and_execute_command() {
    // Get a Multiton Controller instance
    let controller = Controller::get_instance("ControllerTestKey5", |k| Controller::new(k));

    // Register the ControllerTestCommand2 for 'ControllerTest2' notifications
    controller.register_command("ControllerTest2", || Box::new(ControllerTestCommand2::new()));
    // Remove the command
    controller.remove_command("ControllerTest2");
    // Re-register the command
    controller.register_command("ControllerTest2", || Box::new(ControllerTestCommand2::new()));

    // Create a value object with input value 12
    let vo = Arc::new(RwLock::new(ControllerTestVO { input: 12, result: 0 }));
    // Create a notification with the value object
    let notification: Arc<dyn INotification> = Arc::new(Notification::new("ControllerTest2", Some(vo.clone()), None));

    // Get a Multiton View instance
    let view = View::get_instance("ControllerTestKey5", |k| View::new(k));

    // Notify observers to trigger the command execution
    view.notify_observers(&notification);

    // Assert that the result is input * 2 (12 * 2 = 24), confirming single execution
    assert_eq!(vo.read().unwrap().result, 24);
}
