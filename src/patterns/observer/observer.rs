use std::any::Any;
use std::rc::Rc;
use crate::INotification;
use crate::interfaces::IObserver;

pub struct Observer {
    notify: Option<Rc<dyn Fn(&mut dyn INotification)>>,
    context: Option<Rc<dyn Any>>,
}

impl Observer {
    pub fn new(notify: Option<Rc<dyn Fn(&mut dyn INotification)>>, context: Option<Rc<dyn Any>>) -> Self {
        Self {
            notify,
            context
        }
    }
}

impl IObserver for Observer {

    fn notify(&self) -> Option<Rc<dyn Fn(&mut dyn INotification)>> {
        self.notify.as_ref().map(Rc::clone)
    }

    fn set_notify(&mut self, notify: Option<Rc<dyn Fn(&mut dyn INotification)>>) {
        self.notify = notify;
    }

    fn context(&self) -> Option<Rc<dyn Any>> {
        self.context.as_ref().map(Rc::clone)
    }

    fn set_context(&mut self, context: Option<Rc<dyn Any>>) {
        self.context = context;
    }

    fn notify_observer(&self, notification: &mut dyn INotification) {
        if let Some(callback) = self.notify() {
            callback(notification);
        }
    }

    fn compare_notify_context(&self, object: &Rc<dyn Any>) -> bool {
        match &self.context {
            Some(ctx) => {
                // Get raw data pointers
                let ctx_ptr = (&**ctx) as *const dyn Any as *const ();
                let obj_ptr = (&**object) as *const dyn Any as *const ();
                ctx_ptr == obj_ptr
            }
            None => false,
        }
    }
}
