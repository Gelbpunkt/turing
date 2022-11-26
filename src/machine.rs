use crate::{tape::Tape, Program, State, ExecutionError, Move};

/// The actual turing machine that can execute [`Program`]s.
#[derive(Debug)]
pub struct TuringMachine<T> {
    tape: T,
}

impl<T> TuringMachine<T>
where
    T: Tape,
{
    /// Create a new [`TuringMachine`] from a [`Tape`].
    #[must_use]
    pub fn from_tape(tape: T) -> Self {
        Self { tape }
    }

    /// Returns a reference to the internal [`Tape`] used by the machine.
    #[must_use]
    pub fn tape(&self) -> &T {
        &self.tape
    }

    /// Returns a mutable reference to the internal [`Tape`] used by the
    /// machine.
    #[must_use]
    pub fn tape_mut(&mut self) -> &mut T {
        &mut self.tape
    }

    /// Run a [`Program`] with this turing machine.
    ///
    /// # Errors
    ///
    /// This method will error if it encounters undefined behaviour or reaches
    /// an error state.
    pub fn execute(&mut self, program: &Program) -> Result<State, ExecutionError> {
        let mut state = program.initial_state;

        // Find the next transition
        loop {
            let current = self.tape.current();
            let transition = program
                .transitions
                .get(&(state, *current))
                .ok_or(ExecutionError::UndefinedBehavior(state, *current))?;

            self.tape.put(transition.write);

            match transition.action {
                Move::Left => self.tape.left(),
                Move::Right => self.tape.right(),
                Move::Nothing => {}
            }

            state = transition.to;

            if program.final_states.contains(&state) {
                break;
            }

            if program.error_states.contains(&state) {
                return Err(ExecutionError::ReachedError(state));
            }
        }

        Ok(state)
    }
}
