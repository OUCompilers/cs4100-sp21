enum List<T> {
    Nil,
    Cons(T, Box::<List<T>>)
}
use List::*;

fn length<T>(l: List<T>) -> usize {
    match l {
	Nil => 0,
	Cons(_, tl) => 1 + length(*tl)
    }
}

impl<T> List<T> {
    fn len(&self) -> usize {
	match self {
	    Nil => 0,
	    Cons(_, tl) => 1 + tl.len()
	}
    }
}

fn main() {
    let l: List<i32> = Cons(1, Box::new(Cons(3, Box::new(Nil))));
    println!("{}", l.len());
	
    let l2: List<bool> = Cons(true, Box::new(Cons(false, Box::new(Nil))));
    println!("{}", l2.len());
}
