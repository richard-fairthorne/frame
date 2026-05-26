use cargo_frame::export::{export_project, ExportConfig, ExportTarget, GradleExporter, XcodeExporter};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_export_dir_unique(name: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("frame-export-test-{}-{}", name, id));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn export_xcode_project() {
    let dir = temp_export_dir_unique("xcode");
    let config = ExportConfig::new("MyApp")
        .bundle_id("com.test.myapp")
        .output_dir(&dir);

    XcodeExporter::new(config).export().unwrap();

    let ios_dir = dir.join("ios").join("MyApp");
    assert!(ios_dir.join("Info.plist").exists());
    assert!(ios_dir.join("MyApp.entitlements").exists());
    assert!(ios_dir.join("MyApp-Bridging-Header.h").exists());
    assert!(ios_dir
        .join("Base.lproj")
        .join("LaunchScreen.storyboard")
        .exists());
    assert!(ios_dir
        .join("MyApp.xcodeproj")
        .join("project.pbxproj")
        .exists());

    let plist = fs::read_to_string(ios_dir.join("Info.plist")).unwrap();
    assert!(plist.contains("com.test.myapp"));
    assert!(plist.contains("MyApp"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn export_gradle_project() {
    let dir = temp_export_dir_unique("gradle");
    let config = ExportConfig::new("MyApp")
        .bundle_id("com.test.myapp")
        .output_dir(&dir);

    GradleExporter::new(config).export().unwrap();

    let android_dir = dir.join("android");
    assert!(android_dir
        .join("app")
        .join("src")
        .join("main")
        .join("AndroidManifest.xml")
        .exists());
    assert!(android_dir.join("app").join("build.gradle").exists());
    assert!(android_dir
        .join("app")
        .join("src")
        .join("main")
        .join("res")
        .join("values")
        .join("strings.xml")
        .exists());
    assert!(android_dir
        .join("app")
        .join("src")
        .join("main")
        .join("res")
        .join("values")
        .join("styles.xml")
        .exists());
    assert!(android_dir.join("gradle.properties").exists());

    let manifest = fs::read_to_string(
        android_dir
            .join("app")
            .join("src")
            .join("main")
            .join("AndroidManifest.xml"),
    )
    .unwrap();
    assert!(manifest.contains("com.test.myapp"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn export_both_targets() {
    let dir = temp_export_dir_unique("both");
    let config = ExportConfig::new("TestApp").output_dir(&dir);

    export_project(config, &[ExportTarget::All]).unwrap();

    assert!(dir.join("ios").exists());
    assert!(dir.join("android").exists());

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn export_config_defaults() {
    let config = ExportConfig::new("MyApp");
    assert_eq!(config.app_name, "MyApp");
    assert_eq!(config.bundle_id, "com.example.myapp");
    assert_eq!(config.version, "0.1.0");
}

#[test]
fn export_gradle_strings_content() {
    let dir = temp_export_dir_unique("strings");
    let config = ExportConfig::new("CoolApp").output_dir(&dir);

    GradleExporter::new(config).export().unwrap();

    let strings = fs::read_to_string(
        dir.join("android")
            .join("app")
            .join("src")
            .join("main")
            .join("res")
            .join("values")
            .join("strings.xml"),
    )
    .unwrap();
    assert!(strings.contains("CoolApp"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn export_xcode_min_ios_version() {
    let dir = temp_export_dir_unique("ios_ver");
    let config = ExportConfig::new("Test").output_dir(&dir);

    XcodeExporter::new(config).export().unwrap();

    let plist =
        fs::read_to_string(dir.join("ios").join("Test").join("Info.plist")).unwrap();
    assert!(plist.contains("15.0"));

    let _ = fs::remove_dir_all(&dir);
}
