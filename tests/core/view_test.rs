use std::any::{Any, TypeId};
use std::sync::{mpsc, Arc, Mutex, Weak};
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

    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| object.lock().unwrap().last_notification = notification.name().to_string());
    }
}

struct ViewTestMediator3 {
    mediator: Mediator
}

impl ViewTestMediator3 {
    pub const NAME: &'static str = "ViewTestMediator3";
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {mediator: Mediator::new(Some(Self::NAME), component)}
    }
}

impl INotifier for ViewTestMediator3 {}

impl IMediator for ViewTestMediator3 {
    fn name(&self) -> &str { self.mediator.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE3.to_string()]
    }

    fn handle_notification(&mut self, notification: &Arc<dyn INotification>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| object.lock().unwrap().last_notification = notification.name().to_string());
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
            .map(|object| { object.lock().unwrap().on_register_called = true });
    }

    fn on_remove(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| { object.lock().unwrap().on_remove_called = true });
    }
}

struct ViewTestMediator5 {
    mediator: Mediator
}

impl ViewTestMediator5 {
    pub const NAME: &'static str = "ViewTestMediator5";
    pub fn new(component: Option<Weak<dyn Any + Send + Sync>>) -> Self {
        Self {mediator: Mediator::new(Some(Self::NAME), component)}
    }
}

impl INotifier for ViewTestMediator5 {}

impl IMediator for ViewTestMediator5 {
    fn name(&self) -> &str { self.mediator.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE4.to_string(), view_test::NOTE5.to_string()]
    }

    fn handle_notification(&mut self, _notification: &Arc<dyn INotification>) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| object.lock().unwrap().counter += 1 );
    }
}

struct ViewTestMediator6 {
    mediator: Mediator,
    sender: mpsc::Sender<String>,
}

impl ViewTestMediator6 {
    pub const NAME: &'static str = "ViewTestMediator6";
    pub fn new(name: Option<&str>, component: Option<Weak<dyn Any + Send + Sync>>, sender: mpsc::Sender<String>) -> Self {
        Self { mediator: Mediator::new(name, component), sender }
    }
}

impl INotifier for ViewTestMediator6 {}

impl IMediator for ViewTestMediator6 {
    fn name(&self) -> &str { self.mediator.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }

    fn list_notification_interests(&self) -> Vec<String> {
        vec![view_test::NOTE6.to_string()]
    }

    fn handle_notification(&mut self, _notification: &Arc<dyn INotification>) {
        let _ = self.sender.send(self.name().to_string()); // deferred removal (enqueue)
    }

    fn on_remove(&mut self) {
        self.mediator.component()
            .and_then(|weak| weak.upgrade())
            .and_then(|arc| arc.downcast::<Mutex<Object>>().ok())
            .map(|object| object.lock().unwrap().counter += 1);
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
    let notify = {
        let context = context.clone();
        Arc::new(move |notification: &Arc<dyn INotification>| {
            if let Some(body) = notification.body() {
                context.lock().unwrap().test_var = *body.downcast_ref::<i32>().unwrap()
            }
        })
    };

    let observer = Observer::new(Some(notify), Some(context.clone()));
    view.register_observer("ObserverTestNote", Arc::new(observer));

    let notification = Notification::new("ObserverTestNote", Some(Arc::new(10)), None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

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

    // println!("TypeId: {:?}", (&*(retrieved.lock().unwrap())).type_id());
    // println!("Is ViewTestMediator? {}", (&*retrieved.lock().unwrap()).type_id() == TypeId::of::<ViewTestMediator>());

    assert_eq!((&*(retrieved.lock().unwrap())).type_id(), TypeId::of::<ViewTestMediator>());

    view.remove_mediator(ViewTestMediator::NAME);

    assert!(view.retrieve_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_none() == true");

    assert!(view.remove_mediator(ViewTestMediator::NAME).is_none(),
            "Expecting view.remove_mediator(ViewTestMediator::NAME).is_none() == true");

    let mediator = ViewTestMediator::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let retrieved = view.retrieve_mediator(ViewTestMediator::NAME)
        .expect("Expecting view.retrieve_mediator(ViewTestMediator::NAME).is_some()");

    assert_eq!((&*(retrieved.lock().unwrap())).type_id(), TypeId::of::<ViewTestMediator>());

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
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.lock().unwrap().last_notification, view_test::NOTE1);

    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    view.remove_mediator(ViewTestMediator2::NAME);
    assert!(view.retrieve_mediator(ViewTestMediator2::NAME).is_none(),
            "Expecting view.retrieve_mediator(ViewTestMediator2::NAME).is_none() == true");

    component.lock().unwrap().last_notification = String::new();

    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.lock().unwrap().last_notification, view_test::NOTE1);

    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.lock().unwrap().last_notification, view_test::NOTE2);
}

