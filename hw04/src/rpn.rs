use std::result;
use std::io;
use rand::{thread_rng, Rng};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y.
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

#[derive(Debug)]
pub struct Stack {
    elts: Vec<Elt>,
}

pub type Result<T> = result::Result<T, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        Stack { elts: Vec::new() }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        self.elts.push(val);
        Ok(())
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        self.elts.pop().ok_or(Error::Underflow)
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => {
                // Adds two numbers: pop x, pop y, push x + y.
                if let (Elt::Int(x), Elt::Int(y)) = (try!(self.pop()), try!(self.pop())) {
                    self.push(Elt::Int(x + y))
                } else {
                    Err(Error::Type)
                }
            },
            Op::Eq => {
                // Checks equality of two values: pop x, pop y, push x == y.
                match (try!(self.pop()), try!(self.pop())) {
                    (Elt::Int(x), Elt::Int(y)) => {
                        self.push(Elt::Bool(x == y))
                    },
                    (Elt::Bool(x), Elt::Bool(y)) => {
                        self.push(Elt::Bool(x == y))
                    },
                    _ => {
                        Err(Error::Type)
                    }
                }
            },
            Op::Neg => {
                // Negates a value: pop x, push ~x.
                match try!(self.pop()) {
                    Elt::Bool(x) => {
                        self.push(Elt::Bool(!x))
                    },
                    Elt::Int(x) => {
                        self.push(Elt::Int(-x))
                    }
                }
            },
            Op::Swap => {
                // Swaps two values: pop x, pop y, push x, push y.
                let x = try!(self.pop());
                let y = try!(self.pop());

                try!(self.push(x));
                try!(self.push(y));
                Ok(())
            },
            Op::Rand => {
                // Computes a random number: pop x, push random number in [0, x).
                if let Elt::Int(x) = try!(self.pop()) {
                    if x <= 0 {
                        Err(Error::Type)
                    } else {
                        self.push(Elt::Int(thread_rng().gen_range(0, x)))
                    }
                } else {
                    Err(Error::Type)
                }
            },
            Op::Quit => {
                // Quit the calculator.
                Err(Error::Quit)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res { } else { assert!(false); }
    }
}
