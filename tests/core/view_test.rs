use std::any::Any;
use std::sync::{Arc, Mutex, Weak};
use puremvc::{IMediator, INotification, INotifier, Mediator, Notification, Observer, View};

pub mod view_test {
    pub const NOTE1: &'static str = "note1";
    pub const NOTE2: &'static str = "note2";
    pub const NOTE3: &'static str = "note3";
    pub const NOTE4: &'static str = "note4";
    pub const NOTE5: &'static str = "note5";
    pub const NOTE6: &'static str = "note6";
}

struct Object {
    test_var: i32,
    last_notification: String,
    on_register_called: bool,
    on_remove_called: bool
}

impl Default for Object {
    fn default() -> Self {
        Self {
            test_var: 0,
            last_notification: String::new(),
            on_register_called: false,
            on_remove_called: false
        }
    }
}

struct ViewTestMediator {
    mediator: Mediator
}

impl ViewTestMediator {
    pub const NAME: &'static str = "ViewTestMediator";
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self { mediator: Mediator::new(Some(Self::NAME), component) }
    }
}

impl INotifier for ViewTestMediator {}

impl IMediator for ViewTestMediator {
    fn name(&self) -> &str { self.mediator.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }
    fn list_notification_interests(&self) -> Vec<String> {
        vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()]
    }
}

struct ViewTestMediator2 {
    mediator: Mediator
}

impl ViewTestMediator2 {
    pub const NAME: &'static str = "ViewTestMediator2";
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {mediator: Mediator::new(Some(Self::NAME), component)}
    }
}

impl INotifier for ViewTestMediator2 {}

impl IMediator for ViewTestMediator2 {
    fn name(&self) -> &str { self.mediator.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE1.to_string(), view_test::NOTE2.to_string()]
    }

    fn handle_notification(&mut self, notification: &Arc<Mutex<dyn INotification>>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| {
                object.lock().unwrap().last_notification = notification.lock().unwrap().name().to_string();
            });
    }
}

struct ViewTestMediator4 {
    mediator: Mediator
}

impl ViewTestMediator4 {
    pub const NAME: &'static str = "ViewTestMediator4";

    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self { mediator: Mediator::new(Some(Self::NAME), component) }
    }
}

impl INotifier for ViewTestMediator4 {}

impl IMediator for ViewTestMediator4 {
    fn name(&self) -> &str {
        self.mediator.name()
    }

    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> {
        self.mediator.notifier()
    }

    fn on_register(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| {
                object.lock().unwrap().on_register_called = true;
            });
    }

    fn on_remove(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| {
                object.lock().unwrap().on_remove_called = true;
            });
    }
}

#[test]
fn test_get_instance() {
    let view = View::get_instance("ViewTestKey1", |k| Arc::new(View::new(k)));

    assert!(Arc::strong_count(&view) > 0, "Expecting instance not null");
}

#[test]
fn test_register_and_notify_observer() {
    let view = View::get_instance("ViewTestKey2", |k| Arc::new(View::new(k)));

    let context = Arc::new(Mutex::new(Object::default()));

    let notify = Arc::new({
        let ctx = context.clone();
        move |notification: &Arc<Mutex<dyn INotification>>| {
            let note = notification.lock().unwrap();
            ctx.lock().unwrap().test_var = *note.body().unwrap().lock().unwrap().downcast_ref::<i32>().unwrap();
        }
    });

    let observer = Observer::new(Some(notify), Some(context.clone()));
    view.register_observer("ObserverTestNote", Arc::new(observer));

    let notification = Notification::new("ObserverTestNote", Some(Arc::new(Mutex::new(10))), None);
    view.notify_observers(&(Arc::new(Mutex::new(notification)) as Arc<Mutex<dyn INotification>>));

    assert_eq!(context.lock().unwrap().test_var, 10);
}

#[test]
fn test_register_and_retrieve_mediator() {
    let view = View::get_instance("ViewTestKey3", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));

    view.register_mediator(Arc::new(Mutex::new(mediator)));
}

#[test]
fn test_has_mediator() {
    let view = View::get_instance("ViewTestKey4", |k| Arc::new(View::new(k)));

    let mediator = Mediator::new(Some("hasMediatorTest"), None);
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    assert_eq!(view.has_mediator("hasMediatorTest"), true, "Expecting view.has_mediator('hasMediatorTest') == true");

    view.remove_mediator("hasMediatorTest");

    assert_eq!(view.has_mediator("hasMediatorTest"), false, "Expecting view.has_mediator('hasMediatorTest') == false");
}

#[test]
fn test_register_and_remove_mediator() {
    let view = View::get_instance("ViewTestKey5", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = Mediator::new(Some("testing"), Some(Arc::downgrade(&component).clone()));

    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let removed_mediator = view.remove_mediator("testing");

    assert!(removed_mediator.is_some());
    assert_eq!(removed_mediator.unwrap().lock().unwrap().name(), "testing", "Expecting removed_mediator.name() == 'testing'");
    assert!(view.retrieve_mediator("testing").is_none(), "Expecting view.retrieve_mediator('testing').is_none()");
}

#[test]
fn test_on_register_and_on_remove() {
    let view = View::get_instance("ViewTestKey6", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = ViewTestMediator4::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    assert!(component.lock().unwrap().on_register_called, "Expecting component.on_register_called == true");

    view.remove_mediator(ViewTestMediator4::NAME);

    assert!(component.lock().unwrap().on_remove_called, "Expecting component.on_remove_called == true");
}

#[test]
fn test_successive_register_and_remove_mediator() {
    let view = View::get_instance("ViewTestKey7", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let retrieved = view
        .retrieve_mediator(ViewTestMediator::NAME)
        .expect("Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_some()");

    assert!(retrieved.lock().unwrap().as_any().is::<ViewTestMediator>(),
        "Expecting mediator is ViewTestMediator");

    view.remove_mediator(ViewTestMediator::NAME);

    assert!(view.retrieve_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_none() == true");

    assert!(view.remove_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.remove_mediator(ViewTestMediator::NAME).is_none() == true");

    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let retrieved = view.retrieve_mediator(ViewTestMediator::NAME)
        .expect("Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_some()");

    assert!(retrieved.lock().unwrap().as_any().is::<ViewTestMediator>(), "Expecting mediator is ViewTestMediator");

    view.remove_mediator(ViewTestMediator::NAME);

    assert!(view.retrieve_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_none() == true");
}

#[test]
fn test_remove_mediator_and_subsequent_notify() {
    let view = View::get_instance("ViewTestKey8", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = ViewTestMediator2::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(Mutex::new(notification)) as Arc<Mutex<dyn INotification>>));

    assert_eq!(component.lock().unwrap().last_notification, view_test::NOTE1)
}