#[test]
fn test_remove_one_of_two_mediators_and_subsequent_notify() {
    let view = View::get_instance("ViewTestKey9", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = ViewTestMediator2::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let mediator = ViewTestMediator3::new(Some(Arc::downgrade(&component).clone()));
    view.register_mediator(Arc::new(Mutex::new(mediator)));

    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.lock().unwrap().last_notification, view_test::NOTE1);

    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.lock().unwrap().last_notification, view_test::NOTE2);

    let notification = Notification::new(view_test::NOTE3, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.lock().unwrap().last_notification, view_test::NOTE3);

    view.remove_mediator(ViewTestMediator2::NAME);

    assert!(view.retrieve_mediator(ViewTestMediator2::NAME).is_none());

    component.lock().unwrap().last_notification = String::new();

    let notification = Notification::new(view_test::NOTE1, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.lock().unwrap().last_notification, view_test::NOTE1);

    let notification = Notification::new(view_test::NOTE2, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_ne!(component.lock().unwrap().last_notification, view_test::NOTE2);

    let notification = Notification::new(view_test::NOTE3, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.lock().unwrap().last_notification, view_test::NOTE3);
}

#[test]
fn test_mediator_reregistration() {
    let view = View::get_instance("ViewTestKey10", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let mediator = Arc::new(Mutex::new(ViewTestMediator5::new(Some(Arc::downgrade(&component).clone()))));

    view.register_mediator(Arc::clone(&(mediator.clone() as Arc<Mutex<dyn IMediator>>)));
    view.register_mediator(Arc::clone(&(mediator as Arc<Mutex<dyn IMediator>>)));

    let notification = Notification::new(view_test::NOTE5, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    assert_eq!(component.lock().unwrap().counter, 1);

    view.remove_mediator(ViewTestMediator5::NAME);
    assert!(view.retrieve_mediator(ViewTestMediator5::NAME).is_none());

    component.lock().unwrap().counter = 0;
    let notification = Notification::new(view_test::NOTE5, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));
    assert_eq!(component.lock().unwrap().counter, 0);
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
    let view = View::get_instance("ViewTestKey11", |k| Arc::new(View::new(k)));

    let component = Arc::new(Mutex::new(Object::default()));
    let weak = Arc::downgrade(&component);

    let (sender, receiver) = mpsc::channel::<String>();

    let name = format!("{}/1", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/2", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/3", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/4", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/5", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/6", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/7", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let name = format!("{}/8", ViewTestMediator6::NAME);
    let mediator = ViewTestMediator6::new(Some(&name), Some(weak.clone()), sender.clone());
    view.register_mediator(Arc::clone(&(Arc::new(Mutex::new(mediator)) as Arc<Mutex<dyn IMediator>>)));

    let notification = Notification::new(view_test::NOTE6, None, None);
    view.notify_observers(&(Arc::new(notification) as Arc<dyn INotification>));

    while let Ok(name) = receiver.try_recv() {
        view.remove_mediator(&name);
    }

    assert_eq!(component.lock().unwrap().counter, 8);

    component.lock().unwrap().counter = 0;
    let notification = Arc::new(Notification::new(view_test::NOTE6, None, None));
    view.notify_observers(&(notification as Arc<dyn INotification>));

    assert_eq!(component.lock().unwrap().counter, 0);
}
