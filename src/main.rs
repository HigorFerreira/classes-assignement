use std::process::Command;
use std::io::{self, Write};

fn main() {
    let python_check = Command::new("python")
        .arg("--version")
        .output();

    println!("Comming to match python check");
    match python_check {
        Ok(output) => {
            let out = match String::from_utf8(output.stdout) {
                Ok(str) => Some(str),
                Err(_) => None
            };
            println!("{}", out.unwrap_or_else(|| "<invalid UTF-8>".into()));
        },
        Err(_) => {
            show_error_message("Python not foung", "Python not installed");
        }
    }
}

fn show_error_message(title: &str, message: &str) {
    unsafe {
        println!("{title}");
        println!("{message}");
    }
}
