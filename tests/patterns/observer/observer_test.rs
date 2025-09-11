use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{INotification, IObserver, Notification, Observer};

struct Object {
    value: f64,
}

#[test]
fn test_observer_accessors() {
    let object = Arc::new(Mutex::new(Object { value: 0.0 }));

    let context = Arc::downgrade(&object);
    let notify = {
        let ctx = context.clone();
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                if let Some(context_arc) = ctx.upgrade() {
                    context_arc.lock().unwrap().value = *body.downcast_ref::<f64>().unwrap();
                }
            }
        })
    };

    let mut observer = Observer::new(None, None);
    observer.set_notify(Some(notify));
    observer.set_context(Some(object.clone()));

    let vo = Arc::new(10.0);
    let note = Arc::new(Notification::new("ObserverTestNote", Some(vo), None));
    observer.notify_observer(&(note as Arc<dyn INotification>));

    assert_eq!(context.upgrade().unwrap().lock().unwrap().value, 10.0);
}

#[test]
fn test_observer_constructor() {
    let object = Arc::new(Mutex::new(Object { value: 0.0 }));

    let context = Arc::downgrade(&object);
    let notify = {
        let ctx = context.clone();
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                if let Some(context_arc) = ctx.upgrade() {
                    context_arc.lock().unwrap().value = *body.downcast_ref::<f64>().unwrap();
                }
            }
        })
    };

    let observer = Observer::new(Some(notify), Some(object.clone()));

    let vo = Arc::new(5.0);
    let note = Arc::new(Notification::new("ObserverTestNote", Some(vo), None));
    observer.notify_observer(&(note as Arc<dyn INotification>));

    assert_eq!(context.upgrade().unwrap().lock().unwrap().value, 5.0);
}

// #[test]
// fn test_compare_notify_context() {
//     let controller: Arc<dyn IController> = Controller::get_instance("ObserverTestKey1", |k| Arc::new(Controller::new(k)));
//
//     let context: Arc<dyn Any + Send + Sync> = controller.clone(); // do not inline as it will create a temp version
//     let weak = Arc::downgrade(&context);
//     let observer = Observer::new(None, Some(context.clone()));
//
//     assert_eq!(observer.compare_notify_context(&context), true);
//
//     let neg_controller = Controller::get_instance("ObserverTestKey2", |k| Arc::new(Controller::new(k)));
//     let neg_context = neg_controller as Arc<dyn Any + Send + Sync>;
//     // let neg_context_weak = Arc::downgrade(&(neg_context)); // todo
//
//     assert_eq!(observer.compare_notify_context(&neg_context), false);
// }

// #[test]
// fn test_compare_notify_context2() {
//     let mediator: Arc<Mutex<dyn IMediator>> = Arc::new(Mutex::new(Mediator::new(None, None)));
//
//     let context = mediator.clone();
//     let weak = Arc::downgrade(&context);
//     let observer = Observer::new(None, Some(context.clone()));
//
//     assert_eq!(observer.compare_notify_context(context.clone()), true);
//
//     let neg_mediator = Arc::new(Mutex::new(Mediator::new(None, None)));
//     let neg_mediator_context = neg_mediator as Arc<dyn Any + Send + Sync>;
//     // let neg_mediator_weak = Arc::downgrade(&neg_mediator_context); // todo
//
//     assert_eq!(observer.compare_notify_context(&neg_mediator_context), false);
// }

#[test]
fn test_compare_notify_context3() {
    let object = Arc::new(Mutex::new(Object{value: 0.0}));

    let context = object as Arc<dyn Any + Send + Sync>;
    // let context_weak = Arc::downgrade(&(context)); // todo weak
    let observer = Observer::new(None, Some(context.clone()));

    assert_eq!(observer.compare_notify_context(&context), false); // unsupported type
}
