extern crate inotify;

use inotify::{Inotify, WatchMask};
use std::env;
use std::io;
use crate::shell_commands::run_command;

pub struct Watcher {
    inotify: Inotify,
}

impl Watcher {
    pub fn new() -> io::Result<Watcher> {
        let inotify = Inotify::init()?;
        let current_dir = env::current_dir()?;
        println!("Watching directory: {:?}", current_dir);
        inotify.watches().add(
            current_dir,
            WatchMask::CLOSE_WRITE | WatchMask::CREATE | WatchMask::DELETE,
        )?;
        Ok(Watcher { inotify })
    }

    pub fn handle_events(&mut self, _container_name: &str) -> io::Result<()> {
        let mut buffer = [0u8; 4096];
        println!("Waiting for inotify events...");
        let events = self.inotify.read_events_blocking(&mut buffer)?;
        println!("Events received");

        for event in events {
            println!("File change detected: {:?}", event);

            // Build and bring up Docker containers
            println!("Rebuilding and restarting containers...");
            if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "build", "--no-cache"]) {
                eprintln!("Error during build: {}", e);
                continue;
            }

            if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "up", "-d"]) {
                eprintln!("Error during up: {}", e);
                continue;
            }

            println!("Containers have been restarted.");
        }
        Ok(())
    }
}

