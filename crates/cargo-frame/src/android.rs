use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct AndroidConfig {
    pub ndk_home: PathBuf,
    pub sdk_home: PathBuf,
    pub api_level: u32,
    pub target_triple: String,
}

impl AndroidConfig {
    pub fn detect() -> Result<Self, String> {
        let sdk_home = find_sdk_home()?;
        let ndk_home = find_ndk_home(&sdk_home)?;
        let api_level = find_api_level(&sdk_home).unwrap_or(29);
        let target_triple = "aarch64-linux-android".to_string();

        Ok(Self {
            ndk_home,
            sdk_home,
            api_level,
            target_triple,
        })
    }

    pub fn linker_path(&self) -> PathBuf {
        let host_tag = host_tag();
        let max_ndk_api = find_max_ndk_api(&self.ndk_home, &self.target_triple);
        let api = match max_ndk_api {
            Some(max) => self.api_level.min(max),
            None => self.api_level,
        };
        self.ndk_home
            .join("toolchains")
            .join("llvm")
            .join("prebuilt")
            .join(host_tag)
            .join("bin")
            .join(format!("{}{}-clang", self.target_triple, api))
    }

    pub fn adb_path(&self) -> PathBuf {
        self.sdk_home.join("platform-tools").join("adb")
    }

    pub fn ensure_target_installed(&self) -> Result<(), String> {
        let output = Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output()
            .map_err(|e| format!("Failed to run rustup: {}", e))?;

        let installed = String::from_utf8_lossy(&output.stdout);
        if !installed.contains(&self.target_triple) {
            eprintln!("Installing {} target...", self.target_triple);
            let status = Command::new("rustup")
                .args(["target", "add", &self.target_triple])
                .status()
                .map_err(|e| format!("Failed to run rustup: {}", e))?;

            if !status.success() {
                return Err(format!("Failed to install {}", self.target_triple));
            }
        }
        Ok(())
    }

    pub fn write_cargo_config(&self, project_dir: &Path) -> Result<(), String> {
        let cargo_dir = project_dir.join(".cargo");
        fs::create_dir_all(&cargo_dir).map_err(|e| e.to_string())?;

        let linker = self.linker_path();
        if !linker.exists() {
            return Err(format!("NDK linker not found at {}", linker.display()));
        }

        let config = format!(
            "[target.{}]\nlinker = \"{}\"\n",
            self.target_triple,
            linker.display()
        );
        fs::write(cargo_dir.join("config.toml"), config).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn build(&self, project_dir: &Path, release: bool) -> Result<PathBuf, String> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target")
            .arg(&self.target_triple);

        if release {
            cmd.arg("--release");
        }

        cmd.current_dir(project_dir);

        let status = cmd.status().map_err(|e| format!("Failed to run cargo build: {}", e))?;
        if !status.success() {
            return Err("Build failed".into());
        }

        let profile = if release { "release" } else { "debug" };
        Ok(project_dir
            .join("target")
            .join(&self.target_triple)
            .join(profile)
            .to_path_buf())
    }

    pub fn find_emulator(&self) -> Result<String, String> {
        let adb = self.adb_path();
        if !adb.exists() {
            return Err("adb not found at {}. Is Android SDK platform-tools installed?".into());
        }

        let output = Command::new(&adb)
            .args(["devices"])
            .output()
            .map_err(|e| format!("Failed to run adb: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                return Ok(parts[0].to_string());
            }
        }

        Err("No Android device or emulator found. Start an emulator or connect a device.".into())
    }

