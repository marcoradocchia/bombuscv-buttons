use std::fmt::{self, Display, Formatter};

/// Enum representing handled runtime errors.
#[derive(Debug)]
pub enum ErrorKind {
    /// Occurs when list of system processes could not be retrieved.
    ProcListErr,

    /// Occurs when forking of child process fails.
    ForkFail(String),

    /// Occurs when spawning process fails.
    SpawnFail(String),
}

/// Implementing Display trait for ErrorKind enum.
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcListErr => write!(f, "unable to retrieve process list"),
            Self::ForkFail(bin) => write!(f, "unable to fork child process `{}`", bin),
            Self::SpawnFail(bin) => write!(f, "unable to spawn process `{}`", bin),
        }
    }
}
