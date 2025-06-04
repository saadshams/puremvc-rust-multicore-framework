use puremvc::{Model};

#[test]
fn test_model() {
    // Foo::print_registry();
    // Registry {
    // }

    let _instance = Model::get_instance("myfoo".to_string(), |k| Box::new(Model::new(k)));
    //println!("{:?}", instance);
    // MyFoo(Foo { key: "myfoo" })

    //Foo::print_registry();
    // Registry {
    //     myfoo : MyFoo(Foo { key: "myfoo" })
    // }

    // second call
    let _instance = Model::get_instance("myfoo".to_string(), |k| Box::new(Model::new(k)));
    // println!("{:?}", instance);
    // MyFoo(Foo { key: "myfoo" })

    let _instance = Model::get_instance("myfoo2".to_string(), |k| Box::new(Model::new(k)));
    println!("Done")
    // println!("{:?}", instance);
    // MyFoo(Foo { key: "myfoo2" })

    // Model::print_registry();
    // Registry {
    //     myfoo2 : MyFoo(Foo { key: "myfoo2" })
    //     myfoo : MyFoo(Foo { key: "myfoo" })
    // }
}