    pub fn start_emulator(&self) -> Result<(), String> {
        let emulator = self.sdk_home.join("emulator").join("emulator");
        if !emulator.exists() {
            return Err("Android emulator not found. Install via Android Studio > Tools > SDK Manager.".into());
        }

        let avds = list_avds(&emulator)?;
        if avds.is_empty() {
            return Err("No AVDs found. Create one with: android-studio > Tools > Device Manager".into());
        }

        eprintln!("Starting emulator '{}'...", avds[0]);
        Command::new(&emulator)
            .args(["-avd", &avds[0], "-no-snapshot-load"])
            .spawn()
            .map_err(|e| format!("Failed to start emulator: {}", e))?;

        eprintln!("Waiting for device to boot...");
        let adb = self.adb_path();
        loop {
            let output = match Command::new(&adb)
                .args(["shell", "getprop", "sys.boot_completed"])
                .output()
            {
                Ok(o) => o,
                Err(_) => {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    continue;
                }
            };
            if String::from_utf8_lossy(&output.stdout).trim() == "1" {
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        eprintln!("Emulator ready.");
        Ok(())
    }

    pub fn install_and_launch(
        &self,
        apk_path: &Path,
        package: &str,
        activity: &str,
    ) -> Result<(), String> {
        let adb = self.adb_path();

        eprintln!("Installing {}...", apk_path.display());
        let status = Command::new(&adb)
            .args(["install", "-r"])
            .arg(apk_path)
            .status()
            .map_err(|e| format!("Failed to run adb install: {}", e))?;

        if !status.success() {
            return Err("adb install failed".into());
        }

        eprintln!("Launching {}.{}...", package, activity);
        let status = Command::new(&adb)
            .args([
                "shell",
                "am",
                "start",
                "-n",
                &format!("{}/{}", package, activity),
            ])
            .status()
            .map_err(|e| format!("Failed to launch activity: {}", e))?;

        if !status.success() {
            return Err("Failed to launch activity".into());
        }
        Ok(())
    }
}

fn find_sdk_home() -> Result<PathBuf, String> {
    if let Ok(v) = env::var("ANDROID_HOME") {
        let p = PathBuf::from(&v);
        if p.exists() {
            return Ok(p);
        }
    }

    if let Ok(v) = env::var("ANDROID_SDK_ROOT") {
        let p = PathBuf::from(&v);
        if p.exists() {
            return Ok(p);
        }
    }

    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".into());

    let candidates = vec![
        PathBuf::from(format!("{home}/Library/Android/sdk")),
        PathBuf::from(format!("{home}/Android/Sdk")),
        PathBuf::from(format!("{home}/android-sdk")),
    ];

    for c in &candidates {
        if c.exists() {
            return Ok(c.clone());
        }
    }

    Err(format!(
        "Android SDK not found. Set ANDROID_HOME or install via Android Studio.\nChecked: {:?}",
        candidates
    ))
}

fn find_ndk_home(sdk_home: &Path) -> Result<PathBuf, String> {
    if let Ok(v) = env::var("ANDROID_NDK_HOME") {
        let p = PathBuf::from(&v);
        if p.exists() {
            return Ok(p);
        }
    }

    let ndk_dir = sdk_home.join("ndk");
    if ndk_dir.exists() {
        if let Ok(entries) = fs::read_dir(&ndk_dir) {
            let mut versions: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .collect();
            versions.sort_by_key(|e| e.file_name());
            if let Some(latest) = versions.last() {
                return Ok(latest.path());
            }
        }
    }

    let ndk_bundle = sdk_home.join("ndk-bundle");
    if ndk_bundle.exists() {
        return Ok(ndk_bundle);
    }

    Err(format!(
        "Android NDK not found. Install via Android Studio > Tools > SDK Manager > NDK.\nExpected at: {}",
        ndk_dir.display()
    ))
}

fn find_api_level(sdk_home: &Path) -> Option<u32> {
    let platforms = sdk_home.join("platforms");
    if !platforms.exists() {
        return None;
    }

    let mut max_level = 0u32;
    if let Ok(entries) = fs::read_dir(&platforms) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if let Some(level_str) = name_str.strip_prefix("android-") {
                if let Ok(level) = level_str.parse::<u32>() {
                    max_level = max_level.max(level);
                }
            }
        }
    }
    if max_level > 0 { Some(max_level) } else { None }
}

fn find_max_ndk_api(ndk_home: &Path, target_triple: &str) -> Option<u32> {
    let host_tag = host_tag();
    let bin_dir = ndk_home
        .join("toolchains")
        .join("llvm")
        .join("prebuilt")
        .join(&host_tag)
        .join("bin");

    let mut max_level = 0u32;

    if let Ok(entries) = fs::read_dir(&bin_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let suffix = "-clang";
            if name.starts_with(target_triple) && name.ends_with(suffix) && !name.contains("-clang++") {
                let middle = &name[target_triple.len()..name.len() - suffix.len()];
                if let Ok(level) = middle.parse::<u32>() {
                    max_level = max_level.max(level);
                }
            }
        }
    }

    if max_level > 0 { Some(max_level) } else { None }
}

fn host_tag() -> String {
    if cfg!(target_os = "macos") {
        "darwin-x86_64".into()
    } else if cfg!(target_os = "linux") {
        "linux-x86_64".into()
    } else if cfg!(target_os = "windows") {
        "windows-x86_64".into()
    } else {
        "unknown".into()
    }
}

fn list_avds(emulator: &Path) -> Result<Vec<String>, String> {
    let output = Command::new(emulator)
        .args(["-list-avds"])
        .output()
        .map_err(|e| format!("Failed to list AVDs: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect())
}
