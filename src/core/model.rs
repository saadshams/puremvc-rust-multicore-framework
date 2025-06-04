use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use crate::{IModel};

static INSTANCE_MAP: LazyLock<Mutex<HashMap<String, Arc<dyn IModel>>>> = LazyLock::new(|| Default::default());

pub struct Model {
    pub key: String,
}

impl Model {
    pub fn new(key: String) -> Self {
        Self { key }
    }
    
    pub fn get_instance(key: String, factory: impl Fn(String) -> Box<dyn IModel>) -> Arc<dyn IModel> {
        INSTANCE_MAP.lock().unwrap()
            .entry(key.clone())
            .or_insert_with(|| Arc::from(factory(key)))
            .clone()
    }

    // pub fn print_registry() {
    //     let registry = INSTANCE_MAP.lock().unwrap();
    //     println!("Registry {{");
    //     for (k, v) in registry.iter() {
    //         println!("    {k} : {v:?}")
    //     }
    //     println!("}}");
    // }
}

impl IModel for Model {
    
}

// #[derive(Debug)]
// pub struct Foo {
//     pub key: String,
// }

// #[derive(Debug)]
// pub struct MyFoo(Foo);

// impl MyFoo {
//     pub fn new(key: String) -> Self {
//         Self(Foo { key })
//     }
// }





