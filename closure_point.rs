use std::{cell::RefCell, rc::Rc};
use std::collections::HashMap;

//CREATING A HASHMAP WITH CLOSURES LOOKS PROBLEMATIC
fn closure_point() -> HashMap<&'static str, &'static dyn FnOnce()> {
    let x = Rc::new(RefCell::new(0));

    let mut methods = HashMap::new();

    let x_add = x.clone();
    let add = move || { *x_add.borrow_mut() += 1; };    
    methods.insert("add", &add);

    /*
    let x_get = x.clone();
    methods.insert("get", Box::new(move || -> i32 { 
        //println!("x strong_count: {}", Rc::strong_count(&x));
        *x_get.borrow() 
    }));
    */

    return methods;
}


fn main() {
    let obj = closure_point();

    println!("{}", obj.get("add")());
}

