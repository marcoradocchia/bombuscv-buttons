mod error;

pub use error::ErrorKind;
use procfs::process::all_processes;

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
                return Err(ErrorKind::SpawnFail(bin.to_string()));
            }
        }

        Err(_) => return Err(ErrorKind::ForkFail(bin.to_string())),
    };

    Ok(())
}

/// Check for running process returning bool whether the process is running or not.
pub fn pgrep(name: &str) -> Result<Option<Pid>, ErrorKind> {
    if let Ok(proc_list) = all_processes() {
        for proc in proc_list {
            let proc = proc.unwrap();
            if let Ok(bin) = proc.exe() {
                if bin.file_stem().unwrap() == name {
                    return Ok(Some(Pid::from_raw(proc.pid)));
                }
            }
        }
    } else {
        return Err(ErrorKind::ProcListErr);
    }

    Ok(None)
}
