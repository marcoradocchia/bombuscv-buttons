use std::fmt::{self, Display, Formatter};

/// Enum representing handled runtime errors.
#[derive(Debug)]
pub enum ErrorKind {
    /// Occurs when list of system processes could not be retrieved.
    ProcListErr,

    /// Occurs when forking of child process fails.
    ForkErr(String),

    /// Occurs when spawning process fails.
    SpawnErr(String),

    /// Occurs when unable to send signal to process.
    SignalErr(String),

    /// Occurs when accessing Raspberry Pi GPIO fails.
    GpioErr,

    /// Occurs when accessing GPIO Pin fails.
    PinErr(String),

    /// Occurs when unable to set interrupt on specified GPIO Pin.
    InterruptErr,

    /// Occurs when unable to join thread.
    ThreadJoinErr(String),

    /// Occurs when unable to poll interrupt.
    PollInterruptErr,
}

/// Implementing Display trait for ErrorKind enum.
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcListErr => write!(f, "unable to retrieve process list"),
            Self::ForkErr(bin) => write!(f, "unable to fork child process `{}`", bin),
            Self::SpawnErr(bin) => write!(f, "unable to spawn process `{}`", bin),
            Self::SignalErr(bin) => write!(f, "unable to send signal to process `{}`", bin),
            Self::GpioErr => write!(f, "unable to access GPIO"),
            Self::PinErr(err) => write!(f, "unable to access GPIO pin `{}`", err),
            Self::InterruptErr => write!(f, "unable to set interrupt on specified pin"),
            Self::ThreadJoinErr(err) => write!(f, "unable to join thread `{}`", err),
            Self::PollInterruptErr => write!(f, "unable to poll interrupt"),
        }
    }
}
