use std::env;

fn main() {
    let av: Vec<String> = env::args().collect();
    println!("args = {:?}", av[1]);
}
