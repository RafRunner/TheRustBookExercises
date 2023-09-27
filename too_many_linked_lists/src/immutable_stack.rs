use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub struct ImmutableStack<T> {
    head: Link<T>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

impl <T> Default for ImmutableStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ImmutableStack<T> {
    pub fn new() -> Self {
        ImmutableStack { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn prepend(&self, value: T) -> ImmutableStack<T> {
        let new_node = Node {
            value,
            next: self.head.clone(),
        };

        ImmutableStack {
            head: Some(Rc::new(new_node)),
            len: self.len + 1,
        }
    }

    pub fn tail(&self) -> ImmutableStack<T> {
        let tail = self.head.as_ref().and_then(|node| node.next.clone());
        ImmutableStack {
            head: tail,
            len: self.len.saturating_sub(1),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }
}

impl<T> Drop for ImmutableStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> Display for ImmutableStack<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let mut first = true;

        for val in self.iter() {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            f.write_fmt(format_args!("{}", *val))?;
        }

        f.write_str("]")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        let list = ImmutableStack::new();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn is_immutable() {
        let list1 = ImmutableStack::default();

        let list2 = list1.prepend("hello").prepend("I am").prepend("immutable");

        assert_eq!(None, list1.head());
        assert_eq!(Some(&"immutable"), list2.head());
        assert_eq!(0, list1.len());
        assert_eq!(3, list2.len());

        let list3 = list2.tail();

        assert_eq!(3, list2.len());
        assert_eq!(2, list3.len());
    }

    #[test]
    fn iter() {
        let list = ImmutableStack::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        assert!(vec![3, 2, 1].iter().eq(list.iter()));
    }

    #[test]
    fn display() {
        let list = ImmutableStack::new();
        assert_eq!("[]", list.to_string());

        let list = list.prepend(1).prepend(2).prepend(3);

        assert_eq!("[3, 2, 1]", list.to_string());
    }
}