#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: bool
}

impl MyStruct {
    fn get_x(&self) -> i32 {
	self.x
    }
    fn new() -> MyStruct {
	MyStruct { x: 0, y: false }
    }
}

enum MyEnum {
    A(i32),
    B(bool)
}
use MyEnum::*;

fn main() {
    let a: MyStruct =
	MyStruct { x: 123, y: true };

    let b: MyEnum = A(123);
    // let c: MyEnum = B(false);
    
    println!("{:?}", a);
    println!("{:?}", a.get_x());
    println!("{:?}", a.y);

    // let d: MyStruct = MyStruct::new();
    // println!("{:?}", d);

    match b {
	A(i) if i == 123 =>
	    println!("matched 123!"),
	A(i) => println!("{:?}", i),
	B(b) => println!("{:?}", b)
    }
}
