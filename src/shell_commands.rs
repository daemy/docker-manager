
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::thread;

pub fn run_command(command: &str, args: &[&str]) -> io::Result<()> {
    println!("Running command: {} {:?}", command, args);
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let stdout_reader = io::BufReader::new(stdout);
        for line in stdout_reader.lines() {
            println!("{}", line?);
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let stderr_reader = io::BufReader::new(stderr);
        for line in stderr_reader.lines() {
            eprintln!("{}", line?);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Command failed"));
    }

    Ok(())
}

pub fn run_command_async(command: &str, args: &[&str]) -> io::Result<()> {
    println!("Running command asynchronously: {} {:?}", command, args);
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    thread::spawn(move || {
        let stdout_reader = io::BufReader::new(stdout);
        for line in stdout_reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    thread::spawn(move || {
        let stderr_reader = io::BufReader::new(stderr);
        for line in stderr_reader.lines() {
            match line {
                Ok(line) => eprintln!("{}", line),
                Err(e) => eprintln!("Error reading stderr: {}", e),
            }
        }
    });

    Ok(())
}
