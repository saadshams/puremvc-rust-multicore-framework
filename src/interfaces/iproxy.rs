use std::any::Any;
use crate::INotifier;

pub trait IProxy: INotifier {
    fn name(&self) -> &str;

    fn data(&self) -> Option<&(dyn Any + Send + Sync)> {
        None
    }

    fn data_mut(&mut self) -> Option<&mut (dyn Any + Send + Sync)> {
        None
    }

    fn set_data(&mut self, _data: Option<Box<dyn Any + Send + Sync>>) {

    }
    
    fn notifier(&mut self) -> &mut Box<dyn INotifier + Send + Sync>;

    fn on_register(&mut self) {

    }

    fn on_remove(&mut self) {

    }
}
