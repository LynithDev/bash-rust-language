use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Copy, Eq)]
pub struct Cursor(u16, u16, u32);

impl Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cursor({}:{}@{})", self.0, self.1, self.2)
    }
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Hash for Cursor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Cursor {
    pub fn create() -> Self {
        Self::from(1, 1)
    }

    pub fn from(col: u16, line: u16) -> Self {
        Self::from_full(col, line, 0)
    }

    pub fn from_full(col: u16, line: u16, index: u32) -> Self {
        Self(col, line, index)
    }
    
    /// Moves the cursor
    pub fn next_col(&mut self) {
        self.1 += 1;
        self.2 += 1;
    }

    pub fn next_line(&mut self) {
        self.0 += 1;
        self.1 = 1;
        self.2 += 1;
    }
    
    /// Resets the cursor
    pub fn reset(&mut self) {
        *self = Self::create();
    }

    /// Gets the line
    pub fn line(&self) -> u16 {
        self.0
    }

    /// Gets the column
    pub fn col(&self) -> u16 {
        self.1
    }
    
    /// Gets the index in the input file of the cursor
    pub fn index(&self) -> u32 {
        self.2
    } 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WithCursor<T> {
    pub value: T,
    pub start: Cursor,
    pub end: Cursor,
}

impl<T> WithCursor<T> {
    pub fn create(value: T) -> Self {
        Self::create_with(Cursor::create(), Cursor::create(), value)
    }

    pub fn create_with(start: Cursor, end: Cursor, value: T) -> Self {
        Self {
            start,
            end,
            value,
        }
    }
}
