use std::{fmt, fmt::Display, marker::PhantomData, ptr};

pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(Node {
            value,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        })
    }
}

impl <T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_back(&mut self, value: T) {
        self.len += 1;

        let new_tail = Box::into_raw(Node::new(value));

        unsafe {
            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
            }

            (*new_tail).prev = self.tail;
            self.tail = new_tail;
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.len += 1;

        let new_head = Box::into_raw(Node::new(value));

        unsafe {
            if self.head.is_null() {
                self.tail = new_head;
            } else {
                (*self.head).prev = new_head;
            }

            (*new_head).next = self.head;
            self.head = new_head;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                } else {
                    (*self.head).prev = ptr::null_mut();
                }

                self.len -= 1;
                Some(head.value)
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            None
        } else {
            unsafe {
                let tail = Box::from_raw(self.tail);
                self.tail = tail.prev;

                if self.tail.is_null() {
                    self.head = ptr::null_mut();
                } else {
                    (*self.tail).next = ptr::null_mut();
                }

                self.len -= 1;
                Some(tail.value)
            }
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&(*self.head).value) }
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        if self.tail.is_null() {
            None
        } else {
            unsafe { Some(&(*self.tail).value) }
        }
    }

    pub fn peek_mut_front(&mut self) -> Option<&mut T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&mut (*self.head).value) }
        }
    }

    pub fn peek_mut_back(&mut self) -> Option<&mut T> {
        if self.tail.is_null() {
            None
        } else {
            unsafe { Some(&mut (*self.tail).value) }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current_head: self.head,
            current_tail: self.tail,
            _boo: PhantomData,
        }
    }
}

// Drop here is also needed or we'll leak memory
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {
        }
    }
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        formater.write_str("[")?;

        let mut iter = self.iter();

        if let Some(first) = iter.next() {
            formater.write_fmt(format_args!("{}", *first))?;
        }

        for value in iter {
            formater.write_fmt(format_args!(", {}", *value))?;
        }

        formater.write_str("]")
    }
}

pub struct Iter<'a, T> {
    current_head: *const Node<T>,
    current_tail: *const Node<T>,
    _boo: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    fn have_pointers_met(&mut self) -> bool {
        // front and back have met in the middle
        if self.current_head == self.current_tail {
            self.current_head = ptr::null();
            self.current_tail = ptr::null();

            true
        } else {
            false
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_head.is_null() {
            None
        } else {
            unsafe {
                let reference = &(*self.current_head).value;
                if !self.have_pointers_met() {
                    self.current_head = (*self.current_head).next;
                }
                Some(reference)
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_tail.is_null() {
            None
        } else {
            unsafe {
                let reference = &(*self.current_tail).value;
                if !self.have_pointers_met() {
                    self.current_tail = (*self.current_tail).prev;
                }
                Some(reference)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None);

        // Check the exhaustion case fixed the pointer right
        list.push_back(6);
        list.push_back(7);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn back_and_front() {
        let mut list = List::default();

        assert_eq!("[]", list.to_string());

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!("[10, 20, 30]", list.to_string());

        assert_eq!(Some(30), list.pop_back());
        assert_eq!(Some(10), list.pop_front());

        assert_eq!("[20]", list.to_string());

        list.push_front(40);
        list.push_back(50);

        assert_eq!("[40, 20, 50]", list.to_string());

        assert_eq!(Some(40), list.pop_front());
        assert_eq!(Some(20), list.pop_front());
        assert_eq!(Some(50), list.pop_front());
        assert_eq!(None, list.pop_front());

        assert_eq!("[]", list.to_string());

        // Just checking nothing is corrupted
        assert_eq!(None, list.pop_front());
        assert_eq!(None, list.pop_back());

        list.push_front(100);
        list.push_front(200);

        assert_eq!("[200, 100]", list.to_string());

        assert_eq!(Some(100), list.pop_back());
        assert_eq!(Some(200), list.pop_back());
        assert_eq!(None, list.pop_back());
    }

    #[test]
    fn peek_and_peek_mut() {
        let mut list = List::new();

        assert_eq!(None, list.peek_front());
        assert_eq!(None, list.peek_back());

        list.push_front("hello");
        list.push_back("world");

        assert_eq!(Some(&"hello"), list.peek_front());
        assert_eq!(Some(&"world"), list.peek_back());

        list.pop_back();

        assert_eq!(Some(&"hello"), list.peek_front());
        assert_eq!(Some(&"hello"), list.peek_back());

        list.peek_mut_back().map(|val| *val = "rust");

        assert_eq!(Some(&"rust"), list.peek_front());
        assert_eq!(Some(&"rust"), list.peek_back());

        list.push_front("hello");
        list.peek_mut_front().map(|val| *val = "HELLO");

        assert_eq!(Some(&"HELLO"), list.peek_front());
        assert_eq!(Some(&"rust"), list.peek_back());

        list.pop_back();
        list.pop_front();

        assert_eq!(None, list.peek_front());
        assert_eq!(None, list.peek_back());
    }

    #[test]
    fn iter() {
        let mut list = List::new();

        assert_eq!(0, list.len());
        assert!(list.is_empty());
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        list.push_back(50);
        assert_eq!(5, list.len());

        let mut iter = list.iter();

        assert_eq!(Some(&10), iter.next());
        assert_eq!(Some(&20), iter.next());
        assert_eq!(Some(&50), iter.next_back());
        assert_eq!(Some(&40), iter.next_back());
        assert_eq!(Some(&30), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());

        // List is unchanged
        assert_eq!(5, list.len());
        assert_eq!(Some(&10), list.peek_front());
        assert_eq!(Some(&50), list.peek_back());
        assert!(!list.is_empty());
    }
}
