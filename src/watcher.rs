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
    logs: Arc<Mutex<Vec<String>>>,
    rx: Receiver<Result<Event>>,
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    pub fn new() -> io::Result<Arc<Mutex<FileWatcher>>> {
        let current_dir = env::current_dir()?;
        let logs = Arc::new(Mutex::new(vec![
            "Started docker-manager successfully.".to_string(),
            format!("Watching directory: {:?}", current_dir),
        ]));

        // Collect a list of files in the watched directory and all subdirectories
        let mut watched_files = HashSet::new();
        for entry in WalkDir::new(&current_dir) {
            let entry = entry?;
            let path = entry.path().to_path_buf();
            if path.is_file() {
                watched_files.insert(path);
            }
        }

        let (tx, rx) = channel();
        let watcher = RecommendedWatcher::new(tx, Config::default()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Initialize FileWatcher
        let file_watcher = Arc::new(Mutex::new(FileWatcher {
            watched_files,
            logs: Arc::clone(&logs),
            rx,
            watcher,
        }));

        // Start watching the directory
        //watcher.watch(&current_dir, RecursiveMode::Recursive).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Clone file_watcher and logs for running file event watcher on separate thread
        let file_watcher_clone = Arc::clone(&file_watcher);
        thread::spawn(move || {
            let mut file_watcher = file_watcher_clone.lock().unwrap();
            file_watcher.handle_events().unwrap();
        });

        Ok(file_watcher)
    }

    fn should_watch(&self, filepath: &PathBuf) -> bool {
        self.watched_files.contains(filepath)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        {
            let mut logs = self.logs.lock().unwrap();
            logs.push("Waiting for changes...".to_string());
        }

        // Here wait for event changes
        while let Ok(res) = self.rx.recv() {
            match res {
                Ok(event) => {
                    let mut logs = self.logs.lock().unwrap();
                    logs.push(format!("Change: {:?}", event));
                    for path in event.paths {
                        if path.is_file() {
                            logs.push(format!("File change detected: {:?}", path));
                            logs.push("Rebuilding and restarting containers...".to_string());
                            // Execute your command here, e.g., run_command("docker-compose restart")
                            logs.push("Containers have been restarted.".to_string());
                        }
                    }
                },
                Err(e) => {
                    let mut logs = self.logs.lock().unwrap();
                    logs.push(format!("Watch error: {:?}", e));
                },
            }
        }

        Ok(())
    }

    pub fn get_watched_files(&self) -> &HashSet<PathBuf> {
        &self.watched_files
    }

    pub fn get_logs(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.logs)
    }
}
