use std::any::Any;
use std::sync::{Arc, RwLock, Weak};
use puremvc::core::{Controller, Model, View};
use puremvc::interfaces::{ICommand, IController, IFacade, IMediator, IModel, INotification, INotifier, IObserver, IProxy, IView};
use puremvc::patterns::{Mediator, Notification, Proxy, SimpleCommand};

#[derive(Debug, PartialEq, Eq)]
enum State { Allocated, Released }
struct Resource { state: State }

struct TestMediator {
    mediator: Mediator,
    resource: Arc<RwLock<Resource>>
}
// ======================================================================
impl TestMediator {
    fn new(resource: Arc<RwLock<Resource>>) -> Self {
        Self { mediator: Mediator::new(None, None), resource }
    }
}

impl Drop for TestMediator {
    fn drop(&mut self) { self.resource.write().unwrap().state = State::Released }
}

impl INotifier for TestMediator {
    fn key(&self) -> &str {
        self.mediator.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.mediator.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.mediator.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.mediator.send_notification(name, body, type_);
    }
}
impl IMediator for TestMediator {
    fn name(&self) -> &str { self.mediator.name() }

    fn component(&self) -> Option<&Weak<dyn Any + Send + Sync>> {
        self.mediator.component()
    }

    fn set_component(&mut self, component: Option<Weak<dyn Any + Send + Sync>>) {
        self.mediator.set_component(component);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct TestView {
    _key: String,
    view: Option<Arc<dyn IView>>,
    resource: Arc<RwLock<Resource>>,
}

impl TestView {
    fn new(key: &str, resource: Arc<RwLock<Resource>>) -> Self {
        Self {
            _key: key.to_string(),
            view: None,
            resource
        }
    }
}

impl Drop for TestView {
    fn drop(&mut self) { self.resource.write().unwrap().state = State::Released; }
}

impl IView for TestView {
    fn initialize_view(&self) {

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

    fn register_mediator(&self, mediator: Arc<RwLock<dyn IMediator>>) {
        if let Some(view) = &self.view { view.register_mediator(mediator) }
    }

    fn retrieve_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.view.as_ref()?.retrieve_mediator(mediator_name)
    }

    fn has_mediator(&self, mediator_name: &str) -> bool {
        self.view.as_ref().map_or(false, |v| v.has_mediator(mediator_name))
    }

    fn remove_mediator(&self, mediator_name: &str) -> Option<Arc<RwLock<dyn IMediator>>> {
        self.view.as_ref()?.remove_mediator(mediator_name)
    }
}


#[test]
fn test_mediator() {
    let resource = Arc::new(RwLock::new(Resource{state: State::Allocated}));
    {
        let mediator = TestMediator::new(resource.clone());
        drop(mediator);
        assert_eq!(resource.write().unwrap().state, State::Released);
    }
}

#[test]
fn test_view() {
    let resource = Arc::new(RwLock::new(Resource{state: State::Allocated}));
    {
        let view = TestView::new("TestView", resource.clone());
        let mediator = TestMediator::new(resource.clone());
        view.register_mediator(Arc::new(RwLock::new(mediator)));
        View::remove_view("TestView");
        drop(view);
        assert_eq!(resource.write().unwrap().state, State::Released);
    }
}

// ======================================================================
struct TestProxy {
    proxy: Proxy,
    resource: Arc<RwLock<Resource>>
}

impl TestProxy {
    fn new(resource: Arc<RwLock<Resource>>) -> Self {
        Self{proxy: Proxy::new(None, None), resource}
    }
}

impl Drop for TestProxy {
    fn drop(&mut self) { self.resource.write().unwrap().state = State::Released }
}

impl INotifier for TestProxy {
    fn key(&self) -> &str {
        self.proxy.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.proxy.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.proxy.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.proxy.send_notification(name, body, type_);
    }
}

impl IProxy for TestProxy {
    fn name(&self) -> &str { self.proxy.name() }

    fn data(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.proxy.data()
    }

    fn set_data(&mut self, data: Option<Arc<dyn Any + Send + Sync>>) {
        self.proxy.set_data(data);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct TestModel {
    model: Arc<dyn IModel>,
    resource: Arc<RwLock<Resource>>
}

impl TestModel {
    fn new(key: &str, resource: Arc<RwLock<Resource>>) -> Self {
        Self { model: Model::get_instance(key, |k| Model::new(k)), resource }
    }
}

impl Drop for TestModel {
    fn drop(&mut self) { self.resource.write().unwrap().state = State::Released }
}

impl IModel for TestModel {
    fn initialize_model(&self) {}

    fn register_proxy(&self, proxy: Arc<RwLock<dyn IProxy>>) { self.model.register_proxy(proxy) }
    fn retrieve_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> { self.model.retrieve_proxy(proxy_name) }
    fn has_proxy(&self, proxy_name: &str) -> bool { self.model.has_proxy(proxy_name) }
    fn remove_proxy(&self, proxy_name: &str) -> Option<Arc<RwLock<dyn IProxy>>> { self.model.remove_proxy(proxy_name) }
}

#[test]
fn test_proxy() {
    let resource = Arc::new(RwLock::new(Resource{state: State::Allocated}));
    {
        let proxy = TestProxy::new(resource.clone());
        drop(proxy);
        assert_eq!(resource.write().unwrap().state, State::Released);
    }
}

#[test]
fn test_model() {
    let resource1 = Arc::new(RwLock::new(Resource{state: State::Allocated}));
    let resource2 = Arc::new(RwLock::new(Resource{state: State::Allocated}));
    {
        let model = TestModel::new("TestModel", resource1.clone());
        let proxy = TestProxy::new(resource2.clone());
        model.register_proxy(Arc::new(RwLock::new(proxy)));
        Model::remove_model("TestModel");
        drop(model);
        assert_eq!(resource1.read().unwrap().state, State::Released);
        assert_eq!(resource2.read().unwrap().state, State::Released);
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
    fn key(&self) -> &str {
        self.command.key()
    }

    fn facade(&self) -> Arc<dyn IFacade> {
        self.command.facade()
    }

    fn initialize_notifier(&mut self, key: &str) {
        self.command.initialize_notifier(key);
    }

    fn send_notification(&self, name: &str, body: Option<Arc<dyn Any + Send + Sync>>, type_: Option<&str>) {
        self.command.send_notification(name, body, type_);
    }
}
impl ICommand for TestCommand {
    fn execute(&mut self, _notification: &Arc<dyn INotification>) {}
}

struct TestController {
    controller: Arc<dyn IController>,
    resource: Arc<RwLock<Resource>>
}

impl TestController {
    pub fn new(key: &str, resource: Arc<RwLock<Resource>>) -> Self {
        Self { controller: Controller::get_instance(key, |k| Controller::new(k)), resource }
    }
}

impl Drop for TestController {
    fn drop(&mut self) {
        self.resource.write().unwrap().state = State::Released
    }
}

impl IController for TestController {
    fn initialize_controller(&self) {}

    fn register_command(&self, notification_name: &str, factory: fn() -> Box<dyn ICommand + Send + Sync>) { self.controller.register_command(&notification_name, factory) }
    fn execute_command(&self, notification: &Arc<dyn INotification>) { self.controller.execute_command(&notification); }
    fn has_command(&self, notification_name: &str) -> bool { self.controller.has_command(notification_name) }
    fn remove_command(&self, notification_name: &str) { self.controller.remove_command(notification_name); }
}

// ======================================================================

#[test]
fn test_command() {
    {
        let command = TestCommand::new();
        drop(command);
    }
}

#[test]
fn test_controller() {
    let resource1 = Arc::new(RwLock::new(Resource{state: State::Allocated}));
    let resource2 = Arc::new(RwLock::new(Resource{state: State::Allocated}));

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

        assert_eq!(resource1.read().unwrap().state, State::Released);
        assert_eq!(resource2.read().unwrap().state, State::Released);
    }
}
