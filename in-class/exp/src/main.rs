use sexp::*;

#[derive(Debug)]
enum Binop {
    Plus,
    Minus,
    Mult,
    Div
}
use Binop::*;

#[derive(Debug)]
enum Exp {
    Num(i32),
    Op(Binop, Box<Exp>, Box<Exp>)
}
use Exp::*;

fn parse_sexp(s: &Sexp) -> Exp {
    match s {
	Sexp::Atom(Atom::I(i)) => Num(*i as i32),
	Sexp::List(v) => {
	    let e1 = Box::new(parse_sexp(&v[1]));
	    let e2 = Box::new(parse_sexp(&v[2]));
	    let op = match &v[0] {
		Sexp::Atom(Atom::S(s)) if s == "+" =>
		    Plus,
		Sexp::Atom(Atom::S(s)) if s == "-" =>
		    Minus,
		Sexp::Atom(Atom::S(s)) if s == "*" =>
		    Mult,
		Sexp::Atom(Atom::S(s)) if s == "/" =>
		    Div,
		_ => panic!("parse error")
	    };
	    Op(op, e1, e2)
	}
	_ => panic!("parse error")
    }
}

impl Exp {
    fn parse(s: &str) -> Exp {
	parse_sexp(&sexp::parse(s)
		   .expect("sexp parse error"))
    }
	
    fn eval(&self) -> i32 {
	match self {
	    Num(i) => *i,
	    Op(op, e1, e2) => {
		let i1 = e1.eval();
		let i2 = e2.eval();
		match op {
		    Plus => i1 + i2,
		    Minus => i1 - i2,
		    Mult => i1 * i2,
		    Div => {
			if i2 == 0 {
			    panic!("division by zero")
			} else {
			    i1 / i2
			}
		    }
		}
	    }
	}
    }

    fn to_rpn(&self) -> String {
	match self {
	    Num(i) => i.to_string(),
	    Op(op, e1, e2) =>
		e1.to_rpn() + " " + &e2.to_rpn() +
		match op {
		    Plus  => " +",
		    Minus => " -",
		    Mult  => " *",
		    Div   => " /"
		}
	}
    }
}

fn main() {
    let e: Exp = Exp::parse("(+ 2 5)");
    println!("{:?}", e);
    println!("{}", e.eval());

    let e2: Exp = Exp::parse("(+ (- 2 1) (/ 10 2))");
    println!("{}", e2.eval());
    println!("{}", e2.to_rpn());
}
