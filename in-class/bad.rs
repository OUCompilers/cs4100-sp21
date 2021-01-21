fn main() {
    let mut s1 = String::from("hello world");
    let s4 = s1;
    let s2 = &mut s1;

    //println!("{}", s2);
    //println!("{}", s1);    
}

// fn borrow(i : &str) {
//     println!("{}", i)
// }

// fn main() {
//     let mut s1 = String::from("hello");
//     // borrow(&s1);
//     let s2 = &s1;
//     s1 = "hello, world".into();

//     println!("{}", s2);
//     println!("{}", s1);
// }

// fn main() {

//     let mut a = 0;
//     let b = &a;
//     a = 1;
//     println!("a: {}", a);
//     println!("b: {}", b);
// }
