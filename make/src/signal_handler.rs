use std::{fs::remove_file, process};

use libc::{signal, SIGHUP, SIGINT, SIGQUIT, SIGTERM};

use crate::rule::INTERRUPT_FLAG;


/// Handles incoming signals by setting the interrupt flag and exiting the process.
///
/// # Safety
///
/// This function is marked as `extern "C"` because it is used as a signal handler.
/// Signal handlers should be as minimal as possible.
pub fn handle_signals(signal_code: libc::c_int) {
    let interrupt_flag = INTERRUPT_FLAG.lock().unwrap();
    if let Some((target, precious)) = interrupt_flag.as_ref() {
        eprintln!("make: Interrupt");
        // Assuming 'precious' is false; adjust as needed
        if !precious {
            eprintln!("make: Deleting file '{}'", target);
            if let Err(err) = remove_file(target) {
                eprintln!("Error deleting file: {}", err);
            }
        }
    }

    process::exit(128 + signal_code);
}

/// Registers signal handlers for SIGINT, SIGQUIT, SIGTERM, and SIGHUP.
pub fn register_signals() {
    unsafe {
        signal(SIGINT, handle_signals as usize);
        signal(SIGQUIT, handle_signals as usize);
        signal(SIGTERM, handle_signals as usize);
        signal(SIGHUP, handle_signals as usize);
    }
}
