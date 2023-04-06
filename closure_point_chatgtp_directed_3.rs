use std::{cell::RefCell, rc::Rc};

struct Point {
    x: Rc<RefCell<i32>>,
    y: Rc<RefCell<i32>>,
}

impl Point {
    fn new() -> Point {
        Point {
            x: Rc::new(RefCell::new(0)),
            y: Rc::new(RefCell::new(0)),
        }
    }

    fn move_fn(&mut self) {
        *self.x.borrow_mut() += 1;
        *self.y.borrow_mut() += 1;
    }

    fn get(&self) {
        println!("x: {}, y: {}", *self.x.borrow(), *self.y.borrow());
    }

    fn set(&mut self, new_x: i32, new_y: i32) {
        *self.x.borrow_mut() = new_x;
        *self.y.borrow_mut() = new_y;
    }
}

fn main() {
    let mut p = Point::new();

    p.get(); // prints "x: 0, y: 0"
    p.set(3, 5);
    p.move_fn();
    p.get(); // prints "x: 4, y: 6"
}

