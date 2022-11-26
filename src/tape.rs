use std::{
    collections::VecDeque,
    fmt::{self, Write},
    str::FromStr,
};

use crate::error::InvalidProgram;

/// A segment on the infinite [`Tape`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Segment {
    Zero,
    One,
    Empty,
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

/// An infinite working buffer for the [`crate::TuringMachine`].
///
/// Advancing the tape past the known segments will create
/// empty segments dynamically.
pub trait Tape {
    /// Advance the cursor to the right by one.
    fn right(&mut self);

    /// Advance the cursor to the left by one.
    fn left(&mut self);

    /// Write to the segment at the cursor position.
    fn put(&mut self, segment: Segment);

    /// View the segment at the cursor position.
    fn current(&self) -> &Segment;
}

/// A [`Tape`] backed by a [`Vec`].
#[derive(Debug, PartialEq, Eq)]
pub struct VecTape {
    pub(crate) inner: Vec<Segment>,
    position: usize,
}

impl VecTape {
    /// Create a new tape with a known part of the tape and a
    /// specific cursor position.
    ///
    /// # Panics
    ///
    /// This method will panic if the position is outside of the tape segment.
    #[must_use]
    pub fn new(inner: Vec<Segment>, position: usize) -> Self {
        assert!(position < inner.len());
        Self { inner, position }
    }
}

impl Tape for VecTape {
    fn right(&mut self) {
        self.position += 1;

        if self.position == self.inner.len() {
            self.inner.push(Segment::Empty);
        }
    }

    fn left(&mut self) {
        if self.position == 0 {
            self.inner.insert(0, Segment::Empty);
        } else {
            self.position -= 1;
        }
    }

    fn put(&mut self, segment: Segment) {
        self.inner[self.position] = segment;
    }

    fn current(&self) -> &Segment {
        &self.inner[self.position]
    }
}

impl FromStr for VecTape {
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

impl fmt::Display for VecTape {
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

/// A [`Tape`] backed by a [`VecDeque`].
#[derive(Debug, PartialEq, Eq)]
pub struct VecDequeTape {
    pub(crate) inner: VecDeque<Segment>,
    position: usize,
}

impl VecDequeTape {
    /// Create a new tape with a known part of the tape and a
    /// specific cursor position.
    ///
    /// # Panics
    ///
    /// This method will panic if the position is outside of the tape segment.
    #[must_use]
    pub fn new(inner: VecDeque<Segment>, position: usize) -> Self {
        assert!(position < inner.len());
        Self { inner, position }
    }
}

impl Tape for VecDequeTape {
    fn right(&mut self) {
        self.position += 1;

        if self.position == self.inner.len() {
            self.inner.push_back(Segment::Empty);
        }
    }

    fn left(&mut self) {
        if self.position == 0 {
            self.inner.push_front(Segment::Empty);
        } else {
            self.position -= 1;
        }
    }

    fn put(&mut self, segment: Segment) {
        self.inner[self.position] = segment;
    }

    fn current(&self) -> &Segment {
        &self.inner[self.position]
    }
}

impl FromStr for VecDequeTape {
    type Err = InvalidProgram;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = VecDeque::with_capacity(s.len());
        let mut position = 0;

        for (idx, part) in s.chars().enumerate() {
            match part {
                '1' => {
                    inner.push_back(Segment::One);

                    if position == 0 {
                        position = idx;
                    }
                }
                '0' => {
                    inner.push_back(Segment::Zero);

                    if position == 0 {
                        position = idx;
                    }
                }
                '_' | ' ' => inner.push_back(Segment::Empty),
                _ => return Err(InvalidProgram::InvalidSegment),
            }
        }

        Ok(Self { inner, position })
    }
}

impl fmt::Display for VecDequeTape {
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
