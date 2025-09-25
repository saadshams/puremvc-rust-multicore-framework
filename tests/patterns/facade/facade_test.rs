use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::interfaces::{ICommand, IFacade, INotification, INotifier};
use puremvc::patterns::{Facade, Mediator, Proxy, SimpleCommand};

/// A utility struct to simulate a Flash Sprite for testing.
struct Sprite {}
impl Default for Sprite {
    fn default() -> Self { Self {} }
}

/// A utility struct used by Facade tests to hold input and result values.
struct FacadeTestVO {
    input: i8,
    result: i8
}

/// A SimpleCommand subclass used by FacadeTest.
struct FacadeTestCommand {
    command: SimpleCommand
}

impl FacadeTestCommand {
    /// Constructor.
    fn new() -> Self {
        Self { command: SimpleCommand::new() }
    }
}

impl INotifier for FacadeTestCommand {
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

impl ICommand for FacadeTestCommand {
    /// Fabricates a result by multiplying the input by 2.
    ///
    /// # Arguments
    /// * `notification` - The notification carrying the FacadeTestVO
    fn execute(&mut self, notification: &Arc<dyn INotification>) {
        notification.body()
            .and_then(|body| body.downcast_ref::<RwLock<FacadeTestVO>>())
            .and_then(|mutex| mutex.write().ok())
            .map(|mut vo| {
                vo.result = 2 * vo.input;
            });
    }
}

/// Tests the Facade Multiton Factory Method.
///
/// Gets a Facade instance and asserts it is not null.
#[test]
fn test_get_instance() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey1", |k| Facade::new(k));

    // Assert that the instance is not null
    assert!(Arc::strong_count(&facade) > 0, "Expecting instance not null");
}

/// Tests command registration and execution via the Facade.
///
/// Gets a Facade instance, registers a `FacadeTestCommand` to handle
/// 'FacadeTestNote' notifications, and sends a notification with a
/// `FacadeTestVO`. The command multiplies the VO's input by 2, and success
/// is verified by asserting that `result` is 64 for an input of 32.
#[test]
fn test_register_command_and_send_notification() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey2", |k| Facade::new(k));
    // Register the FacadeTestCommand for 'FacadeTestNote' notifications
    facade.register_command("FacadeTestNote", || Box::new(FacadeTestCommand::new()));

    // Create a value object with input value 32
    let vo = Arc::new(RwLock::new(FacadeTestVO{input: 32, result: 0}));
    // Send the notification to trigger the command
    facade.send_notification("FacadeTestNote", Some(vo.clone()), None);

    // Assert that the result is input * 2 (32 * 2 = 64)
    assert_eq!(vo.read().unwrap().result, 64);
}

/// Tests command registration and removal via the Facade.
///
/// Gets a Facade instance, registers and removes a `FacadeTestCommand` for
/// 'FacadeTestNote' notifications, then sends a notification with a
/// `FacadeTestVO`. Success is verified by asserting that the command did not
/// execute, so `result` is not 64 for an input of 32.
#[test]
fn test_register_and_remove_command_and_send_notification() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey3", |k| Facade::new(k));
    // Register the FacadeTestCommand for 'FacadeTestNote' notifications
    facade.register_command( "FacadeTestNote", || Box::new(FacadeTestCommand::new()));
    // Remove the command
    facade.remove_command("FacadeTestNote");

    // Create a value object with input value 32
    let vo = Arc::new(RwLock::new(FacadeTestVO{input: 32, result: 0}));
    // Send the notification, which should not trigger the command
    facade.send_notification("FacadeTestNote", Some(vo.clone()), None);

    // Assert that the result is not input * 2 (32 * 2 = 64)
    assert_ne!(vo.read().unwrap().result, 64);
}

/// Tests registering and retrieving a Proxy via the Facade.
///
/// Gets a Facade instance, registers a `Proxy` with a vector of colors,
/// retrieves it, and verifies its data. Success is verified by asserting
/// the data is a vector containing "red", "green", and "blue", and that
/// appending "yellow" works as expected.
#[test]
fn test_register_and_retrieve_proxy() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey4", |k| Facade::new(k));
    // Create and register a proxy with a vector of colors
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    let proxy = Proxy::new(Some("colors"), Some(Arc::new(RwLock::new(colors))));
    facade.register_proxy(Arc::new(RwLock::new(proxy)));

    // Retrieve the proxy
    let proxy = facade.retrieve_proxy("colors").unwrap();

    // Verify the proxy's data
    if let Some(data) = proxy.read().unwrap().data() {
        let mut colors = data.downcast_ref::<RwLock<Vec<String>>>().unwrap().write().unwrap();

        // Assert that the data matches the expected colors
        assert_eq!(&*colors, &["red", "green", "blue"]);

        // Append a new color to test data modification
        colors.push("yellow".to_string());
    }

    // Verify the modified data
    if let Some(data) = proxy.read().unwrap().data() {
        let colors = data.downcast_ref::<RwLock<Vec<String>>>().unwrap().read().unwrap();
        // Assert that the data now includes the new color
        assert_eq!(&*colors, &["red", "green", "blue", "yellow"]);
    }
}

