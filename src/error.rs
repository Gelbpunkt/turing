use crate::{program::State, tape::Segment};

/// Error returned when parsing a [`crate::Program`] fails or a check
/// is violated.
#[derive(Debug)]
pub enum InvalidProgram {
    /// A transition was missing a from state.
    MissingFrom,
    /// A transition was missing a to state.
    MissingTo,
    /// A transition was missing a condition.
    MissingCondition,
    /// A transition was missing a write segment.
    MissingWrite,
    /// A transition was missing a movement action.
    MissingAction,
    /// A state could not be parsed, because it is not a valid integer.
    InvalidState,
    /// A segment could not be parsed, because it is not "1", "0", "_" or " ".
    InvalidSegment,
    /// An action could not be parsed, because it is not "r", "l", "n" in upper-
    /// or lowercase.
    InvalidAction,
    /// The program is missing an initial state.
    MissingInitialState,
}

/// An error returned by executing a program with a [`crate::TuringMachine`].
#[derive(Debug)]
pub enum ExecutionError {
    /// No transition is defined for the current state and segment.
    UndefinedBehavior(State, Segment),
    /// Error state was reached.
    ReachedError(State),
}
