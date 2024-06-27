extern crate notify;

use std::collections::HashSet;
use std::env;
//use std::fs;
use std::io;
use std::thread;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc::{channel, Receiver}};
use crate::shell_commands::run_command;
use walkdir::WalkDir;
use notify::{Config, Watcher, RecommendedWatcher, RecursiveMode, Result, Event, Error};

pub struct FileWatcher {
    watched_files: HashSet<PathBuf>,
    logs: Vec<String>,
    rx: Receiver<Result<Event>>,
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    pub fn new() -> io::Result<FileWatcher> {
        let current_dir = env::current_dir()?;
        let mut logs = Vec::new();
        // Collect a list of files in the watched directory and all subdirectories
        let mut watched_files = HashSet::new();
        for entry in WalkDir::new(&current_dir) {
            let entry = entry?;
            let path = entry.path().to_path_buf();
            if path.is_file() {
                watched_files.insert(path);
            }
        }

        logs.push("Initialized watcher...".to_string());
        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(tx, Config::default()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Initialize FileWatcher
        let file_watcher = FileWatcher {
            watched_files,
            logs,
            rx,
            watcher,
        };

        Ok(file_watcher)
    }

    fn should_watch(&self, filepath: &PathBuf) -> bool {
        self.watched_files.contains(filepath)
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        self.logs.push("Waiting for changes...".to_string());

        // Here wait for event changes
        while let Ok(res) = self.rx.recv() {
            match res {
                Ok(event) => {
                    self.logs.push(format!("Change: {:?}", event));
                    for path in event.paths {
                        if path.is_file() {
                            self.logs.push(format!("File change detected: {:?}", path));
                            self.logs.push("Rebuilding and restarting containers...".to_string());
                            // Execute your command here, e.g., run_command("docker-compose restart")
                            self.logs.push("Containers have been restarted.".to_string());
                        }
                    }
                },
                Err(e) => {
                    self.logs.push(format!("Watch error: {:?}", e));
                },
            }
        }

        Ok(())
    }

    pub fn get_watched_files(&self) -> &HashSet<PathBuf> {
        &self.watched_files
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs
    }
}
