use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error type returned by stack allocation methods.
#[derive(Debug)]
pub enum StackError {
    /// Contains the maximum amount of memory allowed to be allocated as stack space.
    ExceedsMaximumSize(usize),

    /// Returned if some kind of I/O error happens during allocation.
    IoError(io::Error),
}

impl Display for StackError {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match *self {
            StackError::ExceedsMaximumSize(size) => {
                write!(fmt, "Requested more than max size of {} bytes for a stack", size)
            }
            StackError::IoError(ref e) => e.fmt(fmt),
        }
    }
}

impl Error for StackError {
    fn description(&self) -> &str {
        match *self {
            StackError::ExceedsMaximumSize(_) => "exceeds maximum stack size",
            StackError::IoError(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            StackError::ExceedsMaximumSize(_) => None,
            StackError::IoError(ref e) => Some(e),
        }
    }
}