use std::any::Any;
use std::sync::Arc;
use crate::INotification;
use crate::interfaces::IObserver;

pub struct Observer {
    notify: Option<Arc<dyn Fn(&mut dyn INotification) + Send + Sync>>,
    context: Option<Arc<Box<dyn Any + Send + Sync>>>,
}

impl Observer {
    pub fn new(notify: Option<Arc<dyn Fn(&mut dyn INotification) + Send + Sync>>, context: Option<Arc<Box<dyn Any + Send + Sync>>>) -> Self {
        Self {
            notify,
            context,
        }
    }
}

impl IObserver for Observer {

    fn notify(&self) -> Option<Arc<dyn Fn(&mut dyn INotification) + Send + Sync>> {
        self.notify.clone()
    }

    fn set_notify(&mut self, notify: Option<Arc<dyn Fn(&mut dyn INotification) + Send + Sync>>) {
        self.notify = notify;
    }

    fn context(&self) -> Option<Arc<Box<dyn Any + Send + Sync>>> {
        self.context.clone()
    }

    fn set_context(&mut self, context: Option<Arc<Box<dyn Any + Send + Sync>>>) {
        self.context = context;
    }

    fn notify_observer(&self, notification: &mut dyn INotification) {
        if let Some(callback) = self.notify() {
            callback(notification);
        }
    }

    fn compare_notify_context(&self, other: &Arc<Box<dyn Any + Send + Sync>>) -> bool {
        match &self.context {
            Some(ctx) => Arc::as_ptr(ctx) as *const () == Arc::as_ptr(other) as *const (),
            None => false,
        }
    }
}
