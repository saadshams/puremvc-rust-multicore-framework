use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::{IController, IMediator, INotification};
use crate::interfaces::IObserver;

pub struct Observer {
    notify: Option<Arc<dyn Fn(&Arc<Mutex<dyn INotification>>) + Send + Sync>>,
    context: Option<Arc<dyn Any + Send + Sync>>,
}

impl Observer {
    pub fn new(notify: Option<Arc<dyn Fn(&Arc<Mutex<dyn INotification>>) + Send + Sync>>, context: Option<Arc<dyn Any + Send + Sync>>) -> Self {
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
    fn notify(&self) -> Option<Arc<dyn Fn(&Arc<Mutex<dyn INotification>>) + Send + Sync>> {
        self.notify.clone()
    }

    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&Arc<Mutex<dyn INotification>>) + Send + Sync>>) {
        self.notify = notify;
    }

    fn context(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        self.context.clone()
    }

    fn set_context(&mut self, context: Option<Arc<dyn Any + Send + Sync>>) {
        self.context = context;
    }

    fn notify_observer(&self, notification: &Arc<Mutex<dyn INotification>>) {
        if let Some(notify) = self.notify() {
            notify(notification);
        }
    }

    fn compare_notify_context(&self, object: &Arc<dyn Any + Send + Sync>) -> bool {
        match self.context() {
            Some(context) => {
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

                false // Unsupported type
            }
            None => false,
        }
    }
}
