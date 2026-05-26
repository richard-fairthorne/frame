use cargo_frame::watch::{DevServer, DevServerConfig};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cargo_frame::export::{export_project, ExportConfig, ExportTarget};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "run" => cmd_run(&args[2..]),
        "build" => cmd_build(&args[2..]),
        "test" => cmd_test(&args[2..]),
        "dev" => cmd_dev(&args[2..]),
        "export" => cmd_export(&args[2..]),
        "generate-association-files" => cmd_generate_association_files(),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    eprintln!("cargo-frame — Frame framework CLI");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  run [--target <platform>]    Build and run the app");
    eprintln!("  build [--target <platform>]   Build the app");
    eprintln!("  test [--target <platform>]    Run tests");
    eprintln!("  dev [--host/--port/--watch]   Start dev server with hot reload");
    eprintln!("  export [--target <platform>]  Export Xcode/Gradle project files");
    eprintln!("  generate-association-files    Generate deep link association files");
    eprintln!("  help                          Show this help");
    eprintln!();
    eprintln!("Platforms: ios, android, macos, windows, linux, web, all");
}

fn cmd_run(args: &[String]) {
    let target = parse_target(args);

    if target == "android" {
        cmd_run_android(args);
    } else if target == "web" {
        cmd_run_web(args);
    } else {
        let triple = target_to_triple(target);
        run_cargo("run", triple);
    }
}

fn cmd_run_android(args: &[String]) {
    use cargo_frame::android::AndroidConfig;

    let config = match AndroidConfig::detect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    eprintln!("Android SDK: {}", config.sdk_home.display());
    eprintln!("Android NDK: {}", config.ndk_home.display());
    eprintln!("API level:   {}", config.api_level);
    eprintln!("Target:      {}", config.target_triple);

    if let Err(e) = config.ensure_target_installed() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let project_dir = find_project_dir(args);

    if let Err(e) = config.write_cargo_config(&project_dir) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    eprintln!("Wrote .cargo/config.toml with NDK linker");

    let output_dir = match config.build(&project_dir, false) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let device = match config.find_emulator() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("No device found: {}", e);
            eprintln!("Attempting to start emulator...");
            if let Err(e2) = config.start_emulator() {
                eprintln!("Error: {}", e2);
                std::process::exit(1);
            }
            config.find_emulator().unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            })
        }
    };
    eprintln!("Using device: {}", device);

    let _package = parse_arg(args, "--package")
        .unwrap_or_else(|| "com.example.frameapp".to_string());
    let _activity = parse_arg(args, "--activity")
        .unwrap_or_else(|| ".MainActivity".to_string());

    let apk_name = project_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let so_path = output_dir.join(format!("lib{}.so", apk_name.replace('-', "_")));

    if !so_path.exists() {
        eprintln!("Built .so not found at {}", so_path.display());
        eprintln!("Expected the library name to match the crate name.");
        eprintln!("Make sure your Cargo.toml has [lib] crate-type = [\"cdylib\"]");
        std::process::exit(1);
    }

    eprintln!("Built: {}", so_path.display());
    eprintln!();

    match check_cargo_apk() {
        true => {
            eprintln!("Found cargo-apk. Building and installing APK...");
            let mut apk_cmd = Command::new("cargo");
            apk_cmd
                .arg("apk")
                .arg("run")
                .arg("--target")
                .arg(&config.target_triple)
                .current_dir(&project_dir);

            let status = apk_cmd.status().unwrap_or_else(|e| {
                eprintln!("Failed to run cargo-apk: {}", e);
                std::process::exit(1);
            });
            std::process::exit(status.code().unwrap_or(1));
        }
        false => {
            eprintln!("cargo-apk not found. Install it for automatic APK packaging:");
            eprintln!("  cargo install cargo-apk");
            eprintln!();
            eprintln!("Alternative: export the Gradle project and build manually:");
            eprintln!("  cargo frame export --target android");
            eprintln!("Then build with Gradle and install with adb.");
        }
    }
}

fn cmd_run_web(args: &[String]) {
    let _ = args;
    eprintln!("Web target: build with trunk or wasm-pack");
    eprintln!("  trunk serve --release");
    eprintln!("  wasm-pack build --target web");
}

