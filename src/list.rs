use std::rc::Rc;

pub struct List<T> {
    list: Rc<ListImpl<T>>
}

impl<T> List<T> {
    pub fn empty() -> List<T> {
        List {
            list: ListImpl::empty()
        }
    }

    pub fn prepend(&self, value: T) -> List<T> {
        List {
            list: prepend((*self).list.clone(), value)
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn head(&self) -> Option<&T> {
        self.list.head()
    }

    pub fn tail(&self) -> List<T> {
        List {
            list: self.list.tail().clone()
        }
    }
}

enum ListImpl<T> {
    Cons(T, Rc<ListImpl<T>>),
    Empty
}

impl<T> ListImpl<T> {
    fn empty() -> Rc<ListImpl<T>> {
        Rc::new(ListImpl::Empty)
    }

    fn len(&self) -> usize {
        match *self {
            ListImpl::Cons(_, ref tail) => 1 + tail.len(),
            ListImpl::Empty => 0
        }
    }

    fn head(&self) -> Option<&T> {
        match *self {
            ListImpl::Cons(ref value, _) => Some(value),
            ListImpl::Empty => None
        }
    }

    fn tail(&self) -> Rc<ListImpl<T>> {
        match *self {
            ListImpl::Cons(_, ref tail) => tail.clone(),
            ListImpl::Empty => ListImpl::empty()
        }
    }
}

fn prepend<T>(list: Rc<ListImpl<T>>, value: T) -> Rc<ListImpl<T>> {
    Rc::new(ListImpl::Cons(value, list))
}

#[test]
fn test_len() {
    let x = List::empty().prepend(1).prepend(2).prepend(3);
    let yx = x.prepend(4);
    let zx = x.prepend(5);

    assert!(*x.head().unwrap() == 3);
    assert!(*yx.head().unwrap() == 4);
    assert!(*zx.head().unwrap() == 5);

    assert!(*x.tail().head().unwrap() == 2);
    assert!(*yx.tail().head().unwrap() == 3);
    assert!(*zx.tail().head().unwrap() == 3);

    assert!(x.len() == 3);
    assert!(yx.len() == 4);
    assert!(zx.len() == 4);
}
