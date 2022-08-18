mod error;

pub use error::ErrorKind;
pub use nix::sys::signal::Signal;
pub use nix::{
    sys::{
        signal::{
            kill,
            Signal::{SIGINT, SIGUSR1},
        },
        wait::waitpid,
    },
    unistd::{fork as nix_fork, ForkResult, Pid},
};

use procfs::process::all_processes;
use rppal::gpio::{Gpio, InputPin, PullUpDown};
use std::process::Command;

/// Spawn & fork child process.
pub fn fork(bin: &str) -> Result<(), ErrorKind> {
    match unsafe { nix_fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("Forked `{}` child process with pid: {}", bin, child);
            // Wait for the fork in order to prevent it from becoming a zombie!
            waitpid(Some(child), None).unwrap();
        }

        Ok(ForkResult::Child) => {
            if Command::new(bin).spawn().is_err() {
                return Err(ErrorKind::SpawnErr(bin.to_string()));
            }
        }

        Err(_) => return Err(ErrorKind::ForkErr(bin.to_string())),
    };

    Ok(())
}

/// Check for running process returning bool whether the process is running or not.
pub fn pgrep(bin: &str) -> Result<Option<Pid>, ErrorKind> {
    if let Ok(proc_list) = all_processes() {
        for proc in proc_list {
            let proc = proc.unwrap();
            if let Ok(exe) = proc.exe() {
                if exe.file_stem().unwrap() == bin {
                    return Ok(Some(Pid::from_raw(proc.pid)));
                }
            }
        }
    } else {
        return Err(ErrorKind::ProcListErr);
    }

    Ok(None)
}

/// Signal process sending SIGUSR1 signal.
pub fn signal(bin: &str, signal: Signal) -> Result<(), ErrorKind> {
    if let Some(pid) = pgrep(bin)? {
        kill(pid, signal).map_err(|_| ErrorKind::SignalErr(bin.to_string()))?;
    }

    Ok(())
}

/// Setup Pin as InputPin with internal pullup resistor enabled.
pub fn input_pin(gpio: &Gpio, pin: u8, pull_up_down: PullUpDown) -> Result<InputPin, ErrorKind> {
    let pin = gpio
        .get(pin)
        .map_err(|err| ErrorKind::PinErr(err.to_string()))?;

    Ok(match pull_up_down {
        PullUpDown::Off => pin.into_input(),
        PullUpDown::PullDown => pin.into_input_pulldown(),
        PullUpDown::PullUp => pin.into_input_pullup(),
    })
}
