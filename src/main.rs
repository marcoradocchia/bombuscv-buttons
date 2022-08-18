use bombuscv_buttons::{fork, input_pin, pgrep, signal, ErrorKind, Signal};
use rppal::gpio::{Gpio, Trigger};
use std::{sync::Mutex, thread};

const DATALOGGER_BIN: &str = "datalogger";
const STREAM_BIN: &str = "rstp-simple-server";
const BOMBUSCV_BIN: &str = "bombuscv";

/// Run program and return errors.
fn run() -> Result<(), ErrorKind> {
    let gpio = Gpio::new().map_err(|_| ErrorKind::GpioErr)?;

    // Set Pin for datalogger CSV printing beahaviour on Pin 27.
    let mut datalogger_button = input_pin(&gpio, 27, rppal::gpio::PullUpDown::PullUp)?;
    datalogger_button
        .set_interrupt(Trigger::RisingEdge)
        .map_err(|_| ErrorKind::InterruptErr)?;
    // Register interrupt for button presses on Pin 27.
    datalogger_button
        .set_interrupt(Trigger::RisingEdge)
        .map_err(|_| ErrorKind::InterruptErr)?;
    // Status of the CSV printing behaviour:
    //  false -> datalogger not printing CSV
    //  true -> datalogger printing CSV
    let datalogger_csv_status = Mutex::new(false);

    // Set Pin for datalogger CSV on Pin 22.
    let mut stream_button = input_pin(&gpio, 22, rppal::gpio::PullUpDown::PullUp)?;
    // Register interrupt for button presses on Pin 22.
    stream_button
        .set_interrupt(Trigger::RisingEdge)
        .map_err(|_| ErrorKind::InterruptErr)?;

    // Set Pin for bombuscv start/stop on Pin 17.
    let mut bombuscv_button = input_pin(&gpio, 17, rppal::gpio::PullUpDown::PullUp)?;
    // Register interrupt for button presses on Pin 27.
    bombuscv_button
        .set_interrupt(Trigger::RisingEdge)
        .map_err(|_| ErrorKind::InterruptErr)?;

    // Spawn datalogger button handler thread.
    let datalogger_button_thread = thread::spawn(move || -> Result<(), ErrorKind> {
        loop {
            // Poll the interrupt indefinitely for button presses (blocking).
            datalogger_button
                .poll_interrupt(true, None)
                .map_err(|_| ErrorKind::PollInterruptErr)?;

            if let Ok(mut status) = datalogger_csv_status.lock() {
                *status = !*status;
                println!("datalogger status: {}", status); // TODO: this is for testing purposes, delete it.

                // Signal datalogger process with SIGUSR1 to swap CSV printing behaviour.
                signal(DATALOGGER_BIN, Signal::SIGUSR1)?;
            } else {
                continue;
            };
        }
    });

    // Spawn stream button handler thread.
    let stream_button_thread = thread::spawn(move || -> Result<(), ErrorKind> {
        loop {
            stream_button
                .poll_interrupt(true, None)
                .map_err(|_| ErrorKind::PollInterruptErr)?;

            // Bomubscv already running, ignore button press.
            if pgrep(BOMBUSCV_BIN)?.is_some() {
                continue;
            }

            // Fork new process if not running, else stop currently running process.
            // TODO: maybe add long press for stopping, in order to prevent accidental presses.
            if pgrep(STREAM_BIN)?.is_none() {
                fork(STREAM_BIN)?;
            } else {
                signal(STREAM_BIN, Signal::SIGINT)?;
            }
        }
    });

    // Spawn bombuscv button handler thread.
    let bombuscv_button_thread = thread::spawn(move || -> Result<(), ErrorKind> {
        loop {
            bombuscv_button
                .poll_interrupt(true, None)
                .map_err(|_| ErrorKind::PollInterruptErr)?;

            // Stream already running, ignore button press.
            if pgrep(STREAM_BIN)?.is_some() {
                continue;
            }

            // Fork new process if not running, else stop currently running process.
            // TODO: maybe add long press for stopping, in order to prevent accidental presses.
            if pgrep(BOMBUSCV_BIN)?.is_none() {
                fork(BOMBUSCV_BIN)?;
            } else {
                signal(BOMBUSCV_BIN, Signal::SIGINT)?;
            }
        }
    });

    // Join all threads.
    datalogger_button_thread
        .join()
        .map_err(|_| ErrorKind::ThreadJoinErr("datalogger button handler".to_string()))??;
    stream_button_thread
        .join()
        .map_err(|_| ErrorKind::ThreadJoinErr("stream button handler".to_string()))??;
    bombuscv_button_thread
        .join()
        .map_err(|_| ErrorKind::ThreadJoinErr("bombuscv button handler".to_string()))??;

    Ok(())
}

/// Run program and catch errors.
fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
    }
}
