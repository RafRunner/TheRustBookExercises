use std::fmt::Display;

#[derive(Debug)]
pub struct OkStack<T> {
    head: Link<T>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> OkStack<T> {
    pub fn new() -> Self {
        OkStack { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_head = Node {
            value,
            next: old_head,
        };

        self.len += 1;
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.len -= 1;
            self.head = head.next;

            head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }
}

impl<T> Drop for OkStack<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();

        while let Some(mut boxed_node) = curr {
            curr = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(OkStack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<T> Display for OkStack<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn push_and_pop() {
        let mut stack = OkStack::new();

        assert_eq!(0, stack.len());

        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(3, stack.len());

        assert_eq!(Some(30), stack.pop());
        assert_eq!(Some(20), stack.pop());

        stack.push(100);

        assert_eq!(Some(100), stack.pop());
        assert_eq!(Some(10), stack.pop());

        assert_eq!(0, stack.len());

        assert_eq!(None, stack.pop());
        assert_eq!(None, stack.pop());

        assert_eq!(0, stack.len());
    }

    #[test]
    fn test_peek() {
        let mut stack = OkStack::new();

        assert_eq!(None, stack.peek());

        stack.push(10);
        assert_eq!(10, *stack.peek().unwrap());

        stack.push(20);
        assert_eq!(20, *stack.peek().unwrap());

        stack.pop();
        assert_eq!(10, *stack.peek().unwrap());

        stack.pop();
        assert_eq!(None, stack.peek());
    }

    #[test]
    fn test_iter() {
        let mut stack = OkStack::new();

        stack.push("hello");
        stack.push("world");
        stack.push("from");
        stack.push("rust");

        let mut as_vec = stack.into_iter().collect::<Vec<_>>();
        as_vec.reverse();

        assert_eq!("hello world from rust", as_vec.join(" "));

        let mut stack = OkStack::new();

        stack.push("again");
        stack.push("hello");

        let as_vec = stack.iter().collect::<Vec<_>>();

        assert_eq!("hello", *as_vec[0]);
        assert_eq!("again", *as_vec[1]);

        assert_eq!(2, stack.len());
        assert_eq!("hello", stack.pop().as_deref().unwrap());
        assert_eq!("again", stack.pop().as_deref().unwrap());
    }

    #[test]
    fn test_iter_mut() {
        let mut stack = OkStack::new();

        stack.push(String::from("Hello everyone!"));
        stack.push(String::from("How are you,"));
        stack.push(String::from("fine, thank you"));

        stack
            .iter_mut()
            .skip(2)
            .map(|val| val.push_str(" Oh my gah!"))
            .count();

        assert_eq!("fine, thank you", stack.pop().as_deref().unwrap());
        assert_eq!("How are you,", stack.pop().as_deref().unwrap());
        assert_eq!(
            "Hello everyone! Oh my gah!",
            stack.pop().as_deref().unwrap()
        );
    }

    #[test]
    fn display_test() {
        let mut stack = OkStack::new();

        stack.push(0.10);
        stack.push(1.43);
        stack.push(3.14);

        assert_eq!("[3.14, 1.43, 0.1]", stack.to_string());

        stack.push(83.0);

        assert_eq!("[83, 3.14, 1.43, 0.1]", stack.to_string());

        stack.pop();
        stack.pop();

        assert_eq!("[1.43, 0.1]", stack.to_string());
    }

    #[test]
    fn test_peek_mut() {
        let mut stack = OkStack::new();
        assert_eq!(None, stack.peek_mut());

        stack.push(100);
        assert_eq!(Some(&mut 100), stack.peek_mut());

        *stack.peek_mut().unwrap() = 99;

        assert_eq!(99, *stack.peek().unwrap());
    }
}
