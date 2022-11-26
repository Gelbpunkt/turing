#![deny(clippy::pedantic)]
#![doc = include_str!("../README.md")]

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Write},
    str::FromStr,
};

/// A segment on the infinite [`Band`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Segment {
    Zero,
    One,
    Empty,
}

/// An infinite working buffer for the [`TuringMachine`].
///
/// Advancing the band past the known segments will create
/// empty segments dynamically.
#[derive(Debug)]
pub struct Band {
    inner: Vec<Segment>,
    position: usize,
}

impl Band {
    /// Create a new band with a known part of the band and a
    /// specific cursor position.
    ///
    /// # Panics
    ///
    /// This method will panic if the position is outside of the band segment.
    #[must_use]
    pub fn new(inner: Vec<Segment>, position: usize) -> Self {
        assert!(position < inner.len());
        Self { inner, position }
    }

    /// Advance the cursor to the right by one.
    pub fn right(&mut self) {
        self.position += 1;

        if self.position == self.inner.len() {
            self.inner.push(Segment::Empty);
        }
    }

    /// Advance the cursor to the left by one.
    pub fn left(&mut self) {
        if self.position == 0 {
            self.inner.insert(0, Segment::Empty);
        } else {
            self.position -= 1;
        }
    }

    /// Write to the segment at the cursor position.
    pub fn put(&mut self, segment: Segment) {
        self.inner[self.position] = segment;
    }

    /// View the segment at the cursor position.
    #[must_use]
    pub fn current(&self) -> &Segment {
        &self.inner[self.position]
    }
}

impl FromStr for Band {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Vec::with_capacity(s.len());
        let mut position = 0;

        for (idx, part) in s.chars().enumerate() {
            match part {
                '1' => {
                    inner.push(Segment::One);

                    if position == 0 {
                        position = idx;
                    }
                }
                '0' => {
                    inner.push(Segment::Zero);

                    if position == 0 {
                        position = idx;
                    }
                }
                '_' | ' ' => inner.push(Segment::Empty),
                _ => return Err(InvalidProgram::InvalidSegment),
            }
        }

        Ok(Self { inner, position })
    }
}

impl fmt::Display for Band {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.inner {
            match segment {
                Segment::One => f.write_char('1')?,
                Segment::Zero => f.write_char('0')?,
                Segment::Empty => f.write_char('_')?,
            }
        }

        Ok(())
    }
}

/// An movement action in a program.
#[derive(Debug)]
enum Move {
    Left,
    Right,
    Nothing,
}

/// A state in a [`Program`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct State(usize);

/// A transition in a [`Program`].
///
/// If the transition matches the [`TuringMachine`]'s current
/// state, it will write to the band and move the cursor.
#[derive(Debug)]
struct Transition {
    from: State,
    to: State,
    condition: Segment,
    write: Segment,
    action: Move,
}

/// Error returned when parsing a program fails or a check
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

/// A program for the [`TuringMachine`].
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
    initial_state: State,
    final_states: HashSet<State>,
    error_states: HashSet<State>,
    transitions: HashMap<(State, Segment), Transition>,
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

impl FromStr for Segment {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::One),
            "0" => Ok(Self::Zero),
            "_" | " " => Ok(Self::Empty),
            _ => Err(InvalidProgram::InvalidSegment),
        }
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

/// An error returned by executing a program with a [`TuringMachine`].
#[derive(Debug)]
pub enum ExecutionError {
    /// No transition is defined for the current state and segment.
    UndefinedBehavior(State, Segment),
    /// Error state was reached.
    ReachedError(State),
}

/// The actual turing machine that can execute [`Program`]s.
#[derive(Debug)]
pub struct TuringMachine {
    band: Band,
}

impl TuringMachine {
    /// Create a new [`TuringMachine`] from a [`Band`].
    #[must_use]
    pub fn from_band(band: Band) -> Self {
        Self { band }
    }

    /// Returns a reference to the internal [`Band`] used by the machine.
    #[must_use]
    pub fn band(&self) -> &Band {
        &self.band
    }

    /// Returns a mutable reference to the internal [`Band`] used by the
    /// machine.
    #[must_use]
    pub fn band_mut(&mut self) -> &mut Band {
        &mut self.band
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
            let current = self.band.current();
            let transition = program
                .transitions
                .get(&(state, *current))
                .ok_or(ExecutionError::UndefinedBehavior(state, *current))?;

            self.band.put(transition.write);

            match transition.action {
                Move::Left => self.band.left(),
                Move::Right => self.band.right(),
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

#[test]
fn test_next_integer() {
    let program = Program::from_str(include_str!("../examples/next_integer.tng")).unwrap();
    let band = Band::from_str("_111_").unwrap();
    let mut machine = TuringMachine::from_band(band);
    machine.execute(&program).unwrap();
    assert_eq!(machine.band.inner, Band::from_str("1000_").unwrap().inner,);
}

#[test]
fn test_append() {
    let program = Program::from_str(include_str!("../examples/append.tng")).unwrap();
    let band = Band::from_str("_111_").unwrap();
    let mut machine = TuringMachine::from_band(band);
    machine.execute(&program).unwrap();
    assert_eq!(machine.band.inner, Band::from_str("_11101").unwrap().inner,);
}

#[test]
fn test_palindrome() {
    let program = Program::from_str(include_str!("../examples/palindrome.tng")).unwrap();
    let band = Band::from_str("_110000011_").unwrap();
    let mut machine = TuringMachine::from_band(band);
    assert!(machine.execute(&program).is_ok());
}
