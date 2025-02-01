use std::process::{Command, Child, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    let output = Command::new("echo")
        .arg("Hello from subprocess!")
        .output()
        .expect("Failed to execute command");
    println!("Simple command output: {}", String::from_utf8_lossy(&output.stdout));

    let process = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ls command");
    
    let output = process.wait_with_output().expect("Failed to wait on child");
    println!("Directory listing: {}", String::from_utf8_lossy(&output.stdout));

    let mut background_process = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to start background process");

    println!("Background process ID: {}", background_process.id());
    
    match background_process.try_wait() {
        Ok(Some(status)) => println!("Process finished with status: {}", status),
        Ok(None) => println!("Process still running"),
        Err(e) => println!("Error attempting to wait: {}", e),
    }

    let process = Command::new("echo")
        .arg("Hello")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo command");

    let output = Command::new("tr")
        .arg("a-z")
        .arg("A-Z")
        .stdin(Stdio::from(process.stdout.unwrap()))
        .output()
        .expect("Failed to execute tr command");

    println!("Chained command output: {}", String::from_utf8_lossy(&output.stdout));

    let mut long_process = Command::new("sleep")
        .arg("10")
        .spawn()
        .expect("Failed to start long process");

    thread::sleep(Duration::from_secs(2));
    println!("Killing process...");
    long_process.kill().expect("Failed to kill process");

    println!("Current process ID: {}", std::process::id());
}