#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// Cancel panic
    Cancel,
    /// Type mismatch panic
    TypeErr,
    /// Stack overflow panic
    StackErr,
    /// Wrong Context panic
    ContextErr,
}