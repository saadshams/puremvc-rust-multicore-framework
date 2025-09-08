use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{Controller, IController, IMediator, INotification, IObserver, Mediator, Notification, Observer};

struct Object {
    value: f64,
}

#[test]
fn test_observer_accessors() {
    let context = Arc::new(Mutex::new(Object{value: 0.0}));
    let notify = Arc::new({
        let ctx = context.clone();
        move |notification: &Arc<Mutex<dyn INotification>>| {
            let note = notification.lock().unwrap();
            ctx.lock().unwrap().value = *note.body().unwrap().lock().unwrap().downcast_ref::<f64>().unwrap();
        }
    });

    let mut observer = Observer::new(None, None);
    observer.set_notify(Some(notify));
    observer.set_context(Some(context.clone()));

    let vo = Arc::new(Mutex::new(10.0));
    let note: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new("ObserverTestNote", Some(vo), None)));
    observer.notify_observer(&note);

    assert_eq!(context.lock().unwrap().value, 10.0);
}

#[test]
fn test_observer_constructor() {
    let context = Arc::new(Mutex::new(Object{value: 0.0}));
    let notify = Arc::new({
        let ctx = context.clone();
        move |notification: &Arc<Mutex<dyn INotification>>| {
            let note = notification.lock().unwrap();
            ctx.lock().unwrap().value = *note.body().unwrap().lock().unwrap().downcast_ref::<f64>().unwrap();
        }
    });

    let observer = Observer::new(Some(notify), Some(context.clone()));

    let vo = Arc::new(Mutex::new(5.0));
    let note: Arc<Mutex<dyn INotification>> = Arc::new(Mutex::new(Notification::new("ObserverTestNote", Some(vo), None)));
    observer.notify_observer(&note);

    assert_eq!(context.lock().unwrap().value, 5.0);
}

#[test]
fn test_compare_notify_context() {
    let controller: Arc<dyn IController> = Controller::get_instance("ObserverTestKey1", |k| Arc::new(Controller::new(k)));
    let controller_any: Arc<dyn Any + Send + Sync> = Arc::new(controller.clone());

    let observer = Observer::new(None, Some(controller_any.clone()));

    assert_eq!(observer.compare_notify_context(&controller_any), true);

    let neg_controller = Controller::get_instance("ObserverTestKey2", |k| Arc::new(Controller::new(k)));
    let neg_controller_any: Arc<dyn Any + Send + Sync> = Arc::new(neg_controller.clone());

    assert_eq!(observer.compare_notify_context(&neg_controller_any), false);
}

#[test]
fn test_compare_notify_context2() {
    let mediator: Arc<Mutex<dyn IMediator>> = Arc::new(Mutex::new(Mediator::new(None, None)));
    let mediator_any: Arc<dyn Any + Send + Sync> = Arc::new(mediator.clone());

    let observer = Observer::new(None, Some(mediator_any.clone()));

    assert_eq!(observer.compare_notify_context(&mediator_any), true);

    let neg_mediator: Arc<Mutex<dyn IMediator>> = Arc::new(Mutex::new(Mediator::new(None, None)));
    let neg_mediator_any: Arc<dyn Any + Send + Sync> = Arc::new(neg_mediator.clone());

    assert_eq!(observer.compare_notify_context(&neg_mediator_any), false);
}
