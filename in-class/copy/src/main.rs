#[derive(Debug, Clone, PartialEq)]
struct MyStruct {
    x: i32,
    y: Box<bool>
}

fn main() {
    let a: MyStruct =
    	MyStruct { x: 123, y: Box::new(true) };

    let b = a.clone();

    // let a: i32 = 123;
    // let b = a;

    println!("{:?}", a);
    println!("{}", a == b);
}
