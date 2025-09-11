use std::any::Any;
use std::sync::{Arc, Mutex};
use puremvc::{Controller, ICommand, IController, IMediator, IModel, INotification, INotifier, IObserver, IProxy, IView, Mediator, Model, Notification, Proxy, SimpleCommand, View};

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

impl INotifier for TestMediator {}
impl IMediator for TestMediator {
    fn name(&self) -> &str { self.mediator.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.mediator.notifier() }
}

struct TestView {
    view: Arc<dyn IView>,
    resource: Arc<Mutex<Resource>>,
}

impl TestView {
    fn new(key: &str, resource: Arc<Mutex<Resource>>) -> Self {
        Self { view: View::get_instance(key, |k| Arc::new(View::new(k))), resource }
    }
}

impl Drop for TestView {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released; }
}

impl IView for TestView {
    fn register_observer(&self, notification_name: &str, observer: Arc<dyn IObserver>) { self.view.register_observer(notification_name, observer) }
    fn remove_observer(&self, notification_name: &str, context: Arc<dyn Any + Send + Sync>) { self.view.remove_observer(notification_name, context) }
    fn notify_observers(&self, notification: &Arc<dyn INotification>) { self.view.notify_observers(notification) }

    fn register_mediator(&self, mediator: Arc<Mutex<dyn IMediator>>) { self.view.register_mediator(mediator) }
    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> { self.view.retrieve_mediator(mediator_name) }
    fn has_mediator(&self, mediator_name: &str) -> bool { self.view.has_mediator(mediator_name) }
    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<Mutex<dyn IMediator>>> { self.view.remove_mediator(mediator_name) }
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

impl INotifier for TestProxy {}
impl IProxy for TestProxy {
    fn name(&self) -> &str { self.proxy.name() }
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.proxy.notifier() }
}

struct TestModel {
    model: Arc<dyn IModel>,
    resource: Arc<Mutex<Resource>>
}

impl TestModel {
    fn new(key: &str, resource: Arc<Mutex<Resource>>) -> Self {
        Self { model: Model::get_instance(key, |k| Arc::new(Model::new(k))), resource }
    }
}

impl Drop for TestModel {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released }
}

impl IModel for TestModel {
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
    command: SimpleCommand,
    resource: Arc<Mutex<Resource>>
}

impl TestCommand {
    fn new(resource: Arc<Mutex<Resource>>) -> Self {
        Self{command: SimpleCommand::new(), resource}
    }
}

impl Drop for TestCommand {
    fn drop(&mut self) { self.resource.lock().unwrap().state = State::Released }
}

impl INotifier for TestCommand {}
impl ICommand for TestCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {}
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync> { self.command.notifier() }
}

struct TestController {
    controller: Arc<dyn IController>,
    resource: Arc<Mutex<Resource>>
}

impl TestController {
    pub fn new(key: &str, resource: Arc<Mutex<Resource>>) -> Self {
        Self { controller: Controller::get_instance(key, |k| Arc::new(Controller::new(k))), resource }
    }
}

impl Drop for TestController {
    fn drop(&mut self) {
        println!("Controller Dropped");
        self.resource.lock().unwrap().state = State::Released }
}

impl IController for TestController {
    fn execute_command(&self, notification: &Arc<dyn INotification>) { self.controller.execute_command(&notification); }
    fn register_command(&self, notification_name: &str, factory: Arc<dyn Fn() -> Box<dyn ICommand> + Send + Sync>) { self.controller.register_command(notification_name, factory) }
    fn has_command(&self, notification_name: &str) -> bool { self.controller.has_command(notification_name) }
    fn remove_command(&self, notification_name: &str) { self.controller.remove_command(notification_name); }
}

// ======================================================================

#[test]
fn test_command() {
    let resource = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    {
        let command = TestCommand::new(resource.clone());
        drop(command);
        assert_eq!(resource.lock().unwrap().state, State::Released);
    }
}

#[test]
fn test_controller() {
    let resource1 = Arc::new(Mutex::new(Resource{state: State::Allocated}));
    let resource2 = Arc::new(Mutex::new(Resource{state: State::Allocated}));

    {
        let controller = TestController::new("TestController", resource1.clone());

        controller.register_command("TestCommand", {
            let resource2 = Arc::clone(&resource2);
            Arc::new(move || Box::new(TestCommand::new(resource2.clone())))
        });

        let notification = Arc::new(Notification::new("TestCommand", None, None));
        controller.execute_command(&(notification as Arc<dyn INotification>));
        assert_eq!(resource2.lock().unwrap().state, State::Released);

        Controller::remove_controller("TestController");
        View::remove_view("TestController");
        // assert_eq!(resource1.lock().unwrap().state, State::Released);
    }
}