fn check_cargo_apk() -> bool {
    Command::new("cargo")
        .arg("apk")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn find_project_dir(args: &[String]) -> PathBuf {
    if let Some(p) = parse_arg(args, "--project") {
        PathBuf::from(p)
    } else {
        env::current_dir().unwrap_or_default()
    }
}

fn cmd_build(args: &[String]) {
    let target = parse_target(args);
    let triple = target_to_triple(target);
    run_cargo("build", triple);
}

fn cmd_test(args: &[String]) {
    let _target = parse_target(args);
    let status = Command::new("cargo")
        .arg("test")
        .arg("--workspace")
        .status()
        .expect("Failed to run cargo test");
    std::process::exit(status.code().unwrap_or(1));
}

fn cmd_generate_association_files() {
    let scheme = parse_arg(&args_from_env(), "--scheme").unwrap_or_else(|| "myapp".to_string());
    let domains = parse_domains_from_args();
    let bundle_id = parse_arg(&args_from_env(), "--bundle-id").unwrap_or_else(|| "com.example.app".to_string());

    let output_dir = PathBuf::from("association-files");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    if !domains.is_empty() {
        generate_apple_association(&output_dir, &domains, &bundle_id);
        generate_assetlinks(&output_dir, &domains);
    }

    generate_uri_scheme_handler(&output_dir, &scheme);

    eprintln!("Generated association files in {}:", output_dir.display());
    eprintln!("  apple-app-site-association  → host at https://<domain>/.well-known/");
    eprintln!("  assetlinks.json             → host at https://<domain>/.well-known/");
    eprintln!("  .desktop (Linux)            → install to ~/.local/share/applications/");
    eprintln!();
    eprintln!("Use --domains <domain1,domain2> to specify your domains.");
    eprintln!("Use --scheme <scheme> to set the URI scheme (default: myapp).");
    eprintln!("Use --bundle-id <id> to set the bundle identifier.");
}

fn generate_apple_association(dir: &Path, _domains: &[String], bundle_id: &str) {
    let content = format!(
        r#"{{
  "applinks": {{
    "details": [
      {{
        "appIDs": [ "{bundle_id}" ],
        "components": [ {{ "/": "/*" }} ]
      }}
    ]
  }},
  "webcredentials": {{
    "apps": [ "{bundle_id}" ]
  }}
}}"#,
        bundle_id = bundle_id,
    );
    fs::write(dir.join("apple-app-site-association"), content).expect("Failed to write apple-app-site-association");
}

fn generate_assetlinks(dir: &Path, _domains: &[String]) {
    let content = r#"[
  {
    "relation": ["delegate_permission/common.handle_all_urls"],
    "target": {
      "namespace": "android_app",
      "package_name": "com.example.app",
      "sha256_cert_fingerprints": [
        "INSERT_YOUR_SIGNING_KEY_FINGERPRINT_HERE"
      ]
    }
  }
]"#;
    fs::write(dir.join("assetlinks.json"), content).expect("Failed to write assetlinks.json");
}

fn generate_uri_scheme_handler(dir: &Path, scheme: &str) {
    let desktop_content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Frame App\n\
         Exec=/usr/bin/frame-app %%u\n\
         MimeType=x-scheme-handler/{scheme};\n\
         NoDisplay=true\n",
        scheme = scheme,
    );
    fs::write(dir.join("frame-app.desktop"), desktop_content).expect("Failed to write .desktop file");
}

fn args_from_env() -> Vec<String> {
    env::args().collect()
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        i += 1;
    }
    None
}

fn parse_domains_from_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if let Some(domains_str) = parse_arg(&args, "--domains") {
        domains_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![]
    }
}

fn cmd_dev(args: &[String]) {
    let mut config = DevServerConfig::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--host" if i + 1 < args.len() => {
                config = config.host(&args[i + 1]);
                i += 2;
            }
            "--port" if i + 1 < args.len() => {
                if let Ok(port) = args[i + 1].parse::<u16>() {
                    config = config.port(port);
                }
                i += 2;
            }
            "--watch" if i + 1 < args.len() => {
                config = config.watch(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => i += 1,
        }
    }

    let mut server = DevServer::new(config);
    server.run();
}

fn cmd_export(args: &[String]) {
    let app_name = "FrameApp";
    let target = parse_target(args);

    let targets = match target {
        "ios" => vec![ExportTarget::Ios],
        "android" => vec![ExportTarget::Android],
        "all" => vec![ExportTarget::All],
        _ => vec![ExportTarget::All],
    };

    let config = ExportConfig::new(app_name);
    match export_project(config, &targets) {
        Ok(()) => eprintln!("Project exported successfully."),
        Err(e) => eprintln!("Export failed: {}", e),
    }
}

fn parse_target(args: &[String]) -> &str {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--target" && i + 1 < args.len() {
            return &args[i + 1];
        }
        i += 1;
    }
    "macos"
}

fn target_to_triple(target: &str) -> &str {
    match target {
        "ios" => "aarch64-apple-ios",
        "android" => "aarch64-linux-android",
        "macos" => "aarch64-apple-darwin",
        "windows" => "x86_64-pc-windows-msvc",
        "linux" => "x86_64-unknown-linux-gnu",
        "web" => "wasm32-unknown-unknown",
        _ => target,
    }
}

fn run_cargo(command: &str, target: &str) {
    let status = Command::new("cargo")
        .arg(command)
        .arg("--target")
        .arg(target)
        .status()
        .expect("Failed to run cargo");
    std::process::exit(status.code().unwrap_or(1));
}
