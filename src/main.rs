use bombuscv_buttons::{fork, kill, pgrep, ErrorKind, SIGINT, SIGUSR1};

const BIN_NAME: &'static str = "bombuscv";

/// Run program.
fn run() -> Result<(), ErrorKind> {
    // Toggle datalogger CSV.
    // if let Some(pid) = pgrep("datalogger")? {
    //     kill(pid, SIGUSR1).unwrap();
    // }

    // Spawn bombuscv process only if not yet running.
    if let None = pgrep(BIN_NAME)? {
        fork(BIN_NAME)?;
    } else {
        eprintln!("warning: `{}` already running", BIN_NAME);
    }

    Ok(())
}

/// Run program and catch errors.
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}
