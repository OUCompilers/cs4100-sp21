use std::io::{self, BufRead, BufReader};

// Read integers on stdin, printing them (in reverse) to stdout.
fn main() -> io::Result<()> {
    // Allocate a new mutable stack (to hold the i32's).
    let mut stack : Vec<i32> = Vec::new();
    
    // Initialize a read buffer on stdin.
    let reader = BufReader::new(io::stdin());
    
    // Read the integers.
    'lines: for line in reader.lines() {
        for tok in line.unwrap().split_whitespace() {
	    match tok {
		"done" => break 'lines,
		_ => {
		    let i = tok.parse().expect("expected an i32");
		    stack.push(i)
		}
	    }
	}
    };
    
    // Print the integers in reverse.
    while let Some(i) = stack.pop() {
    	println!("{}", i)
    };

    // let s = stack.iter().fold(String::from(""), |acc, x| x.to_string() + " " + &acc);
    // let s = stack.iter().fold(String::from(""), |acc, x| acc + " " + &x.to_string());
    // println!("{}", s.trim());
    
    Ok(())
}
