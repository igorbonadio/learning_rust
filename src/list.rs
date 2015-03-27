pub enum List<T> {
    Cons(T, Box<List<T>>),
    Empty
}

impl<T> List<T> {
    pub fn empty() -> List<T> {
        List::Empty
    }

    pub fn prepend(self, value: T) -> List<T> {
        List::Cons(value, Box::new(self))
    }

    pub fn len(&self) -> usize {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Empty => 0
        }
    }
}

#[test]
fn test_len() {
    let x = List::empty().prepend(1).prepend(2).prepend(3);
    assert!(x.len() == 3);

    let y = x.prepend(4);
    assert!(y.len() == 4);
}
