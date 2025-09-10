use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::{IController, IMediator, INotification};
use crate::interfaces::IObserver;

pub struct Observer {
    notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>,
    context: Option<Arc<dyn Any + Send + Sync>>, // todo no Any
}

impl Observer {
    pub fn new(notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>, context: Option<Arc<dyn Any + Send + Sync>>) -> Self {
        Self {
            notify,
            context,
        }
    }
}

impl dyn IObserver {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

impl IObserver for Observer {
    fn notify(&self) -> Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>> {
        self.notify.clone()
    }

    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>) {
        self.notify = notify;
    }

    fn context(&self) -> Option<&Arc<dyn Any + Send + Sync>> {
        self.context.as_ref()
    }

    fn set_context(&mut self, context: Option<Arc<dyn Any + Send + Sync>>) {
        self.context = context;
    }

    fn notify_observer(&self, notification: &Arc<dyn INotification>) {
        if let Some(notify) = &self.notify() {
            notify(notification);
        }
    }

    fn compare_notify_context(&self, object: &Arc<dyn Any + Send + Sync>) -> bool {
        if let Some(context) = self.context() {
            if let (Some(a), Some(b)) = (
                context.downcast_ref::<Arc<dyn IController>>(),
                object.downcast_ref::<Arc<dyn IController>>(),
            ) {
                return Arc::ptr_eq(a, b);
            }

            if let (Some(a), Some(b)) = (
                context.downcast_ref::<Arc<Mutex<dyn IMediator>>>(),
                object.downcast_ref::<Arc<Mutex<dyn IMediator>>>(),
            ) {
                return Arc::ptr_eq(a, b);
            }
        }
        false
    }
}

/*
use std::sync::Arc;

// Notification trait
pub trait INotification: Send + Sync {}

// Example concrete notification
pub struct Notification {
    pub name: String,
}
impl INotification for Notification {}

// Observer struct with optional fields
pub struct Observer {
    callback: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>,
    context: Option<Arc<dyn Send + Sync>>,
}

impl Observer {
    // Constructor
    pub fn new(
        callback: Option<Arc<dyn Fn(&Arc<dyn INotification>) + Send + Sync>>,
        context: Option<Arc<dyn Send + Sync>>,
    ) -> Self {
        Self { callback, context }
    }

    // Notify the observer if a callback exists
    pub fn notify(&self, notification: &Arc<dyn INotification>) {
        if let Some(cb) = &self.callback {
            cb(notification);
        }
    }

    // Optional getter for the context
    pub fn context(&self) -> Option<&Arc<dyn Send + Sync>> {
        self.context.as_ref()
    }
}

// -----------------
// Example usage

fn main() {
    let context = Arc::new(String::from("my context"));

    let observer = Observer::new(
        Some(Arc::new(move |notification: &Arc<dyn INotification>| {
            println!("Observer received notification");
        })),
        Some(Arc::clone(&context)),
    );

    let notification = Arc::new(Notification { name: "Test".into() });
    observer.notify(&notification);

    if let Some(ctx) = observer.context() {
        println!("Observer context: {}", ctx.downcast_ref::<String>().unwrap());
    }
}

 */