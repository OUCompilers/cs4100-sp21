fn print_vec(v: &[i32]) {
    for i in v {
	println!("{}", i)
    }
}

fn print_string(s: &str) {
    println!("{}", s)
}

fn main() {
    // let v: Vec<i32> = vec![4, 5, 7, 0, 1];

    // print_vec(&v);

    let s: String = "hello".into();

    let s1: &str = &s[2..4];
    
    print_string(s1);
    
    // println!("{:?}", s);
}
