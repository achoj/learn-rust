//in src/lib.rs

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

#[derive(Clone)]
struct List<T: Clone> {
    prev: Weak<RefCell<Self>>,
    value: RefCell<T>,
    next: Option<Rc<RefCell<Self>>>,
}

#[derive(Clone)]
pub struct DoubleLinkedList<T: Clone>(Rc<RefCell<List<T>>>);

impl<T: Clone> DoubleLinkedList<T> {
    pub fn new(value: T) -> Self {
        DoubleLinkedList(Rc::new(RefCell::new(List {
            prev: Weak::new(),
            value: RefCell::new(value),
            next: None,
        })))
    }
    pub fn set_next(&mut self, next: Self) {
        self.0.borrow_mut().next = Some(next.0.clone());
        next.0.borrow_mut().prev = Rc::downgrade(&self.0)
    }
    pub fn append(&mut self, value: T) {
        let next = Self::new(value);
        self.set_next(next);
    }
    pub fn prev(&self) -> Option<Self> {
        self.0.borrow().prev.upgrade().map(|rc_ref| Self(rc_ref))
    }
    pub fn next(&self) -> Option<Self> {
        self.0.borrow().next.clone().map(|rc_ref| Self(rc_ref))
    }
    pub fn head(&self) -> Self {
        if let Some(prev) = self.prev() {
            prev.head()
        } else {
            self.clone()
        }
    }
    pub fn tail(&self) -> Self {
        if let Some(next) = self.next() {
            next.tail()
        } else {
            self.clone()
        }
    }
    pub fn value(&self) -> RefCell<T> {
        self.0.borrow().value.clone()
    }
    pub fn set_value(&mut self, value: T) {
        self.0.borrow_mut().value = RefCell::new(value);
    }
}

impl<T: Clone + Display> Display for DoubleLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DoubleLinkedList[")?;
        let item = self.head();
        write!(f, "{}", item.value().borrow())?;
        let mut element = self.next();
        while let Some(item) = element {
            write!(f, ", ")?;
            write!(f, "{}", item.value().borrow())?;
            element = item.next()
        }
        write!(f, "]")
    }
}