mod interfaces;
pub use interfaces::IMediator;
pub use interfaces::INotification;
pub use interfaces::IProxy;

mod patterns;
pub use patterns::Mediator;
pub use patterns::Notification;
pub use patterns::Proxy;
