use std::{
    cmp::Ordering,
    fmt,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ptr::{self, NonNull},
};

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

impl<T> Default for List<T> {
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
            current_head: NonNull::new(self.head),
            current_tail: NonNull::new(self.tail),
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            current_head: NonNull::new(self.head),
            current_tail: NonNull::new(self.tail),
            len: self.len,
            _boo: PhantomData,
        }
    }
}

// Drop here is also needed or we'll leak memory
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, formater: &mut fmt::Formatter<'_>) -> fmt::Result {
        formater.debug_list().entries(self.iter()).finish()
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

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = List::new();

        for item in iter {
            list.push_back(item);
        }

        list
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        let mut clone = List::new();

        for item in self.iter() {
            clone.push_front(item.clone());
        }

        clone
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }

    fn ne(&self, other: &Self) -> bool {
        self.len != other.len || self.iter().ne(other.iter())
    }
}

impl<T: Eq> Eq for List<T> {}

impl<T: PartialOrd> PartialOrd for List<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T: Ord> Ord for List<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: Hash> Hash for List<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for item in self.iter() {
            item.hash(state);
        }
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

pub struct Iter<'a, T> {
    current_head: Option<NonNull<Node<T>>>,
    current_tail: Option<NonNull<Node<T>>>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_head.take() {
            Some(head) if self.len > 0 => unsafe {
                let reference = head.as_ref();
                self.current_head = NonNull::new(reference.next);
                self.len -= 1;

                Some(&reference.value)
            },
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_tail.take() {
            Some(tail) if self.len > 0 => unsafe {
                let reference = tail.as_ref();
                self.current_tail = NonNull::new(reference.prev);
                self.len -= 1;

                Some(&reference.value)
            },
            _ => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

pub struct IterMut<'a, T> {
    current_head: Option<NonNull<Node<T>>>,
    current_tail: Option<NonNull<Node<T>>>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_head.take() {
            Some(mut head) if self.len > 0 => unsafe {
                let reference = head.as_mut();
                self.current_head = NonNull::new(reference.next);
                self.len -= 1;

                Some(&mut reference.value)
            },
            _ => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_tail.take() {
            Some(mut tail) if self.len > 0 => unsafe {
                let reference = tail.as_mut();
                self.current_tail = NonNull::new(reference.prev);
                self.len -= 1;

                Some(&mut reference.value)
            },
            _ => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

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

        iter.len();

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

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Normal order
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        // Testing DoubleEndedIterator
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);

        // Testing size_hint
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, 3);
        assert_eq!(upper, Some(3));
        assert_eq!(iter.len(), 3);

        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn from_iterator_and_iter_mut() {
        let mut count = 0;
        let counter = std::iter::from_fn(move || {
            count += 1;
            Some(count)
        })
        .take(5);

        let mut list = counter.collect::<List<_>>();

        assert_eq!(5, list.iter().len());
        assert!([1, 2, 3, 4, 5].iter().eq(list.iter()));

        for item in list.iter_mut() {
            *item += 10;
        }

        assert_eq!(5, list.iter().len());
        assert!([11, 12, 13, 14, 15].iter().eq(list.iter()));
    }
}
