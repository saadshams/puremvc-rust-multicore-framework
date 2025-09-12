use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{Controller, IController, IMediator, INotification, IObserver, Mediator, Notification, Observer};

struct Object {
    value: f64,
}

#[test]
fn test_observer_accessors() {
    let object = Object { value: 0.0 };

    let context = Arc::new(Mutex::new(object));
    let notify = {
        let object = Arc::clone(&context);
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                object.lock().unwrap().value = *body.downcast_ref::<f64>().unwrap();
            }
        })
    };

    let mut observer = Observer::new(None, None);
    observer.set_notify(Some(notify));
    observer.set_context(Some(context.clone()));

    let vo = Arc::new(10.0);
    let note = Arc::new(Notification::new("ObserverTestNote", Some(vo), None));
    observer.notify_observer(&(note as Arc<dyn INotification>));

    assert_eq!(context.lock().unwrap().value, 10.0);
}

#[test]
fn test_observer_constructor() {
    let object = Object { value: 0.0 };

    let context = Arc::new(Mutex::new(object));
    let notify = {
        let object = context.clone();
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                object.lock().unwrap().value = *body.downcast_ref::<f64>().unwrap();
            }
        })
    };

    let observer = Observer::new(Some(notify), Some(context.clone()));

    let vo = Arc::new(5.0);
    let note = Arc::new(Notification::new("ObserverTestNote", Some(vo), None));
    observer.notify_observer(&(note as Arc<dyn INotification>));

    assert_eq!(context.lock().unwrap().value, 5.0);
}

#[test]
fn test_compare_notify_context() {
    let context: Arc<dyn IController> = Controller::get_instance("ObserverTestKey1", |k| Controller::new(k));

    let observer = Observer::new(None, Some(Arc::new(context.clone())));

    assert_eq!(observer.compare_notify_context(&(Arc::new(context) as Arc<dyn Any + Send + Sync>)), true);

    let neg_controller = Controller::get_instance("ObserverTestKey2", |k| Controller::new(k));
    let neg_context = neg_controller as Arc<dyn Any + Send + Sync>;

    assert_eq!(observer.compare_notify_context(&(Arc::new(neg_context))), false);
}

#[test]
fn test_compare_notify_context2() {
    let mediator: Arc<Mutex<dyn IMediator>> = Arc::new(Mutex::new(Mediator::new(None, None)));

    let observer = Observer::new(None, Some(Arc::new(mediator.clone())));

    assert_eq!(observer.compare_notify_context(&(Arc::new(mediator) as Arc<dyn Any + Send + Sync>) ), true);

    let neg_mediator = Arc::new(Mutex::new(Mediator::new(None, None)));
    let neg_context = neg_mediator as Arc<dyn Any + Send + Sync>;

    assert_eq!(observer.compare_notify_context(&(Arc::new(neg_context))), false);
}

#[test]
fn test_compare_notify_context3() {
    let object = Arc::new(Mutex::new(Object{value: 0.0}));

    let context = object as Arc<dyn Any + Send + Sync>;
    let observer = Observer::new(None, Some(context.clone()));

    assert_eq!(observer.compare_notify_context(&context), true);
}
