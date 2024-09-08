use std::rc::Rc;

struct TheBox<T>(T);

impl<T> TheBox<T> {
    const fn new(val: T) -> Self {
        Self(val)
    }
}

impl<T> std::ops::Deref for TheBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use self::List::{Cons, Nil};

struct Example;

impl Drop for Example {
    fn drop(&mut self) {
        println!("drop");
    }
}

pub fn use_pointers() {
    let x = Rc::new(Example);
    let y = Rc::clone(&x);
    println!("A");
    drop(x);
    println!("B");
    drop(y);
    println!("C");

    let _x: [Box<(usize, usize)>; 4] = Default::default();
    let arr: [usize; 10] = std::array::from_fn(|i| i * 2);
    println!("{arr:?}"); // Output: [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]
    let list =
        Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let _link = Cons(5, Rc::clone(&list));
    let _link2 = Cons(6, Rc::clone(&list));

    let x = 5;
    let y = TheBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