/// Tests registering and removing a Proxy via the Facade.
///
/// Gets a Facade instance, registers a `Proxy` with a vector of sizes,
/// removes it, and verifies it is no longer retrievable. Success is verified
/// by asserting the removed proxy’s name and that it cannot be retrieved.
#[test]
fn test_register_and_remove_proxy() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey5", |k| Facade::new(k));
    // Create and register a proxy with a vector of sizes
    let sizes = vec![7, 13, 21];
    let proxy = Proxy::new(Some("sizes"), Some(Arc::new(sizes)));
    facade.register_proxy(Arc::new(RwLock::new(proxy)));

    // Remove the proxy
    let removed_proxy = facade.remove_proxy("sizes").unwrap();

    // Assert that the removed proxy has the expected name
    assert_eq!(removed_proxy.read().unwrap().name(), "sizes");

    // Assert that the proxy is no longer retrievable
    assert!(facade.retrieve_proxy("sizes").is_none());
}

/// Tests registering, retrieving, and removing a Mediator via the Facade.
///
/// Gets a Facade instance, registers a `Mediator`, retrieves it, removes it,
/// and verifies it is no longer retrievable. Success is verified by asserting
/// the mediator is retrievable after registration, the removed mediator’s name
/// is correct, and it cannot be retrieved after removal.
#[test]
fn test_register_retrieve_and_remove_mediator() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey6", |k| Facade::new(k));
    // Create and register a mediator with a Sprite component
    let component = Arc::new(RwLock::new(Sprite::default()));
    let mediator = Mediator::new(Some(Mediator::NAME), Some(Arc::downgrade(&component).clone()));

    facade.register_mediator(Arc::new(RwLock::new(mediator)));

    // Assert that the mediator is retrievable
    assert!(facade.retrieve_mediator(Mediator::NAME).is_some());

    // Remove the mediator
    let removed_mediator = facade.remove_mediator(Mediator::NAME).unwrap();

    // Assert that the removed mediator has the expected name
    assert_eq!(removed_mediator.read().unwrap().name(), Mediator::NAME);

    // Assert that the mediator is no longer retrievable
    assert!(facade.retrieve_mediator(Mediator::NAME).is_none());
}

/// Tests the `has_proxy` method.
///
/// Gets a Facade instance, registers a `Proxy`, and verifies that `has_proxy`
/// returns true for the proxy’s name.
#[test]
fn test_has_proxy() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey7", |k| Facade::new(k));
    // Create and register a proxy with a vector of numbers
    let proxy = Proxy::new(Some("hasProxyTest"), Some(Arc::new(vec![1, 2, 3])));
    facade.register_proxy(Arc::new(RwLock::new(proxy)));

    // Assert that has_proxy returns true for the registered proxy
    assert!(facade.has_proxy("hasProxyTest"));
}

/// Tests the `has_mediator` method.
///
/// Gets a Facade instance, registers a `Mediator`, verifies that `has_mediator`
/// returns true, removes the mediator, and verifies that `has_mediator` returns
/// false.
#[test]
fn test_has_mediator() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey8", |k| Facade::new(k));
    // Create and register a mediator with a Sprite component
    let component = Arc::new(RwLock::new(Sprite::default()));
    let mediator = Mediator::new(Some("facadeHasMediatorTest"), Some(Arc::downgrade(&component).clone()));
    facade.register_mediator(Arc::new(RwLock::new(mediator)));

    // Assert that has_mediator returns true for the registered mediator
    assert!(facade.has_mediator("facadeHasMediatorTest"));

    // Remove the mediator
    facade.remove_mediator("facadeHasMediatorTest");

    // Assert that has_mediator returns false after removal
    assert!(!facade.has_mediator("facadeHasMediatorTest"));
}

/// Tests the `has_command` method.
///
/// Gets a Facade instance, registers a `FacadeTestCommand`, verifies that
/// `has_command` returns true, removes the command, and verifies that
/// `has_command` returns false.
#[test]
fn test_has_command() {
    // Get a Multiton Facade instance
    let facade = Facade::get_instance("FacadeTestKey9", |k| Facade::new(k));
    // Register the FacadeTestCommand for 'FacadeTestCommand' notifications
    facade.register_command("FacadeTestCommand", || Box::new(FacadeTestCommand::new()));

    // Assert that has_command returns true for the registered command
    assert!(facade.has_command("FacadeTestCommand"));

    // Remove the command
    facade.remove_command("FacadeTestCommand");

    // Assert that has_command returns false after removal
    assert!(!facade.has_command("FacadeTestCommand"));
}

/// Tests the `has_core` and `remove_core` methods.
///
/// Verifies that `has_core` returns false for an unregistered core, creates a
/// Facade instance, verifies that `has_core` returns true, removes the core, and
/// verifies that `has_core` returns false.
#[test]
fn test_has_core_and_remove_core() {
    // Assert that no core exists for the key
    assert!(!Facade::has_core("FacadeTestKey10"));

    // Create a Facade instance
    Facade::get_instance("FacadeTestKey10", |k| Facade::new(k));

    // Assert that the core now exists
    assert!(Facade::has_core("FacadeTestKey10"));

    // Remove the core
    Facade::remove_core("FacadeTestKey10");

    // Assert that the core no longer exists
    assert!(!Facade::has_core("FacadeTestKey10"));
}
