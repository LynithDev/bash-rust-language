pub type CursorTuple = (u16, u16);

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cursor {
    pub col: u16,
    pub line: u16,
    index: u32,
}

impl Cursor {
    pub fn create() -> Self {
        Self {
            col: 1,
            line: 1,
            index: 0,
        }
    }

    pub fn from(line: u16, col: u16) -> Self {
        Self {
            col,
            line,
            index: 0
        }
    }

    pub fn from_full(col: u16, line: u16, index: u32) -> Self {
        Self {
            col,
            line,
            index
        }
    }

    /// Goes to the new line if needed, based on the character
    pub fn next(&mut self, char: &char) {
        if char.eq(&'\n') {
            self.next_line();
        } else {
            self.next_col();
        }
    }
    
    /// Moves the cursor to the next column
    pub fn next_col(&mut self) {
        self.col += 1;

        self.index += 1;
    }
    
    /// Moves the cursor to the next line and resets the column to 0
    pub fn next_line(&mut self) {
        self.line += 1;
        self.col = 1;

        self.index += 1;
    }
    
    /// Gets the index in the input file of the cursor
    pub fn index(&self) -> u32 {
        self.index
    } 
    
    /// Resets the cursor back to line 1 column 1
    pub fn reset(&mut self) {
        self.col = 1;
        self.line = 1;
    }

    /// Returns a (line, col) tuple
    pub fn to_tuple(&self) -> CursorTuple {
        (self.line, self.col)
    } 
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.line == other.line
    }
}

impl From<Cursor> for CursorTuple {
    fn from(val: Cursor) -> Self {
        val.to_tuple()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithCursor<T> {
    pub value: T,
    pub cursor: Cursor
}

impl<T> WithCursor<T> {
    pub fn create(value: T) -> Self {
        Self {
            value,
            cursor: Cursor::create()
        }
    }
}
