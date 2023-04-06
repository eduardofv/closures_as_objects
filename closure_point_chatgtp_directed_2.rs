use std::{cell::RefCell, rc::Rc};
use std::collections::HashMap;

fn closure_point() -> HashMap<&'static str, Box<dyn FnMut()>> {
    let x = Rc::new(RefCell::new(0));
    let y = Rc::new(RefCell::new(0));

    let mut methods: HashMap<&str, Box<dyn FnMut()>> = HashMap::new();

    let x_add = x.clone();
    let y_add = y.clone();
    let mut move_fn = move || {
        *x_add.borrow_mut() += 1;
        *y_add.borrow_mut() += 1;
    };

    let x_get = x.clone();
    let y_get = y.clone();
    let get_fn = move || {
        println!("x: {}, y: {}", *x_get.borrow(), *y_get.borrow());
    };

    let x_set = x.clone();
    let y_set = y.clone();
    let set_fn = move |new_x: i32, new_y: i32| {
        *x_set.borrow_mut() = new_x;
        *y_set.borrow_mut() = new_y;
    };

    methods.insert("move", Box::new(move_fn));
    methods.insert("get", Box::new(get_fn));
    methods.insert("set", Box::new(set_fn));

    methods
}

fn main() {
    let mut object = closure_point();

    object.get()(); // prints "x: 0, y: 0"
    object.set(3, 5)();
    object.move_fn()();
    object.get()(); // prints "x: 4, y: 6"
}

