use smart_pointers::box_demo::List::*;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(32, Box::new(Nil))))));
    println!("{}", list);
}
