use std::any::Any;
use std::process::Command;
use std::sync::{Arc, Mutex};
use puremvc::{Controller, ICommand, IController, IMediator, IModel, INotification, INotifier, IObserver, IProxy, IView, Mediator, Model, Notification, Notifier, Proxy, SimpleCommand, View};

#[derive(Debug, PartialEq, Eq)]
enum State { Allocated, Released }
struct Resource { state: State }

struct TestMediator {
    mediator: Mediator,
    resource: Arc<Mutex<Resource>>
}
// ======================================================================
impl TestMediator {
    fn new(resource: Arc<Mutex<Resource>>) -> Self {
        Self { mediator: Mediator::new(None, None), resource }
    }
}

impl Drop for TestMediator {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released }
}

impl INotifier for TestMediator {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self as &mut dyn INotifier
    }
}
impl IMediator for TestMediator {
    fn name(&self) -> &str { self.mediator.name() }
    // fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }
}

struct TestView {
    _key: String,
    view: Option<Arc<dyn IView>>,
    resource: Arc<Mutex<Resource>>,
}

impl TestView {
    fn new(key: &str, resource: Arc<Mutex<Resource>>) -> Self {
        Self {
            _key: key.to_string(),
            view: None,
            resource
        }
    }
}

impl Drop for TestView {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released; }
}

impl IView for TestView {
    fn initialize_view(&mut self) {

    }

    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>) {
        if let Some(view) = &self.view { view.register_observer(notification_name, observer) }
    }

    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>) {
        if let Some(view) = &self.view { view.remove_observer(notification_name, context) }
    }

    fn notify_observers(&self, notification: &Arc<dyn INotification>) {
        if let Some(view) = &self.view { view.notify_observers(notification) }
    }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) {
        if let Some(view) = &self.view { view.register_mediator(mediator) }
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.view.as_ref()?.retrieve_mediator(mediator_name)
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.view.as_ref().map_or(false, |v| v.has_mediator(mediator_name))
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> {
        self.view.as_ref()?.remove_mediator(mediator_name)
    }
}


#[test]
fn test_mediator() {
    let resource = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    {
        let mediator = TestMediator::new(resource.clone());
        drop(mediator);
        assert_eq!(resource.lock().unwrap().state, State::Released);
    }
}

#[test]
fn test_view() {
    let resource = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    {
        let view = TestView::new("TestView", resource.clone());
        let mediator = TestMediator::new(resource.clone());
        view.register_mediator(Arc::new(Mutex::new(mediator)));
        View::remove_view("TestView");
        drop(view);
        assert_eq!(resource.lock().unwrap().state, State::Released);
    }
}

// ======================================================================
struct TestProxy {
    proxy: Proxy,
    resource: Arc<Mutex<Resource>>
}

impl TestProxy {
    fn new(resource: Arc<Mutex<Resource>>) -> Self {
        Self{proxy: Proxy::new(None, None), resource}
    }
}

impl Drop for TestProxy {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released }
}

impl INotifier for TestProxy {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self as &mut dyn INotifier
    }
}

impl IProxy for TestProxy {
    fn name(&self) -> &str { self.proxy.name() }
    // fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.proxy.notifier() }
}

struct TestModel {
    model: Arc<dyn IModel>,
    resource: Arc<Mutex<Resource>>
}

impl TestModel {
    fn new(key: &str, resource: Arc<Mutex<Resource>>) -> Self {
        Self { model: Model::get_instance(key, |k| Model::new(k)), resource }
    }
}

impl Drop for TestModel {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released }
}

impl IModel for TestModel {
    fn initialize_model(&mut self) {}

    fn register_proxy(&self, proxy: Arc<Mutex<dyn IProxy>>) { self.model.register_proxy(proxy) }
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> { self.model.retrieve_proxy(proxy_name) }
    fn has_proxy(&self, proxy_name: &str) -> bool { self.model.has_proxy(proxy_name) }
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<Mutex<dyn IProxy>>> { self.model.remove_proxy(proxy_name) }
}

#[test]
fn test_proxy() {
    let resource = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    {
        let proxy = TestProxy::new(resource.clone());
        drop(proxy);
        assert_eq!(resource.lock().unwrap().state, State::Released);
    }
}

#[test]
fn test_model() {
    let resource1 = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    let resource2 = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    {
        let model = TestModel::new("TestModel", resource1.clone());
        let proxy = TestProxy::new(resource2.clone());
        model.register_proxy(Arc::new(Mutex::new(proxy)));
        Model::remove_model("TestModel");
        drop(model);
        assert_eq!(resource1.lock().unwrap().state, State::Released);
        assert_eq!(resource2.lock().unwrap().state, State::Released);
    }
}

// ======================================================================
struct TestCommand {
    command: SimpleCommand
}

impl TestCommand {
    fn new() -> Self {
        Self {
            command: SimpleCommand::new()
        }
    }
}

impl Drop for TestCommand {
    fn drop(&mut self) {
    }
}

impl INotifier for TestCommand {
    fn notifier(&mut self) -> &mut dyn INotifier {
        self.command.notifier()
    }
}
impl ICommand for TestCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {}
}

struct TestController {
    controller: Arc<dyn IController>,
    resource: Arc<Mutex<Resource>>
}

impl TestController {
    pub fn new(key: &str, resource: Arc<Mutex<Resource>>) -> Self {
        Self { controller: Controller::get_instance(key, |k| Controller::new(k)), resource }
    }
}

impl Drop for TestController {
    fn drop(&mut self) {
        self.resource.lock().unwrap().state = State::Released
    }
}

impl IController for TestController {
    fn initialize_controller(&mut self) {}

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) { self.controller.register_command(&notification_name, factory) }
    fn execute_command(&self, notification: &Arc<dyn INotification>) { self.controller.execute_command(&notification); }
    fn has_command(&self, notification_name: &str) -> bool { self.controller.has_command(notification_name) }
    fn remove_command(&self, notification_name: &str) { self.controller.remove_command(notification_name); }
}

// ======================================================================

#[test]
fn test_command() {
    let resource = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    {
        let command = TestCommand::new();
        drop(command);
        // assert_eq!(resource.lock().unwrap().state, State::Released);
    }
}

#[test]
fn test_controller() {
    let resource1 = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    let resource2 = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    // let resource3 = Arc::new(Mutex::new(Resource{state: State::Allocated}));

    {
        let view = View::get_instance("TestController", |k| TestView::new(k, resource1.clone()));

        let controller = TestController::new("TestController", resource2.clone());

        controller.register_command("TestCommand", || Box::new(TestCommand::new()) );

        let notification = Arc::new(Notification::new("TestCommand", None, None));
        controller.execute_command(&(notification as Arc<dyn INotification>));

        Controller::remove_controller("TestController");
        View::remove_view("TestController");
        drop(view);
        drop(controller);

        assert_eq!(resource1.lock().unwrap().state, State::Released);
        assert_eq!(resource2.lock().unwrap().state, State::Released);
        // assert_eq!(resource3.lock().unwrap().state, State::Released);
    }
}