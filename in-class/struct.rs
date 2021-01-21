#[derive(Debug)]
struct Dummy {
    a : i32,
    b : i32
}

fn bar(new_res : Box<Dummy>) {
    println!("{}", new_res.a);
    println!("{}", new_res.b);
}

fn foo() {
    let mut res = Box::new(Dummy { a: 0, b: 0 });
    res.a = 2048;
    // bar(res);
    // println!("{}", res.a)
}

fn main() {
    foo();
}
