use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{error::InvalidProgram, tape::Segment};

/// An movement action in a program.
#[derive(Debug)]
pub(crate) enum Move {
    Left,
    Right,
    Nothing,
}

/// A state in a [`Program`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct State(usize);

/// A transition in a [`Program`].
///
/// If the transition matches the [`crate::TuringMachine`]'s current
/// state, it will write to the tape and move the cursor.
#[derive(Debug)]
pub(crate) struct Transition {
    pub(crate) from: State,
    pub(crate) to: State,
    pub(crate) condition: Segment,
    pub(crate) write: Segment,
    pub(crate) action: Move,
}

/// A program for the [`crate::TuringMachine`].
///
/// Each program has:
///     - Exactly one initial [`State`], denoted by "+" followed by a state
///       number
///     - Any amount of final [`State`]s, denoted by any amount of "-" followed
///       by a state number
///     - Any amount of error [`State`]s, denoted by any amount of "+" followed
///       by a state number
///     - Any amount of comments, which are ignored and start with "#" or "/"
///     - Any amount of transitions, which have comma-seperated values:
///         - The "from" state
///         - The "to" state
///         - The segment to match
///         - The segment to write
///         - The movement action to perform
///
/// Simple example:
/// ```tng
/// ## This program adds 1 to a binary number.
/// ## Input format: Binary number with empty spaces at the sides
/// ## Initial state
/// +0
/// ## End state
/// -3
/// ## Program format: from,to,condition,write,action
///
/// ## State 0 is for moving right until the first empty segment
/// 0,0,0,0,r
/// 0,0,1,1,r
/// 0,1,_,_,l
/// ## State 1 is for flipping bits and moving left until a 0 is turned into a 1
/// 1,2,0,1,l
/// 1,1,1,0,l
/// ## Special case: The number is all 1s, in which case we arrive at a blank and turn it into a 1
/// 1,3,_,1,n
/// ## State 2 is for moving left until the blank
/// 2,2,0,0,l
/// 2,2,1,1,l
/// ## First bit reached again, go to end state 3!
/// 2,3,_,_,r
/// ```
#[derive(Debug)]
pub struct Program {
    pub(crate) initial_state: State,
    pub(crate) final_states: HashSet<State>,
    pub(crate) error_states: HashSet<State>,
    pub(crate) transitions: HashMap<(State, Segment), Transition>,
}

impl Program {
    fn from_parts(
        initial_state: State,
        final_states: HashSet<State>,
        error_states: HashSet<State>,
        transitions: HashMap<(State, Segment), Transition>,
    ) -> Self {
        Self {
            initial_state,
            final_states,
            error_states,
            transitions,
        }
    }
}

impl FromStr for State {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map_or_else(|_| Err(InvalidProgram::InvalidState), |v| Ok(Self(v)))
    }
}

impl FromStr for Move {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "R" => Ok(Self::Right),
            "l" | "L" => Ok(Self::Left),
            "n" | "N" | "" | "_" | " " => Ok(Self::Nothing),
            _ => Err(InvalidProgram::InvalidAction),
        }
    }
}

impl FromStr for Transition {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let from = parts.next().ok_or(InvalidProgram::MissingFrom)?;
        let to = parts.next().ok_or(InvalidProgram::MissingTo)?;
        let condition = parts.next().ok_or(InvalidProgram::MissingCondition)?;
        let write = parts.next().ok_or(InvalidProgram::MissingWrite)?;
        let action = parts.next().ok_or(InvalidProgram::MissingAction)?;

        Ok(Self {
            from: State::from_str(from)?,
            to: State::from_str(to)?,
            condition: Segment::from_str(condition)?,
            write: Segment::from_str(write)?,
            action: Move::from_str(action)?,
        })
    }
}

impl FromStr for Program {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut transitions = HashMap::new();
        let mut initial_state = None;
        let mut final_states = HashSet::with_capacity(1);
        let mut error_states = HashSet::new();

        for line in s.lines() {
            // Skip comments
            if line.starts_with('#') || line.starts_with('/') || line.is_empty() {
                continue;
            }

            // SAFETY: We verified that the line is not empty
            match line.chars().next().unwrap() {
                '+' => {
                    initial_state = Some(State::from_str(&line[1..])?);
                }
                '-' => {
                    final_states.insert(State::from_str(&line[1..])?);
                }
                '!' => {
                    error_states.insert(State::from_str(&line[1..])?);
                }
                _ => {
                    let transition = Transition::from_str(line)?;
                    transitions.insert((transition.from, transition.condition), transition);
                }
            }
        }

        Ok(Self::from_parts(
            initial_state.ok_or(InvalidProgram::MissingInitialState)?,
            final_states,
            error_states,
            transitions,
        ))
    }
}
