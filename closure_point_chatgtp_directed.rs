//had to add this line by myself, but this works
use std::{cell::RefCell, rc::Rc};

struct Closures {
    mv: Box<dyn Fn()>,
    get: Box<dyn Fn() -> (i32, i32)>,
    set: Box<dyn Fn(i32, i32)>,
}

fn closure_point() -> Closures {
    let x = Rc::new(RefCell::new(0));
    let y = Rc::new(RefCell::new(0));
    let x_mv = x.clone();
    let y_mv = y.clone();
    let mv = move || {
        *x_mv.borrow_mut() += 1;
        *y_mv.borrow_mut() += 1;
    };
    let x_get = x.clone();
    let y_get = y.clone();
    let get = move || (*x_get.borrow(), *y_get.borrow());
    let x_set = x.clone();
    let y_set = y.clone();
    let set = move |val1, val2| {
        *x_set.borrow_mut() = val1;
        *y_set.borrow_mut() = val2;
    };
    Closures {
        mv: Box::new(mv),
        get: Box::new(get),
        set: Box::new(set),
    }
}

fn main() {
    let closures = closure_point();
    (closures.mv)();
    (closures.mv)();
    println!("{:?}", (closures.get)());
    (closures.set)(5, 10);
    println!("{:?}", (closures.get)());
}

