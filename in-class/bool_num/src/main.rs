use Term::*;
use Type::*;

/// Syntax of terms from TAPL Ch3.
#[derive(Debug, Clone, PartialEq)]
enum Term {
    True,
    False,
    Ite(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    Iszero(Box<Term>)
}

impl Term {
    fn is_val(&self) -> bool {
        match self {
            True  => true,
            False => true,
            Zero  => true,
            Succ(t) => t.is_val(),
            _ => false
        }
    }
    fn to_bool(&self) -> Option<Term> {
        match self {
            True => Some(True),
            False => Some(False),
            _ => None
        }
    }
    fn to_num(&self) -> Option<Term> {
        match self {
            Zero => Some(Zero),
            Succ(t) => t.to_num().and_then(|n| Some(Succ(Box::new(n)))),
            _ => None
        }
    }
}

/// Step function on terms derived from the small-step semantics given
/// in TAPL ch3.
fn step(t: &Term) -> Term {
    match t {
        Ite(t1, t2, t3) => match t1.to_bool() {
            // E-IFTRUE
            Some(True) => *t2.clone(),
            // E-IFFALSE
            Some(False) => *t3.clone(),
            // E-IF
            _ => Ite(Box::new(step(t1)), t2.clone(), t3.clone())
        }
        // E-SUCC
        Succ(t1) => Succ(Box::new(step(t1))),
        Pred(t1) => match t1.to_num() {
            // E-PREDZERO
            Some(Zero) => Zero,
            // E-PREDSUCC
            Some(Succ(n)) => *n.clone(),
            // E-PRED
            _ => Pred(Box::new(step(t1)))
        }
        Iszero(t1) => match t1.to_num() {
            // E-ISZERO-ZERO
            Some(Zero) => True,
            // E-ISZERO-SUCC
            Some(_) => False,
            // E-ISZERO
            _ => Iszero(Box::new(step(t1)))
        }
        // The remaining cases are normal forms (values actually).
        _ => t.clone()
    }
}

/// Multistep function corresponding to the reflexive transitive
/// closure of the small-step relation.
fn star_step(t: &Term) -> Term {
    let mut cur_t = t.clone();
    let mut next_t = step(&cur_t);
    while next_t != cur_t {
        cur_t = next_t;
        next_t = step(&cur_t)
    }
    next_t
}

fn eval(t: &Term) -> Result<Term, ()> {
    if t.is_val() {
        // E-VAL
        Ok(t.clone())
    } else {
        match t {
            Ite(t1, t2, t3) => match eval(t1)?.to_bool() {
                // E-IFTRUE
                Some(True) => eval(t2),
                // E-IFFALSE
                Some(False) => eval(t3),
                // error
                _ => Err(())
            }
            Succ(t1) => match eval(t1)?.to_num() {
                // E-SUCC
                Some(n) => Ok(Succ(Box::new(n))),
                None => Err(())
            },
            Pred(t1) => match eval(t1)?.to_num() {
                // E-PREDZERO
                Some(Zero) => Ok(Zero),
                // E-PREDSUCC
                Some(Succ(n)) => Ok(*n),
                // error
                _ => Err(())
            },
            Iszero(t1) => match eval(t1)?.to_num() {
                // E-ISZEROZERO
                Some(Zero) => Ok(True),
                // E-ISZEROSUCC
                Some(_) => Ok(False),
                // error
                None => Err(())
            },
            // Remaining cases are values so this is unreachable.
            _ => Err(())
        }
    }
}

fn run_smallstep(t: &Term) {
    println!("{:?}", star_step(t));
}

fn run_bigstep(t: &Term) {
    println!("{:?}", eval(t))
}

fn main() {
    let t1 = Ite(Box::new(True), Box::new(Zero), Box::new(Succ(Box::new(Zero))));
    let t2 = Ite(Box::new(False), Box::new(Zero), Box::new(Succ(Box::new(Zero))));

    run_smallstep(&t1);
    run_bigstep(&t1);

    run_smallstep(&t2);
    run_bigstep(&t2);
}


/// Syntax of types from TAPL ch8.
#[derive(Debug, Clone, PartialEq)]
enum Type {
    Bool,
    Num
}

/// Typechecker following the typing rules given in TAPL ch8.
fn tycheck(t: &Term) -> Result<Type, ()> {
    match t {
        True => Ok(Bool),
        False => Ok(Bool),
        Ite(t1, t2, t3) => match (tycheck(t1)?, tycheck(t2)?, tycheck(t3)?) {
            (Bool, ty2, ty3) => if ty2 == ty3 {
                Ok(ty2)
            } else {
                Err(())
            }
            _ => Err(())
        }
        Zero => Ok(Num),
        Succ(t1) => if let Num = tycheck(t1)? {
            Ok(Num)
        } else {
            Err(())
        }
        Pred(t1) => if let Num = tycheck(t1)? {
            Ok(Num)
        } else {
            Err(())
        }
        Iszero(t1) => if let Num = tycheck(t1)? {
            Ok(Bool)
        } else {
            Err(())
        }
    }
}
