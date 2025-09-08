use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::{Controller, IController, IMediator, INotification};
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
                if let (Ok(a_arc), Ok(b_arc)) = (
                    context.clone().downcast::<Arc<dyn IController>>(),
                    object.clone().downcast::<Arc<dyn IController>>(),
                ) {
                    // Downcast trait object to concrete Controller
                    let a_concrete = a_arc.as_ref().as_any().downcast_ref::<Controller>();
                    let b_concrete = b_arc.as_ref().as_any().downcast_ref::<Controller>();
                    if let (Some(a_ctrl), Some(b_ctrl)) = (a_concrete, b_concrete) {
                        // Compare actual memory addresses of the underlying Controllers
                        return (a_ctrl as *const Controller) == (b_ctrl as *const Controller);
                    }
                }

                // Downcast trait object to concrete Mediator
                if let (Ok(a), Ok(b)) = (
                    context.clone().downcast::<Arc<Mutex<dyn IMediator>>>(),
                    object.clone().downcast::<Arc<Mutex<dyn IMediator>>>(),
                ) {
                    return Arc::ptr_eq(&a, &b);
                }

                panic!("Unsupported type in compare_notify_context");
            }
            None => false,
        }
    }
}
