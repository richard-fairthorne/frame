use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub kind: ChangeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    Created,
    Modified,
    Deleted,
}

pub struct FileWatcher {
    watched_paths: Vec<PathBuf>,
    last_seen: HashMap<PathBuf, SystemTime>,
    poll_interval: Duration,
    ignore_patterns: Vec<String>,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            watched_paths: Vec::new(),
            last_seen: HashMap::new(),
            poll_interval: Duration::from_millis(500),
            ignore_patterns: vec![
                "target".into(),
                ".git".into(),
                ".DS_Store".into(),
            ],
        }
    }

    pub fn watch(mut self, path: impl Into<PathBuf>) -> Self {
        self.watched_paths.push(path.into());
        self
    }

    pub fn poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    pub fn ignore(mut self, pattern: &str) -> Self {
        self.ignore_patterns.push(pattern.to_string());
        self
    }

    pub fn scan_initial(&mut self) {
        let watched = self.watched_paths.clone();
        let ignore = self.ignore_patterns.clone();
        for path in &watched {
            scan_dir_recursive(path, &ignore, &mut |file_path, modified| {
                self.last_seen.insert(file_path.clone(), modified);
            });
        }
    }

    pub fn poll_changes(&mut self) -> Vec<FileChange> {
        let mut changes = Vec::new();
        let mut current_files = HashSet::new();
        let watched = self.watched_paths.clone();
        let ignore_patterns = self.ignore_patterns.clone();
        let last_seen_snapshot: HashMap<PathBuf, SystemTime> = self.last_seen.clone();

        for path in &watched {
            scan_dir_recursive(
                path,
                &ignore_patterns,
                &mut |file_path: &PathBuf, modified: SystemTime| {
                    current_files.insert(file_path.clone());
                    if should_ignore(file_path, &ignore_patterns) {
                        return;
                    }
                    match last_seen_snapshot.get(file_path) {
                        None => {
                            changes.push(FileChange {
                                path: file_path.clone(),
                                kind: ChangeKind::Created,
                            });
                        }
                        Some(&last) if last != modified => {
                            changes.push(FileChange {
                                path: file_path.clone(),
                                kind: ChangeKind::Modified,
                            });
                        }
                        _ => {}
                    }
                },
            );
        }

        let tracked: Vec<PathBuf> = self.last_seen.keys().cloned().collect();
        for path in tracked {
            if should_ignore(&path, &ignore_patterns) {
                continue;
            }
            if !current_files.contains(&path)
                && watched.iter().any(|wp| path.starts_with(wp))
            {
                changes.push(FileChange {
                    path: path.clone(),
                    kind: ChangeKind::Deleted,
                });
                self.last_seen.remove(&path);
            }
        }

        for change in &changes {
            match change.kind {
                ChangeKind::Created | ChangeKind::Modified => {
                    if let Ok(metadata) = std::fs::metadata(&change.path) {
                        if let Ok(modified) = metadata.modified() {
                            self.last_seen.insert(change.path.clone(), modified);
                        }
                    }
                }
                ChangeKind::Deleted => {}
            }
        }

        changes
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

fn scan_dir_recursive(
    dir: &Path,
    ignore_patterns: &[String],
    callback: &mut impl FnMut(&PathBuf, SystemTime),
) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if should_ignore(&path, ignore_patterns) {
                continue;
            }
            if path.is_dir() {
                scan_dir_recursive(&path, ignore_patterns, callback);
            } else if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    callback(&path, modified);
                }
            }
        }
    }
}

fn should_ignore(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns
        .iter()
        .any(|pattern| path_str.contains(pattern))
}

#[derive(Debug, Clone)]
pub struct DevServerConfig {
    pub host: String,
    pub port: u16,
    pub watch_paths: Vec<PathBuf>,
    pub poll_interval_ms: u64,
}

impl DevServerConfig {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 9191,
            watch_paths: vec![PathBuf::from("src")],
            poll_interval_ms: 500,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn watch(mut self, path: impl Into<PathBuf>) -> Self {
        self.watch_paths.push(path.into());
        self
    }
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DevServer {
    config: DevServerConfig,
    watcher: FileWatcher,
    build_count: u32,
    last_build: Option<Instant>,
}

impl DevServer {
    pub fn new(config: DevServerConfig) -> Self {
        let mut watcher = FileWatcher::new();
        for path in &config.watch_paths {
            watcher = watcher.watch(path);
        }
        watcher = watcher.poll_interval(Duration::from_millis(config.poll_interval_ms));
        Self {
            config,
            watcher,
            build_count: 0,
            last_build: None,
        }
    }

    pub fn start(&mut self) {
        println!(
            "Frame dev server starting on {}:{}",
            self.config.host, self.config.port
        );
        println!("Watching: {:?}", self.config.watch_paths);
        self.watcher.scan_initial();
        println!("Initial scan complete, watching for changes...");
    }

    pub fn poll(&mut self) -> DevServerEvent {
        let changes = self.watcher.poll_changes();
        if changes.is_empty() {
            DevServerEvent::Idle
        } else {
            self.build_count += 1;
            self.last_build = Some(Instant::now());
            DevServerEvent::ChangesDetected {
                files: changes,
                build_number: self.build_count,
            }
        }
    }

    pub fn build_count(&self) -> u32 {
        self.build_count
    }

    pub fn last_build(&self) -> Option<Instant> {
        self.last_build
    }

    pub fn run_once(&mut self) -> DevServerEvent {
        self.poll()
    }

    pub fn run(&mut self) {
        self.start();
        loop {
            match self.poll() {
                DevServerEvent::Idle => {
                    std::thread::sleep(Duration::from_millis(self.config.poll_interval_ms));
                }
                DevServerEvent::ChangesDetected {
                    ref files,
                    build_number,
                } => {
                    println!(
                        "[build {}] {} file(s) changed:",
                        build_number,
                        files.len()
                    );
                    for change in files {
                        println!("  {:?}: {}", change.kind, change.path.display());
                    }
                    println!("Triggering rebuild...");
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum DevServerEvent {
    Idle,
    ChangesDetected {
        files: Vec<FileChange>,
        build_number: u32,
    },
}
