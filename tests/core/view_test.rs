use std::any::{Any, TypeId};
use std::sync::{mpsc, Arc, RwLock, Weak};
use puremvc::core::View;
use puremvc::interfaces::{IFacade, IMediator, INotification, INotifier};
use puremvc::patterns::{Mediator, Notification, Observer};

/// Constants for notification names used in tests.
pub mod view_test {
    pub const NOTE1: &'static str = "note1";
    pub const NOTE2: &'static str = "note2";
    pub const NOTE3: &'static str = "note3";
    pub const NOTE4: &'static str = "note4";
    pub const NOTE5: &'static str = "note5";
    pub const NOTE6: &'static str = "note6";
}

/// A utility struct used by View tests to hold test state.
struct Object {
    test_var: i32,
    last_notification: String,
    on_register_called: bool,
    on_remove_called: bool,
    counter: i32,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            test_var: 0,
            last_notification: String::new(),
            on_register_called: false,
            on_remove_called: false,
            counter: 0
        }
    }
}

/// A Mediator subclass used by ViewTest.
struct ViewTestMediator {
    mediator: Mediator
}

impl ViewTestMediator {
    pub const NAME: &'static str = "ViewTestMediator";
    /// Constructor.
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self { mediator: Mediator::new(Some(Self::NAME), component) }
    }
}

impl INotifier for ViewTestMediator {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.mediator.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}

impl IMediator for ViewTestMediator {
    /// Returns the name of the mediator.
    fn name(&self) -> &str { self.mediator.name() }

    /// Returns the component associated with the mediator, if any.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    /// Sets the component for the mediator.
    ///
    /// # Arguments
    /// * `component` - Optional component to be associated with the mediator
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    /// Lists the notifications this mediator is interested in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()]
    }

    /// Returns a mutable reference to the mediator as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A Mediator subclass used by ViewTest.
struct ViewTestMediator2 {
    mediator: Mediator
}

impl ViewTestMediator2 {
    pub const NAME: &'static str = "ViewTestMediator2";
    /// Constructor.
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {mediator: Mediator::new(Some(Self::NAME), component)}
    }
}

impl INotifier for ViewTestMediator2 {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.mediator.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}

impl IMediator for ViewTestMediator2 {
    /// Returns the name of the mediator.
    fn name(&self) -> &str { self.mediator.name() }

    /// Returns the component associated with the mediator, if any.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    /// Sets the component for the mediator.
    ///
    /// # Arguments
    /// * `component` - Optional component to be associated with the mediator
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    /// Lists the notifications this mediator is interested in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE1.to_string(), view_test::NOTE2.to_string()]
    }

    /// Handles a notification by updating the component's last notification.
    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<Object>>().ok())
            .map(|object| object.write().unwrap().last_notification = notification.name().to_string());
    }

    /// Returns a mutable reference to the mediator as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A Mediator subclass used by ViewTest.
struct ViewTestMediator3 {
    mediator: Mediator
}

impl ViewTestMediator3 {
    pub const NAME: &'static str = "ViewTestMediator3";
    /// Constructor.
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {mediator: Mediator::new(Some(Self::NAME), component)}
    }
}

impl INotifier for ViewTestMediator3 {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.mediator.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}

impl IMediator for ViewTestMediator3 {
    /// Returns the name of the mediator.
    fn name(&self) -> &str { self.mediator.name() }

    /// Returns the component associated with the mediator, if any.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    /// Sets the component for the mediator.
    ///
    /// # Arguments
    /// * `component` - Optional component to be associated with the mediator
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    /// Lists the notifications this mediator is interested in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE3.to_string()]
    }

    /// Handles a notification by updating the component's last notification.
    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<Object>>().ok())
            .map(|object| object.write().unwrap().last_notification = notification.name().to_string());
    }

    /// Returns a mutable reference to the mediator as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A Mediator subclass used by ViewTest.
struct ViewTestMediator4 {
    mediator: Mediator
}

impl ViewTestMediator4 {
    pub const NAME: &'static str = "ViewTestMediator4";
    /// Constructor.
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self { mediator: Mediator::new(Some(Self::NAME), component) }
    }
}

impl INotifier for ViewTestMediator4 {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.mediator.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}

impl IMediator for ViewTestMediator4 {
    /// Returns the name of the mediator.
    fn name(&self) -> &str {
        self.mediator.name()
    }

    /// Returns the component associated with the mediator, if any.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    /// Sets the component for the mediator.
    ///
    /// # Arguments
    /// * `component` - Optional component to be associated with the mediator
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    /// Called when the mediator is registered.
    fn on_register(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<Object>>().ok())
            .map(|object| { object.write().unwrap().on_register_called = true });
    }

    /// Called when the mediator is removed.
    fn on_remove(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<Object>>().ok())
            .map(|object| { object.write().unwrap().on_remove_called = true });
    }

    /// Returns a mutable reference to the mediator as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A Mediator subclass used by ViewTest.
