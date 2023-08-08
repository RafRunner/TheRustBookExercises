use std::{fmt::Display, fmt, ptr};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
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

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.len += 1;

        let mut new_tail = Box::into_raw(Node::new(value));

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

        let mut new_head = Box::into_raw(Node::new(value));

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
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, formater: &mut fmt::Formatter) -> fmt::Result {
        formater.write_str("[")?;

        let mut first = true;
        let mut pointer = self.head;

        while !pointer.is_null() {
            if first {
                first = false;
            } else {
                formater.write_str(", ")?;
            }

            unsafe {
                formater.write_fmt(format_args!("{}", (*pointer).value))?;
                pointer = (*pointer).next;
            }
        }

        formater.write_str("]")
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
        let mut list = List::new();

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
}
