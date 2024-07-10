extern crate notify;

use std::collections::HashSet;
use std::env;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::sync::mpsc::{Sender};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use crate::shell_commands::run_command;
use notify::{Config, Watcher, RecommendedWatcher, RecursiveMode, Result, Event};

pub struct FileWatcher {
    watched_files: HashSet<PathBuf>,
    pub logs: Arc<RwLock<Vec<String>>>,
    watcher: RecommendedWatcher,
    debounce_set: Arc<Mutex<HashSet<String>>>,
    debounce_duration: Duration,
}

impl FileWatcher {
    pub fn new(tx: Sender<Result<Event>>) -> io::Result<FileWatcher> {
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
        let logs = Arc::new(RwLock::new(logs));
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        watcher.watch(&current_dir, RecursiveMode::Recursive).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // Initialize FileWatcher
        let file_watcher = FileWatcher {
            watched_files,
            logs,
            watcher,
            debounce_set: Arc::new(Mutex::new(HashSet::new())),
            debounce_duration: Duration::from_secs(2),
        };

        Ok(file_watcher)
    }

    fn should_watch(&self, filepath: &PathBuf) -> bool {
        self.watched_files.contains(filepath)
    }

    pub fn handle_event(&mut self, res: Result<Event>) -> io::Result<()> {

        // Here wait for event changes
        match res {
            Ok(event) => {
                //self.logs.push(format!("Change: {:?}", event));
                for path in event.paths {
                    if path.is_file() {
                        let path_str = path.to_string_lossy().to_string();
                        let debounce_set = Arc::clone(&self.debounce_set);
                        let debounce_duration = self.debounce_duration;

                        let mut debounce_guard = debounce_set.lock().unwrap();
                        if debounce_guard.contains(&path_str) {
                            continue;
                        }

                        debounce_guard.insert(path_str.clone());
                        let logs_clone: Arc<RwLock<Vec<String>>> = Arc::clone(&self.logs);
                        {
                            let mut logs = logs_clone.write().unwrap();
                            logs.push(format!("File change detected: {:?}", path));
                            logs.push("Rebuilding and restarting containers...".to_string());
                        }
                        // Execute your command here, e.g., run_command("docker-compose restart")
                        //
                        //
                        thread::spawn(move || {
                            let mut logs = logs_clone.write().unwrap();
                            if let Err(e) = run_command("docker-compose", &["-f", "docker-compose-dev.yml", "build", "--no-cache"]) {
                                logs.push(format!("Error during build: {}", e));
                            } else {
                                logs.push("Containers have been restarted.".to_string());
                            }
                        });
                        //thread::sleep(debounce_duration);
                        //let mut debounce_guard = debounce_set.lock().unwrap();
                        //debounce_guard.remove(&path_str);
                    }
                }
            },
            Err(e) => {
                let mut logs = self.logs.write().unwrap();
                logs.push(format!("Watch error: {:?}", e));
            },
        }

        Ok(())
    }


    pub fn get_watched_files(&self) -> &HashSet<PathBuf> {
        &self.watched_files
    }

    pub fn get_logs(&self) -> &Vec<String> {
        &self.logs.read().unwrap()
    }
}
