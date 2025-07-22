use std::process::{Command, Stdio};
use std::io::{self, Write};
use regex::Regex;

// use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION};

fn main() {
    let python_check = Command::new("python")
        .arg("--version")
        .output();

    match python_check {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                if checkVersion(&version) {
                    match start_python_http_server(Some(8000)) {
                        Ok(_) => println!("Server stopped"),
                        Err(e) => eprintln!("Error:{}", e),
                    }
                }
                else {

                }
            }
            else {

            }
        },
        Err(_) => {
            // show_error_message("Python não encontrado", "Para rodar o seguinte programa, é necessário python 3.12+");
        }
    }
}

// fn show_error_message(title: &str, message: &str) {
//     unsafe {
//         MessageBoxA(
//             std::ptr::null_mut(),
//             message.as_ptr() as *const i8,
//             title.as_ptr() as *const i8,
//             MB_OK | MB_ICONINFORMATION
//         );
//     }
// }

fn start_python_http_server(port: Option<u16>) -> io::Result<()> {
    let mut cmd = Command::new("python");
    cmd.arg("-m");
    cmd.arg("http.server");

    if let Some(p) = port {
        cmd.arg(p.to_string());
    }

    let mut child = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    println!("Started Python HTTP server at http://localhost:{}", port.unwrap_or(8000));
    println!("Press Ctrl+C to stop the server...");

    child.wait()?;
    Ok(())
}

fn checkVersion(str: &str) -> bool {
    let pattern = r"^Python\s*3\.12\.\d+$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(str.trim())
}
