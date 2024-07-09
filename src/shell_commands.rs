use std::io::{self, BufRead};
use std::process::{Command, Stdio};
//use std::thread;
use regex::Regex;
use crate::watcher::FileWatcher;
use crossbeam_channel::{unbounded, Sender};

pub fn run_command(command: &str, args: &[&str]) -> io::Result<()> {
    //watcher.logs.push(format!("Running command: {} {:?}", command, args));
    let mut output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    //if let Some(stdout) = output.stdout.take() {
    //    let stdout_reader = io::BufReader::new(stdout);
    //    for line in stdout_reader.lines() {
    //        match line {
    //            Ok(line) => {
    //                 //watcher.logs.push(format!("{}", line));
    //            }
    //            Err(e) => //watcher.logs.push(format!("Error reading stdout: {}", e)),
    //            ,
    //        }
    //    }
    //}

    //if let Some(stderr) = output.stderr.take() {
    //    let stderr_reader = io::BufReader::new(stderr);
    //    for line in stderr_reader.lines() {
    //        match line {
    //            Ok(line) => {
    //                //watcher.logs.push(format!("{}", line));
    //            }
    //            Err(e) => //watcher.logs.push(format!("Error reading stderr: {}", e)),
    //            ,
    //        }
    //    }
    //}

    let status = output.wait()?;
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Command failed"));
    } else {
        //watcher.logs.push(format!("Command: {} finished successfully.", command));
    }

    Ok(())
}

//pub fn run_command_async(command: &str, args: &[&str]) -> io::Result<()> {
//    println!("Running command asynchronously: {} {:?}", command, args);
//    let mut child = Command::new(command)
//        .args(args)
//        .stdout(Stdio::piped())
//        .stderr(Stdio::piped())
//        .spawn()?;
//
//    let stdout = child.stdout.take().expect("Failed to open stdout");
//    let stderr = child.stderr.take().expect("Failed to open stderr");
//
//    thread::spawn(move || {
//        let stdout_reader = io::BufReader::new(stdout);
//        for line in stdout_reader.lines() {
//            match line {
//                Ok(line) => println!("{}", line),
//                Err(e) => eprintln!("Error reading stdout: {}", e),
//            }
//        }
//    });
//
//    thread::spawn(move || {
//        let stderr_reader = io::BufReader::new(stderr);
//        for line in stderr_reader.lines() {
//            match line {
//                Ok(line) => eprintln!("{}", line),
//                Err(e) => eprintln!("Error reading stderr: {}", e),
//            }
//        }
//    });
//
//    Ok(())
//}
