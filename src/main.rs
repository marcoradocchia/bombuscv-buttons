use bombuscv_buttons::{fork, pgrep, signal, ErrorKind, SIGINT, SIGUSR1};
use rppal::gpio::{Gpio, Level, Trigger};

// const BIN_NAME: &'static str = "bombuscv";

/// Run program.
fn run() -> Result<(), ErrorKind> {
    let gpio = Gpio::new().map_err(|_| ErrorKind::GpioErr)?;

    // Register interrupt for button presses on Pin 17, enabling internal pulldown resistor.
    gpio.get(17)
        .map_err(|err| ErrorKind::PinErr(err.to_string()))?
        .into_input_pulldown();
        // .set_async_interrupt(Trigger::RisingEdge, |level| {
        //     if level == Level::High {
        //         // signal("datalogger").unwrap();
        //         println!("button pressed");
        //     }
        // })
        // .map_err(|_| ErrorKind::InterruptErr)?;

    // Register interrupt for button presses on Pin 27, enabling internal pulldown resistor.

    // Spawn bombuscv process only if not yet running.
    // if let None = pgrep(BIN_NAME)? {
    //     fork(BIN_NAME)?;
    // } else {
    //     eprintln!("warning: `{}` already running", BIN_NAME);
    // }

    Ok(())
}

/// Run program and catch errors.
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}
