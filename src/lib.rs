pub mod interfaces {
    mod icontroller;
    pub use icontroller::IController;
    mod imodel;
    pub use imodel::IModel;
    mod iview;
    pub use iview::IView;

    mod icommand;
    pub use icommand::ICommand;
    mod ifacade;
    pub use ifacade::IFacade;
    mod imediator;
    pub use imediator::IMediator;
    mod inotification;
    pub use inotification::INotification;
    mod inotifier;
    pub use inotifier::INotifier;
    mod iobserver;
    pub use iobserver::IObserver;
    mod iproxy;
    pub use iproxy::IProxy;
}

pub mod core {
    mod controller;
    pub use controller::Controller;

    pub mod model;
    pub use model::Model;

    pub mod view;
    pub use view::View;
}

pub mod patterns {
    mod command;
    pub use command::SimpleCommand;
    pub use command::MacroCommand;

    pub mod facade;
    pub use facade::Facade;

    pub mod mediator;
    pub use mediator::Mediator;

    pub mod observer;
    pub use observer::Notification;
    pub use observer::Notifier;
    pub use observer::Observer;

    pub mod proxy;
    pub use proxy::Proxy;
}
