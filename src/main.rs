extern crate inotify;

use std::env;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use inotify::{
    WatchMask,
    Inotify,
};


fn main() {

    // Get the container name from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: docker-manager <container-name>");
        std::process::exit(1);
    }

    let container_name = &args[1];

    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    let current_dir = env::current_dir()
        .expect("Failed to determine current directory");

    inotify
        .watches()
        .add(
            current_dir,
            WatchMask::CLOSE_WRITE | WatchMask::CREATE | WatchMask::DELETE,
        )
        .expect("Failed to add inotify watch");

    println!("Watching current directory for activity...");

    let mut buffer = [0u8; 4096];
    loop {
        // Run docker logs -f {container-name}
        if let Err(e) = run_command("docker", &["logs", container_name, "-f"]) {
            eprintln!("Error during docker logs: {}", e);
        }
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            println!("File change detected: {:?}", event);

            // Build and bring up Docker containers
            println!("Rebuilding and restarting containers...");
            // Run docker-compose build
            if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "build", "--no-cache"]) {
                eprintln!("Error during build: {}", e);
                continue;
            }

            // Run docker-compose up
            if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "up", "-d"]) {
                eprintln!("Error during up: {}", e);
                continue;
            }

            println!("Containers have been restarted.");

            // Run docker logs -f {container-name}
            if let Err(e) = run_command("docker", &["logs", container_name, "-f"]) {
                eprintln!("Error during docker logs: {}", e);
            }
        }
    }
}

fn run_command(command: &str, args: &[&str]) -> io::Result<()> {
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
