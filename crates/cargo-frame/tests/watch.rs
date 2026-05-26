use cargo_frame::watch::{ChangeKind, FileWatcher};
use std::fs;
use std::path::PathBuf;

fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("frame-watch-test-{}", name));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn file_watcher_detect_new_file() {
    let dir = temp_dir("new_file");
    let mut watcher = FileWatcher::new().watch(&dir);
    watcher.scan_initial();

    fs::write(dir.join("test.txt"), "hello").unwrap();

    let changes = watcher.poll_changes();
    assert!(changes
        .iter()
        .any(|c| c.kind == ChangeKind::Created && c.path.ends_with("test.txt")));

    cleanup(&dir);
}

#[test]
fn file_watcher_detect_modification() {
    let dir = temp_dir("modification");
    let file = dir.join("test.txt");
    fs::write(&file, "v1").unwrap();

    let mut watcher = FileWatcher::new().watch(&dir);
    watcher.scan_initial();

    std::thread::sleep(std::time::Duration::from_millis(10));
    fs::write(&file, "v2").unwrap();

    let changes = watcher.poll_changes();
    assert!(changes.iter().any(|c| c.kind == ChangeKind::Modified));

    cleanup(&dir);
}

#[test]
fn file_watcher_ignores_target() {
    let dir = temp_dir("ignores_target");
    let target = dir.join("target");
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join("build.out"), "data").unwrap();

    let mut watcher = FileWatcher::new().watch(&dir);
    watcher.scan_initial();

    let changes = watcher.poll_changes();
    assert!(!changes
        .iter()
        .any(|c| c.path.to_string_lossy().contains("target")));

    cleanup(&dir);
}

#[test]
fn file_watcher_detect_deletion() {
    let dir = temp_dir("deletion");
    let file = dir.join("temp.txt");
    fs::write(&file, "data").unwrap();

    let mut watcher = FileWatcher::new().watch(&dir);
    watcher.scan_initial();

    fs::remove_file(&file).unwrap();

    let changes = watcher.poll_changes();
    assert!(changes.iter().any(|c| c.kind == ChangeKind::Deleted));

    cleanup(&dir);
}
