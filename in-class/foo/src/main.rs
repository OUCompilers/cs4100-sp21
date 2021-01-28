trait HasFoo {
    fn foo(&self);
    fn foofoo(&self) {
	self.foo();
	self.foo()
    }
}

impl HasFoo for i32 {
    fn foo(&self) {
	println!("i32 {}", self)
    }
}

impl HasFoo for bool {
    fn foo(&self) {
	println!("bool {}", self)
    }
}

impl<T: HasFoo> HasFoo for Vec<T> {
    fn foo(&self) {
	for x in self {
	    x.foo()
	}
    }
}

fn main() {
    123.foofoo();
    true.foo();

    let v: Vec<i32> = vec![1, 5, 7];
    v.foo();

    "hello".into()
}
