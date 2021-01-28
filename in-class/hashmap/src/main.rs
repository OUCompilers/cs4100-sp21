use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, i32> = HashMap::new();

    let s: String = "hello".into();

    hm.insert(s.clone(), 123);

    println!("{}", s);
    println!("{:?}", hm)
}
