use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

pub struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    pub value: T,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            next: None,
            prev: None,
            value,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|head| Ref::map(head.borrow(), |head| &head.value))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|head| RefMut::map(head.borrow_mut(), |head| &mut head.value))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |tail| &tail.value))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |tail| &mut tail.value))
    }

    pub fn push_front(&mut self, value: T) {
        let new_head = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
            }
            None => {
                self.tail = Some(new_head.clone());
            }
        };

        self.len += 1;
        self.head = Some(new_head);
    }

    pub fn push_back(&mut self, value: T) {
        let new_tail = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
            }
        };

        self.tail = Some(new_tail);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current_head: self.head.clone(),
            current_tail: self.tail.clone(),
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

// Really bad ideia, Don't really know how to improve while not using a vec and it will still be bad
pub struct Iter<T> {
    current_head: Link<T>,
    current_tail: Link<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_head.take().map(|node| {
            self.current_head = node.borrow().next.clone();
            node.clone()
        })
    }
}

impl<T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.current_tail.take().map(|node| {
            self.current_tail = node.borrow().prev.clone();
            node.clone()
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_push_pop() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert_eq!(0, list.len());

        list.push_front(23);
        assert_eq!(23, *list.peek_front().unwrap());
        assert_eq!(1, list.len());

        list.push_front(17);
        assert_eq!(17, *list.peek_front().unwrap());
        assert_eq!(2, list.len());

        assert_eq!(17, list.pop_front().unwrap());
        assert_eq!(23, *list.peek_front().unwrap());
        assert_eq!(1, list.len());

        assert_eq!(23, list.pop_back().unwrap());
        assert_eq!(None, list.pop_front());
        assert_eq!(None, list.pop_front());

        list.push_front(10);
        list.push_front(20);
        list.push_front(30);
        list.push_back(50);

        assert_eq!(50, list.pop_back().unwrap());
        assert_eq!(30, list.pop_front().unwrap());
        assert_eq!(10, list.pop_back().unwrap());
        assert_eq!(20, list.pop_front().unwrap());
        assert_eq!(None, list.pop_front());
    }

    #[test]
    fn iters() {
        let mut list = List::new();

        list.push_front("hello");
        list.push_front("I'm a double linked list");
        list.push_front("in Rust");
        list.push_front("please send help");

        // The iter needs to be created in another scope to be dropped so the list can be popped
        // This is why implementing a reference Iterator is a bad ideia with interior mutability
        {
            let in_rust: Vec<_> = list.iter().skip(1).take(1).collect();
            assert_eq!("in Rust", in_rust[0].borrow().value);
        }

        // We can still use the list and it's unchanged
        assert_eq!(4, list.len());
        // We can create an iter in the same scope as long as we don't borrow anything
        assert_eq!(4, list.iter().count());

        let frase = list.into_iter().rev().collect::<Vec<&str>>().join(", ");

        assert_eq!(
            "hello, I'm a double linked list, in Rust, please send help",
            frase
        );
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();

        list.push_back(20);
        list.push_back(30);

        list.peek_front_mut().map(|mut val| *val = 100);

        assert_eq!(100, *list.peek_front().unwrap());
        assert_eq!(30, *list.peek_back().unwrap());
    }
}
