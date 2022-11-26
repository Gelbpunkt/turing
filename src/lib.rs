#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]
pub use crate::{error::*, machine::*, program::*, tape::*};

pub mod error;
pub mod machine;
pub mod program;
pub mod tape;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::*;

    #[test]
    fn test_next_integer() {
        let program = Program::from_str(include_str!("../examples/next_integer.tng")).unwrap();
        let tape = VecTape::from_str("_111_").unwrap();
        let mut machine = TuringMachine::from_tape(tape);
        machine.execute(&program).unwrap();
        assert_eq!(
            machine.tape().inner,
            VecTape::from_str("1000_").unwrap().inner
        );
    }

    #[test]
    fn test_append() {
        let program = Program::from_str(include_str!("../examples/append.tng")).unwrap();
        let tape = VecTape::from_str("_111_").unwrap();
        let mut machine = TuringMachine::from_tape(tape);
        machine.execute(&program).unwrap();
        assert_eq!(
            machine.tape().inner,
            VecTape::from_str("_11101").unwrap().inner
        );
    }

    #[test]
    fn test_palindrome() {
        let program = Program::from_str(include_str!("../examples/palindrome.tng")).unwrap();
        let tape = VecTape::from_str("_110000011_").unwrap();
        let mut machine = TuringMachine::from_tape(tape);
        assert!(machine.execute(&program).is_ok());
    }

    #[test]
    fn test_copy() {
        let program = Program::from_str(include_str!("../examples/copy.tng")).unwrap();
        let tape = VecTape::from_str("_111111_").unwrap();
        let mut machine = TuringMachine::from_tape(tape);
        machine.execute(&program).unwrap();
        assert_eq!(
            machine.tape().inner,
            VecTape::from_str("_111111_111111").unwrap().inner
        );
    }
}
