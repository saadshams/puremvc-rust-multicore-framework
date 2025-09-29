use std::any::Any;
use std::sync::{Arc, RwLock};
use puremvc::core::Controller;
use puremvc::interfaces::{IController, IMediator, INotification, IObserver};
use puremvc::patterns::{Mediator, Notification, Observer};

/// A utility struct to hold a test value for observer notifications.
struct Object {
    value: f64,
}

/// Tests initializing an Observer using accessor methods.
///
/// Creates an `Observer` with null arguments, sets the notification method and
/// context via accessors, and notifies it with a `Notification` containing a
/// body value of 10. Success is verified by asserting that the context’s value
/// is set to 10.
#[test]
fn test_observer_accessors() {
    // Create a test object to serve as the notification context
    let object = Object { value: 0.0 };

    // Create a context and a notification closure
    let context = Arc::new(RwLock::new(object));
    let notify = {
        let object = Arc::clone(&context);
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                // Update the context’s value with the notification’s body
                object.write().unwrap().value = *body.downcast_ref::<f64>().unwrap();
            }
        })
    };

    // Create an observer with null arguments
    let mut observer: Box<dyn IObserver> = Box::new(Observer::new(None, None));
    // Set the notification method and context using accessors
    observer.set_notify(Some(notify));
    observer.set_context(Some(context.clone()));

    // Create a notification with a body value of 10
    let vo = Arc::new(10.0);
    let note = Arc::new(Notification::new("ObserverTestNote", Some(vo), None));
    // Notify the observer
    observer.notify_observer(&(note as Arc<dyn INotification>));

    // Assert that the context’s value is 10
    assert_eq!(context.read().unwrap().value, 10.0);
}

/// Tests initializing an Observer using its constructor.
///
/// Creates an `Observer` with a notification method and context via the
/// constructor, and notifies it with a `Notification` containing a body value
/// of 5. Success is verified by asserting that the context’s value is set to 5.
#[test]
fn test_observer_constructor() {
    // Create a test object to serve as the notification context
    let object = Object { value: 0.0 };

    // Create a context and a notification closure
    let context = Arc::new(RwLock::new(object));
    let notify = {
        let object = context.clone();
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                // Update the context’s value with the notification’s body
                object.write().unwrap().value = *body.downcast_ref::<f64>().unwrap();
            }
        })
    };

    // Create an observer with the notification method and context
    let observer = Observer::new(Some(notify), Some(context.clone()));

    // Create a notification with a body value of 5
    let vo = Arc::new(5.0);
    let note = Arc::new(Notification::new("ObserverTestNote", Some(vo), None));
    // Notify the observer
    observer.notify_observer(&(note as Arc<dyn INotification>));

    // Assert that the context’s value is 5
    assert_eq!(context.read().unwrap().value, 5.0);
}

/// Tests the compare_notify_context method of the Observer class.
///
/// Creates an `Observer` with a `Controller` as the context, and tests the
/// `compare_notify_context` method, asserting it returns `true` for the same
/// context and `false` for a different context.
#[test]
fn test_compare_notify_context() {
    // Create a controller to serve as the notification context
    let context: Arc<dyn IController> = Controller::get_instance("ObserverTestKey1", |k| Controller::new(k));

    // Create an observer with the controller context
    let observer = Observer::new(None, Some(Arc::new(context.clone())));

    // Assert that compare_notify_context returns true for the same context
    assert_eq!(observer.compare_notify_context(&(Arc::new(context) as Arc<dyn Any + Send + Sync>)), true);

    // Create a different controller context
    let neg_controller = Controller::get_instance("ObserverTestKey2", |k| Controller::new(k));
    let neg_context = neg_controller as Arc<dyn Any + Send + Sync>;

    // Assert that compare_notify_context returns false for a different context
    assert_eq!(observer.compare_notify_context(&(Arc::new(neg_context))), false);
}

/// Tests the compare_notify_context method with a Mediator context.
///
/// Creates an `Observer` with a `Mediator` as the context, and tests the
/// `compare_notify_context` method, asserting it returns `true` for the same
/// mediator and `false` for a different mediator.
#[test]
fn test_compare_notify_context2() {
    // Create a mediator to serve as the notification context
    let mediator: Arc<RwLock<dyn IMediator>> = Arc::new(RwLock::new(Mediator::new(None, None)));

    // Create an observer with the mediator context
    let observer = Observer::new(None, Some(Arc::new(mediator.clone())));

    // Assert that compare_notify_context returns true for the same mediator
    assert_eq!(observer.compare_notify_context(&(Arc::new(mediator) as Arc<dyn Any + Send + Sync>) ), true);

    // Create a different mediator context
    let neg_mediator = Arc::new(RwLock::new(Mediator::new(None, None)));
    let neg_context = neg_mediator as Arc<dyn Any + Send + Sync>;

    // Assert that compare_notify_context returns false for a different mediator
    assert_eq!(observer.compare_notify_context(&(Arc::new(neg_context))), false);
}

/// Tests the compare_notify_context method with a generic Object context.
///
/// Creates an `Observer` with an `Object` as the context, and tests the
/// `compare_notify_context` method, asserting it returns `true` for the same
/// object context.
#[test]
fn test_compare_notify_context3() {
    // Create a generic object to serve as the notification context
    let object = Arc::new(RwLock::new(Object{value: 0.0}));

    // Create an observer with the object context
    let context = object as Arc<dyn Any + Send + Sync>;
    let observer = Observer::new(None, Some(context.clone()));

    // Assert that compare_notify_context returns true for the same object context
    assert_eq!(observer.compare_notify_context(&context), true);
}