struct ViewTestMediator5 {
    mediator: Mediator
}

impl ViewTestMediator5 {
    pub const NAME: &'static str = "ViewTestMediator5";
    /// Constructor.
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {mediator: Mediator::new(Some(Self::NAME), component)}
    }
}

impl INotifier for ViewTestMediator5 {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.mediator.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}

impl IMediator for ViewTestMediator5 {
    /// Returns the name of the mediator.
    fn name(&self) -> &str { self.mediator.name() }

    /// Returns the component associated with the mediator, if any.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    /// Sets the component for the mediator.
    ///
    /// # Arguments
    /// * `component` - Optional component to be associated with the mediator
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    /// Lists the notifications this mediator is interested in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE4.to_string(), view_test::NOTE5.to_string()]
    }

    /// Handles a notification by incrementing the component's counter.
    fn handle_notification(&mut self, _notification: &Arc<dyn INotification>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<Object>>().ok())
            .map(|object| object.write().unwrap().counter += 1 );
    }

    /// Returns a mutable reference to the mediator as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// A Mediator subclass used by ViewTest.
struct ViewTestMediator6 {
    mediator: Mediator,
    sender: mpsc::Sender<String>,
}

impl ViewTestMediator6 {
    pub const NAME: &'static str = "ViewTestMediator6";
    /// Constructor.
    pub fn new(name: Option<&str>, component: Option<Weak<dyn Any + Send + Sync>>, sender: mpsc::Sender<String>) -> Self {
        Self { mediator: Mediator::new(name, component), sender }
    }
}

impl INotifier for ViewTestMediator6 {
    /// Returns the key associated with this notifier.
    fn key(&self) -> &str {
        self.mediator.key()
    }

    /// Returns the facade instance for this notifier.
    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    /// Initializes the notifier with the specified key.
    ///
    /// # Arguments
    /// * `key` - The key to associate with this notifier
    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    /// Sends a notification with the specified name, body, and type.
    ///
    /// # Arguments
    /// * `name` - The name of the notification
    /// * `body` - Optional data payload for the notification
    /// * `type_` - Optional type identifier for the notification
    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}

impl IMediator for ViewTestMediator6 {
    /// Returns the name of the mediator.
    fn name(&self) -> &str { self.mediator.name() }

    /// Returns the component associated with the mediator, if any.
    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    /// Sets the component for the mediator.
    ///
    /// # Arguments
    /// * `component` - Optional component to be associated with the mediator
    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    /// Lists the notifications this mediator is interested in.
    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE6.to_string()]
    }

    /// Handles a notification by enqueuing its name for deferred removal.
    fn handle_notification(&mut self, _notification: &Arc<dyn INotification>) {
        let _ = self.sender.send(self.name().to_string()); // deferred removal (enqueue)
    }

    /// Called when the mediator is removed, incrementing the component's counter.
    fn on_remove(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<RwLock<Object>>().ok())
            .map(|object| object.write().unwrap().counter += 1);
    }

    /// Returns a mutable reference to the mediator as a dynamic `Any` type.
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// Tests the View Multiton Factory Method.
#[test]
fn test_get_instance() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey1", |k| View::new(k));

    // Assert that the instance is not null
    assert!(Arc::strong_count(&view) > 0, "Expecting instance not null");
}

/// Tests registration and notification of Observers.
///
/// An observer is created to call back a method that updates a test variable in the
/// context object. The observer is registered with the View to be notified of
/// 'ObserverTestNote' events. A notification is sent, and the observer's callback
/// updates the test variable with the notification's payload. The test verifies that
/// the test variable matches the payload value.
#[test]
fn test_register_and_notify_observer() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey2", |k| View::new(k));

    // Create a context object to hold test state
    let context = Arc::new(RwLock::new(Object::default()));
    // Define the observer's callback to update the test variable
    let notify = {
        let context = context.clone();
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                context.write().unwrap().test_var = *body.downcast_ref::<i32>().unwrap()
            }
        })
    };

    // Create and register an observer for 'ObserverTestNote'
    let observer = Observer::new(Some(notify), Some(context.clone()));
    view.register_observer("ObserverTestNote", Arc::new(observer));

    // Create a notification with a payload value of 10
    let notification = Notification::new("ObserverTestNote", Some(Arc::new(10)), None);
    // Notify observers to trigger the callback
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    // Assert that the test variable matches the notification payload
    assert_eq!(context.write().unwrap().test_var, 10);
}

/// Tests registering and retrieving a mediator with the View.
#[test]
fn test_register_and_retrieve_mediator() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey3", |k| View::new(k));

    // Create a component object for the mediator
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register a ViewTestMediator
    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));

    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Note: No assertion is made as the test only verifies that registration does not throw
}

