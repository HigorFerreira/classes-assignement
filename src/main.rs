use std::process::Command;
use std::io::{self, Write};

fn main() {
    let python_check = Command::new("python")
        .arg("--version")
        .output();

    println!("Comming to match python check");
    match python_check {
        Ok(output) => {
            let out = output.stdout;
            let out = String::from_utf8(out);

            match out {
                Ok(message) => {
                    println!("{message}");
                },
                Err(_) => {
                    print!("Error while converting rust");
                }
            }
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
