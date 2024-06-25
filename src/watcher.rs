extern crate inotify;

use std::collections::HashSet;
use inotify::{Inotify, WatchMask, EventMask};
use std::env;
//use std::fs;
use std::io;
use std::thread;
use std::path::PathBuf;
use std::time::Duration;
use crate::shell_commands::run_command;
use walkdir::WalkDir;

pub struct Watcher {
    inotify: Inotify,
    watched_files: HashSet<PathBuf>,
    logs: Vec<String>,
}

impl Watcher {
    pub fn new() -> io::Result<Watcher> {
        let inotify = Inotify::init()?;
        let current_dir = env::current_dir()?;
        let mut logs = Vec::new();
        logs.push(format!("Watching directory: {:?}", current_dir));

        // Collect a list of files in the watched directory and all subdirectories
        let mut watched_files = HashSet::new();
        for entry in WalkDir::new(&current_dir) {
            let entry = entry?;
            let path = entry.path().to_path_buf();
            if path.is_file() {
                watched_files.insert(path);
            }
        }
        inotify.watches().add(
            &current_dir,
            WatchMask::CLOSE_WRITE | WatchMask::CREATE | WatchMask::DELETE,
        )?;

        Ok(Watcher { inotify, watched_files, logs })
    }

    pub fn get_watched_files(&self) -> &HashSet<PathBuf> {
        &self.watched_files
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }

    fn should_watch(&self, filepath: &PathBuf) -> bool {
        self.watched_files.contains(filepath)
    }

    pub fn handle_events(&mut self, _container_name: &str) -> io::Result<()> {
        let mut buffer = [0u8; 4096];
        self.logs.push("Waiting for changes...".to_string());
        let events = match self.inotify.read_events_blocking(&mut buffer) {
            Ok(events) => events,
            Err(err) => {
                // Handle the error (e.g., log it, return early)
                self.logs.push(format!("Error reading events: {}", err));
                return Err(err);
            },
        };

        for event in events {
            if let Some(name) = event.name {
                let filepath = env::current_dir()?.join(name);

                if !self.should_watch(&filepath) {
                    continue;
                } 
                if event.mask.contains(EventMask::CLOSE_WRITE) {
                    self.logs.push(format!("File change detected: {:?}", event));

                    // Debounce mechanism: introduce a short delay to avoid multiple rapid restarts
                    thread::sleep(Duration::from_secs(1));

                    // Build and bring up Docker containers
                    self.logs.push("Rebuilding and restarting containers...".to_string());
                    //if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "build", "--no-cache"]) {
                    //    self.logs.push(format!("Error during building containers: {}", e));
                    //    continue;
                    //
                    //if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "up", "-d"]) {
                    //    self.logs.push(format!("Error during starting containers: {}", e));
                    //    continue;
                    //}
                    self.logs.push("Containers have been restarted.".to_string());
                }
            }
        }
        Ok(())
    }
}

