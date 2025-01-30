use std::iter::Peekable;

use crate::error::ErrorList;

pub trait ComponentErrors {
    fn fetch_errors(&self) -> &ErrorList;

    fn has_errors(&self) -> bool {
        !self.fetch_errors().is_empty()
    }

    fn print_errors(&self) {
        for error in self.fetch_errors() {
            println!("{}", error)
        }
    }

    #[cfg(feature = "cli")]
    fn source(&self) -> &crate::error::SourceFile;

    #[cfg(feature = "cli")]
    fn get_source_sliced(&self, start: crate::Cursor, end: crate::Cursor) -> crate::error::SourceFile {
        let (path, input) = *self.source().clone();

        let start_index = start.index() as usize;
        let end_index = end.index() as usize;

        let source = &input[start_index..end_index];

        Box::from((path, source.to_string()))
    }
}

pub trait ComponentIter<'a, C, T, I> where 
    T: PartialEq<C> + PartialEq + Clone + 'a,
    I: Iterator<Item = T> + 'a {

    fn get_iter(&mut self) -> &mut Peekable<I>;
    fn cursor_next(&mut self, item: &T);

    /// Skip the list until an item of the same type in `term` is found
    fn skip_until(&mut self, term: &[T]) {
        while let Some(item) = self.peek() {
            if term.contains(item) {
                break;
            }

            self.next();
        }
    }

    /// Iterates to the next character
    fn next(&mut self) -> Option<T> {
        if let Some(item) = self.get_iter().next() {
            self.cursor_next(&item);
            Some(item.to_owned())
        } else {
            None
        }
    }

    /// Iterates to the next character if the next character is equal to the char argument
    fn next_if_eq(&mut self, item: &C) -> Option<T> {
        if self.peek_is(item) {
            self.next()
        } else {
            None
        }
    }

    /// Expects a character to be there
    fn expect(&mut self, expected: &C) -> std::result::Result<T, Option<T>> {
        let Some(item) = self.next_if_eq(expected) else {
            return Err(None);
        };

        if &item == expected {
            Ok(item)
        } else {
            Err(Some(item))
        }
    }

    /// Checks if the next character is equal to the char argument
    fn peek_is(&mut self, item: &C) -> bool {
        if let Some(peek) = self.peek() {
            peek == item
        } else {
            false
        }
    }

    /// Returns the next character if exists without iterating
    fn peek<'b>(&'b mut self) -> Option<&'b T>
    where I: 'b {
        self.get_iter().peek()
    }
}