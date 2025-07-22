use std::process::Command;
use std::io::{self, Write};
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION};

fn main() {
    let python_check = Command::new("python")
        .arg("--version")
        .output();

    match python_check {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("Python installed with version: {}", version);
            }
        },
        Err(_) => {
            show_error_message("Python not foung", "Python not installed");
        }
    }
}

fn show_error_message(title: &str, message: &str) {
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            message.as_ptr() as *const i8,
            title.as_ptr() as *const i8,
            MB_OK | MB_ICONINFORMATION
        );
    }
}
