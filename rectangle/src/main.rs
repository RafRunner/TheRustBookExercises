use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// the PartialOrd trait can be derived but we implement it here
impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.height.partial_cmp(&other.height) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.width.partial_cmp(&other.width)
    }
}

fn main() {
    let scale = 1;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("rec1 1 is {:#?}", rect1);
    println!("The area of the rectangle is {} pixels!", rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigger_can_holder_smaller() {
        let bigger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(bigger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_bigger() {
        let smaller = Rectangle {
            width: 5,
            height: 10,
        };

        let bigger = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(!smaller.can_hold(&bigger));
    }
    
    #[test]
    fn can_compare() {
        let rect1 = Rectangle {
            width: 2,
            height: 3,
        };

        let rect2 = Rectangle {
            width: 2,
            height: 5,
        };

        let rect3 = Rectangle {
            width: 9,
            height: 1,
        };

        let rect4 = Rectangle {
            width: 1,
            height: 1,
        };

        let rect5 = Rectangle {
            width: 10,
            height: 10,
        };

        let equal = rect1.clone();

        // rect1 and 2
        assert_eq!(rect1.partial_cmp(&rect2), Some(Ordering::Less));
        assert_eq!(rect2.partial_cmp(&rect1), Some(Ordering::Greater));

        // rect1 and 3
        assert_eq!(rect1.partial_cmp(&rect3), Some(Ordering::Greater));
        assert_eq!(rect3.partial_cmp(&rect1), Some(Ordering::Less));

        // rect1 and 4
        assert_eq!(rect1.partial_cmp(&rect4), Some(Ordering::Greater));
        assert_eq!(rect4.partial_cmp(&rect1), Some(Ordering::Less));

        // rect1 and 5
        assert_eq!(rect1.partial_cmp(&rect5), Some(Ordering::Less));
        assert_eq!(rect5.partial_cmp(&rect1), Some(Ordering::Greater));

        // rect1 and it's clone
        assert_eq!(rect1.partial_cmp(&equal), Some(Ordering::Equal));
        assert_eq!(equal.partial_cmp(&rect1), Some(Ordering::Equal));
    }
}