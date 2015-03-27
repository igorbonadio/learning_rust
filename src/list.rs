use std::rc::Rc;

pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Empty
}

impl<T> List<T> {
    pub fn empty() -> Rc<List<T>> {
        Rc::new(List::Empty)
    }

    pub fn len(&self) -> usize {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Empty => 0
        }
    }
}

pub fn prepend<T>(list: Rc<List<T>>, value: T) -> Rc<List<T>> {
    Rc::new(List::Cons(value, list))
}

#[test]
fn test_len() {
    let x = prepend(prepend(prepend(List::empty(), 1), 2), 3);
    let yx = prepend(x.clone(), 4);
    let zx = prepend(x.clone(), 5);

    assert!(x.len() == 3);
    assert!(yx.len() == 4);
    assert!(zx.len() == 4);
}
