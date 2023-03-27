use std::{cell::RefCell, rc::Rc};

fn closure_point() -> (Box<dyn FnMut()>, Box<dyn FnMut() -> i32>) {
    let x = Rc::new(RefCell::new(1));

    let x_add = x.clone();
    let add = Box::new(move || { *x_add.borrow_mut() += 1; });    

    let x_get = x.clone();
    let get = Box::new(move || -> i32 { 
        //println!("x strong_count: {}", Rc::strong_count(&x));
        *x_get.borrow() 
    });

    return (add, get);
}


fn main() {
    let (mut obj_add, mut obj_get) = closure_point();

    println!("{}", obj_get());
    obj_add();
    obj_add();
    println!("{}", obj_get());
}


/* simpler example
fn clos() -> Box<dyn FnMut() -> i32> {
    let mut x:i32 = 1;


    let get = Box::new(move || -> i32{ x+=1; x });

    return get;
}


fn main() {
    let mut obj_get = clos();

    println!("{}", obj_get());
    println!("{}", obj_get());
}
*/
