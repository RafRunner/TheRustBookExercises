use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

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

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;


impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub fn is_empty(&self) -> bool {
        self.len == 0
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

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("value", &self.value).finish()
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// Drop is needed to undo circular references in the Rc's
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

pub struct Iter<'a, T> {
    current_head: WeakLink<T>,
    last_yield: WeakLink<T>,
    current_tail: WeakLink<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(list: &'a List<T>) -> Self {
        Iter {
            current_head: list.head.as_ref().map(Rc::downgrade),
            last_yield: None,
            current_tail: list.tail.as_ref().map(Rc::downgrade),
            len: list.len,
            _boo: PhantomData,
        }
    }

    unsafe fn transform_lifetime<'y>(input: Ref<'_, T>) -> &'y T {
        &*(input.deref() as *const T)
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.current_head.take().map(|node| {
                self.len -= 1;
                self.current_head = node
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .map(Rc::downgrade);
                self.last_yield = Some(node);
                unsafe {
                    Iter::transform_lifetime(Ref::map(
                        self.last_yield
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .borrow(),
                        |n| &n.value,
                    ))
                }
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.current_tail.take().map(|node| {
                self.len -= 1;
                self.current_tail = node
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .prev
                    .as_ref()
                    .map(Rc::downgrade);
                self.last_yield = Some(node);
                unsafe {
                    Iter::transform_lifetime(Ref::map(
                        self.last_yield
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .borrow(),
                        |n| &n.value,
                    ))
                }
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

pub struct IterMut<'a, T> {
    current_head: WeakLink<T>,
    last_yield: WeakLink<T>,
    current_tail: WeakLink<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    fn new(list: &'a mut List<T>) -> Self {
        IterMut {
            current_head: list.head.as_ref().map(Rc::downgrade),
            last_yield: None,
            current_tail: list.tail.as_ref().map(Rc::downgrade),
            len: list.len,
            _boo: PhantomData,
        }
    }

    unsafe fn transform_lifetime_mut<'y>(mut input: RefMut<'_, T>) -> &'y mut T {
        &mut *(input.deref_mut() as *mut T)
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.current_head.take().map(|node| {
                self.len -= 1;
                self.current_head = node
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .next
                    .as_ref()
                    .map(Rc::downgrade);
                self.last_yield = Some(node);
                unsafe {
                    IterMut::transform_lifetime_mut(RefMut::map(
                        self.last_yield
                            .as_mut()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .borrow_mut(),
                        |n| &mut n.value,
                    ))
                }
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.current_tail.take().map(|node| {
                self.len -= 1;
                self.current_tail = node
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .prev
                    .as_ref()
                    .map(Rc::downgrade);
                self.last_yield = Some(node);
                unsafe {
                    IterMut::transform_lifetime_mut(RefMut::map(
                        self.last_yield
                            .as_mut()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .borrow_mut(),
                        |n| &mut n.value,
                    ))
                }
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_push_pop() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert_eq!(0, list.len());
        assert!(list.is_empty());

        list.push_front(23);
        assert_eq!(23, *list.peek_front().unwrap());
        assert_eq!(1, list.len());
        assert!(!list.is_empty());

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

        let mut in_rust = list.iter().skip(1).take(1);
        assert_eq!(&"in Rust", in_rust.next().unwrap());

        // We can still use the list and it's unchanged
        assert_eq!(4, list.len());
        assert_eq!(4, list.iter().len());
        assert_eq!(4, list.iter().count());

        let frase = list.into_iter().rev().collect::<Vec<&str>>().join(", ");

        assert_eq!(
            "hello, I'm a double linked list, in Rust, please send help",
            frase
        );
    }

    #[test]
    fn iter_and_iter_back() {
        let mut list = List::new();

        assert_eq!(0, list.len());
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        list.push_back(50);
        assert_eq!(5, list.len());

        let mut iter = list.iter();

        assert_eq!(5, iter.len());
        assert_eq!(Some(&10), iter.next());
        assert_eq!(Some(&20), iter.next());
        assert_eq!(Some(&50), iter.next_back());
        assert_eq!(Some(&40), iter.next_back());
        assert_eq!(Some(&30), iter.next());
        assert!(iter.next().is_none());
        assert!(iter.next_back().is_none());

        // List is unchanged
        assert_eq!(5, list.len());
        assert_eq!(Some(&10), list.peek_front().as_deref());
        assert_eq!(Some(&50), list.peek_back().as_deref());

        let mut list: List<i32> = List::new();
        assert_eq!(0, list.iter().count());

        list.push_front(20);
        assert_eq!(1, list.iter().count());

        list.push_front(30);
        assert_eq!(2, list.iter().count());

        list.pop_front();
        assert_eq!(1, list.iter().count());

        list.pop_front();
        assert_eq!(0, list.iter().count());
    }

    #[test]
    fn peek_mut() {
        let mut list = List::default();

        list.push_back(20);
        list.push_back(30);

        list.peek_front_mut().map(|mut val| *val = 100);

        assert_eq!(Some(&100), list.peek_front().as_deref());
        assert_eq!(Some(&30), list.peek_back().as_deref());

        // Can still pop
        assert_eq!(Some(100), list.pop_front());
        assert_eq!(Some(30), list.pop_front());
        assert_eq!(None, list.pop_front());

        // And push
        list.push_front(1000);
        list.push_front(2000);

        assert_eq!(Some(&1000), list.peek_back().as_deref());
        assert_eq!(Some(&2000), list.peek_front().as_deref());
    }

    #[test]
    fn iter_mut() {
        let mut list = List::default();

        for i in 0..5 {
            list.push_back(i);
        }

        for (index, val) in list.iter_mut().rev().enumerate() {
            *val = *val * 2 + index;
        }

        assert!(vec![4, 5, 6, 7, 8].iter().eq(list.iter()));
        assert_eq!(5, list.len());

        let mut list = List::new();

        list.push_front(String::from("Mario"));
        list.push_front(String::from("Luigi"));

        for name in list.iter_mut() {
            *name = format!("Hello, {}", name);
        }

        assert_eq!(
            Some(&String::from("Hello, Luigi")),
            list.peek_front().as_deref()
        );
        assert_eq!(
            Some(&String::from("Hello, Mario")),
            list.peek_back().as_deref()
        );
    }
}