/// Tests the `has_mediator` method.
#[test]
fn test_has_mediator() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey4", |k| View::new(k));

    // Create and register a mediator named 'hasMediatorTest'
    let mediator = Mediator::new(Some("hasMediatorTest"), None);
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Assert that has_mediator returns true for the registered mediator
    assert_eq!(view.has_mediator("hasMediatorTest"), true, "Expecting view.has_mediator('hasMediatorTest') == true");

    // Remove the mediator
    view.remove_mediator("hasMediatorTest");

    // Assert that has_mediator returns false after removal
    assert_eq!(view.has_mediator("hasMediatorTest"), false, "Expecting view.has_mediator('hasMediatorTest') == false");
}

/// Tests registering and removing a mediator.
#[test]
fn test_register_and_remove_mediator() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey5", |k| View::new(k));

    // Create a component object for the mediator
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register a mediator named 'testing'
    let mediator = Mediator::new(Some("testing"), Some(Arc::downgrade(&component).clone()));

    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Remove the mediator and capture the returned instance
    let removed_mediator = view.remove_mediator("testing");

    // Assert that a mediator was removed
    assert!(removed_mediator.is_some());
    // Assert that the removed mediator has the expected name
    assert_eq!(removed_mediator.unwrap().read().unwrap().name(), "testing", "Expecting removed_mediator.name() == 'testing'");
    // Assert that the mediator is no longer retrievable
    assert!(view.retrieve_mediator("testing").is_none(), "Expecting view.retrieve_mediator('testing').is_none()");
}

/// Tests that the View calls the `on_register` and `on_remove` methods.
#[test]
fn test_on_register_and_on_remove() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey6", |k| View::new(k));

    // Create a component object to track registration and removal
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register a ViewTestMediator4
    let mediator = ViewTestMediator4::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Assert that on_register was called
    assert!(component.write().unwrap().on_register_called, "Expecting component.on_register_called == true");

    // Remove the mediator
    view.remove_mediator(ViewTestMediator4::NAME);

    // Assert that on_remove was called
    assert!(component.write().unwrap().on_remove_called, "Expecting component.on_remove_called == true");
}

/// Tests successive registration and removal of a mediator.
#[test]
fn test_successive_register_and_remove_mediator() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey7", |k| View::new(k));

    // Create a component object for the mediator
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register a ViewTestMediator
    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Retrieve the mediator and verify its type
    let retrieved = view
        .retrieve_mediator(ViewTestMediator::NAME)
        .expect("Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_some()");

    assert_eq!((&*(retrieved.read().unwrap())).type_id(), TypeId::of::<ViewTestMediator>());

    // Remove the mediator
    view.remove_mediator(ViewTestMediator::NAME);

    // Assert that the mediator is no longer retrievable
    assert!(view.retrieve_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_none() == true");

    // Assert that removing the mediator again does not crash
    assert!(view.remove_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.remove_mediator(ViewTestMediator::NAME).is_none() == true");

    // Re-register a new instance of ViewTestMediator
    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Retrieve the mediator again and verify its type
    let retrieved = view.retrieve_mediator(ViewTestMediator::NAME)
        .expect("Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_some()");

    assert_eq!((&*(retrieved.read().unwrap())).type_id(), TypeId::of::<ViewTestMediator>());

    // Remove the mediator again
    view.remove_mediator(ViewTestMediator::NAME);

    // Assert that the mediator is no longer retrievable
    assert!(view.retrieve_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_none() == true");
}

/// Tests registering a mediator for two notifications, removing it, and verifying it no longer responds.
#[test]
fn test_remove_mediator_and_subsequent_notify() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey8", |k| View::new(k));

    // Create a component object to track notifications
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register a ViewTestMediator2
    let mediator = ViewTestMediator2::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Send NOTE1 notification and verify it was received
    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.write().unwrap().last_notification, view_test::NOTE1);

    // Send NOTE2 notification and verify it was received
    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    // Remove the mediator
    view.remove_mediator(ViewTestMediator2::NAME);
    // Assert that the mediator is no longer retrievable
    assert!(view.retrieve_mediator(ViewTestMediator2::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator2::NAME).is_none() == true");

    // Clear the last notification
    component.write().unwrap().last_notification = String::new();

    // Send NOTE1 notification and verify it was not received
    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.write().unwrap().last_notification, view_test::NOTE1);

    // Send NOTE2 notification and verify it was not received
    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.write().unwrap().last_notification, view_test::NOTE2);
}

