mod shell_commands;
mod watcher;
mod ui;
mod utils;

use std::env;
use shell_commands::run_command_async;
use watcher::Watcher;

fn main() {
    // Get the container name from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: docker-manager <container-name>");
        std::process::exit(1);
    }

    let container_name = &args[1];

    // Initialize the watcher
    let mut watcher = Watcher::new().expect("Failed to initialize inotify");

    println!("Watching current directory for activity...");

    // Event loop
    loop {
        // Run docker logs -f {container-name}
        if let Err(e) = run_command_async("docker", &["logs", container_name, "-f"]) {
            eprintln!("Error during docker logs: {}", e);
        }

        // Handle file changes
        if let Err(e) = watcher.handle_events(&container_name) {
            eprintln!("Error handling events: {}", e);
        }
    }
}
