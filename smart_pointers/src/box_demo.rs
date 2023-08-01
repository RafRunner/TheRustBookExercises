use std::fmt::Display;
use std::mem;

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub struct ListIterator<'a, T> {
    current: &'a List<T>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            List::Cons(val, next) => {
                self.current = next;
                Some(val)
            }
            List::Nil => None,
        }
    }
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;

        for val in self.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }

            write!(f, "{}", val)?;
        }

        write!(f, "]")
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> ListIterator<T> {
        ListIterator { current: self }
    }

    pub fn add(&mut self, val: T) {
        let mut pointer = self;

        while let List::Cons(_, next) = pointer {
            pointer = next;
        }

        *pointer = List::Cons(val, Box::new(List::Nil));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut pointer = self;

        loop {
            match pointer {
                List::Cons(_, next) if next.is_nil() => break,
                List::Cons(_, next) => {
                    pointer = next;
                },
                List::Nil => break,
            }
        }

        let last_item = mem::replace(pointer, List::Nil);

        match last_item {
            List::Cons(val, _) => Some(val),
            List::Nil => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self {
            List::Cons(value, _) => Some(value),
            List::Nil => None,
        }
    }

    pub fn is_nil(&self) -> bool {
        if let List::Nil = self {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::List::*;

    #[test]
    fn test_display() {
        let list = Cons(2, Box::new(Cons(10, Box::new(Nil))));
        let list2 = Cons("hello", Box::new(Cons("I'm a", Box::new(Cons("Cons list", Box::new(Nil))))));

        assert_eq!("[2, 10]", list.to_string());
        assert_eq!("[hello, I'm a, Cons list]", list2.to_string());
    }

    #[test]
    fn test_add_and_pop() {
        let mut list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

        assert_eq!("[1, 2]", list.to_string());
        assert_eq!(2, list.iter().count());

        list.add(3);

        assert_eq!("[1, 2, 3]", list.to_string());
        assert_eq!(3, list.iter().count());

        let v = list.pop();

        assert_eq!(Some(3), v);
        assert_eq!("[1, 2]", list.to_string());
        assert_eq!(2, list.iter().count());

        let v = list.pop();
        assert_eq!(Some(2), v);
        let v = list.pop();
        assert_eq!(Some(1), v);
        let v = list.pop();
        assert_eq!(None, v);

        assert_eq!("[]", list.to_string());
        assert_eq!(0, list.iter().count());
    }

    #[test]
    fn test_add_and_pop_2() {
        let mut list: List<i32> = Nil;

        assert_eq!(None, list.peek());
        
        assert!(Vec::<i32>::new().iter().eq(list.iter()));

        list.add(42);
        assert!(vec![42].iter().eq(list.iter()));

        list.add(101);
        assert!(vec![42, 101].iter().eq(list.iter()));

        assert_eq!(42, *list.peek().unwrap());

        list.pop();
        assert!(vec![42].iter().eq(list.iter()));

        list.pop();
        assert!(Vec::<i32>::new().iter().eq(list.iter()));

        let long_list = vec![83, 1, 23, -9, 23, 1, 0, -90];
        long_list.iter().for_each(|v| list.add(*v));

        assert!(long_list.iter().eq(list.iter()));
    }
}