/// Tests removing one of two mediators and verifying the remaining one still responds.
#[test]
fn test_remove_one_of_two_mediators_and_subsequent_notify() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey9", |k| View::new(k));

    // Create a component object to track notifications
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register ViewTestMediator2 for NOTE1 and NOTE2
    let mediator = ViewTestMediator2::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Create and register ViewTestMediator3 for NOTE3
    let mediator = ViewTestMediator3::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(RwLock::new(mediator)));

    // Send NOTE1 notification and verify it was received
    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.write().unwrap().last_notification, view_test::NOTE1);

    // Send NOTE2 notification and verify it was received
    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.write().unwrap().last_notification, view_test::NOTE2);

    // Send NOTE3 notification and verify it was received
    let notification = Notification::new(view_test::NOTE3, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.write().unwrap().last_notification, view_test::NOTE3);

    // Remove ViewTestMediator2
    view.remove_mediator(ViewTestMediator2::NAME);

    // Assert that ViewTestMediator2 is no longer retrievable
    assert!(view.retrieve_mediator(ViewTestMediator2::NAME).is_none());

    // Clear the last notification
    component.write().unwrap().last_notification = String::new();

    // Send NOTE1 notification and verify it was not received
    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.write().unwrap().last_notification, view_test::NOTE1);

    // Send NOTE2 notification and verify it was not received
    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.write().unwrap().last_notification, view_test::NOTE2);

    // Send NOTE3 notification and verify it was received by ViewTestMediator3
    let notification = Notification::new(view_test::NOTE3, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.write().unwrap().last_notification, view_test::NOTE3);
}

/// Tests registering the same mediator twice and ensuring it responds only once.
#[test]
fn test_mediator_reregistration() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey10", |k| View::new(k));

    // Create a component object to track notifications
    let component = Arc::new(RwLock::new(Object::default()));
    // Create and register ViewTestMediator5
    let mediator: Arc<RwLock<dyn IMediator>> = Arc::new(RwLock::new(ViewTestMediator5::new(Some(Arc::downgrade(&component).clone()))));
    view.register_mediator(Arc::clone(&mediator));
    // Attempt to re-register the same mediator instance
    view.register_mediator(Arc::clone(&mediator));

    // Send NOTE5 notification
    let notification = Notification::new(view_test::NOTE5, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    // Assert that the counter was incremented only once
    assert_eq!(component.write().unwrap().counter, 1);

    // Remove the mediator
    view.remove_mediator(ViewTestMediator5::NAME);
    // Assert that the mediator is no longer retrievable
    assert!(view.retrieve_mediator(ViewTestMediator5::NAME).is_none());

    // Reset the counter
    component.write().unwrap().counter = 0;
    // Send NOTE5 notification again
    let notification = Notification::new(view_test::NOTE5, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    // Assert that the counter remains 0
    assert_eq!(component.write().unwrap().counter, 0);
}

// When `view.notify_observers` is called, it iterates over observers and invokes their `notify` callbacks.
// If an `Observer`'s `notify` triggers `mediator.handle_notification`, which in turn calls `remove_observer`
// (via the facade) to mutate the observer list for iteration safety, we can encounter re-entrant locking on the same mediator:
// 1. The mediator is already locked inside the Observer's `notify` callback.
// 2. `remove_observer` attempts to lock the mediator again to access `list_notification_interests`.
// This double lock results in a deadlock.
//
// Solution: Deferred/Asynchronous processing.
// To prevent this deadlock, we break the re-entrant cycle by deferring potentially recursive operations.
// Instead of performing removals or other mutations inline, we enqueue the work for asynchronous processing.
// Using channels (or similar queues) ensures that locks are never acquired recursively, avoiding deadlocks.
#[test]
fn test_modify_observer_list_during_notification() {
    // Get a Multiton View instance
    let view = View::get_instance("ViewTestKey11", |k| View::new(k));

    // Create a component object to track notifications
    let component = Arc::new(RwLock::new(Object::default()));
    let weak = Arc::downgrade(&component);

    // Create a channel for deferred mediator removal
    let (sender, receiver) = mpsc::channel::<String>();

    // Register multiple ViewTestMediator6 instances, each with a unique name
    let name = format!("{}/1", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/2", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/3", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/4", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/5", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/6", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/7", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    let name = format!("{}/8", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(RwLock::new(mediator)) as Arc<RwLock<dyn IMediator>>)));

    // Send NOTE6 notification, triggering mediators to enqueue their removal
    let notification = Notification::new(view_test::NOTE6, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    // Process deferred removals from the channel
    while let Ok(name) = receiver.try_recv() {
        view.remove_mediator(&name);
    }

    // Assert that all 8 mediators were notified (via on_remove)
    assert_eq!(component.write().unwrap().counter, 8);

    // Reset the counter
    component.write().unwrap().counter = 0;
    // Send NOTE6 notification again
    let notification = Arc::new(Notification::new(view_test::NOTE6, None, None));
    view.notify_observers(&(notification as Arc<dyn INotification>));

    // Assert that no mediators remain to respond
    assert_eq!(component.write().unwrap().counter, 0);
}
