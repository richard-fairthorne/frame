use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub app_name: String,
    pub bundle_id: String,
    pub version: String,
    pub min_ios_version: String,
    pub min_android_sdk: u32,
    pub output_dir: PathBuf,
}

impl ExportConfig {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            bundle_id: format!("com.example.{}", app_name.to_lowercase()),
            version: "0.1.0".into(),
            min_ios_version: "15.0".into(),
            min_android_sdk: 24,
            output_dir: PathBuf::from("export"),
        }
    }

    pub fn bundle_id(mut self, id: &str) -> Self {
        self.bundle_id = id.into();
        self
    }
    pub fn version(mut self, v: &str) -> Self {
        self.version = v.into();
        self
    }
    pub fn output_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.output_dir = dir.into();
        self
    }
}

pub struct XcodeExporter {
    config: ExportConfig,
}

impl XcodeExporter {
    pub fn new(config: ExportConfig) -> Self {
        Self { config }
    }

    pub fn export(&self) -> Result<(), String> {
        let dir = self
            .config
            .output_dir
            .join("ios")
            .join(&self.config.app_name);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        self.write_info_plist(&dir)?;
        self.write_entitlements(&dir)?;
        self.write_launch_screen(&dir)?;
        self.write_bridge_header(&dir)?;
        self.write_xcode_project(&dir)?;

        Ok(())
    }

    fn write_info_plist(&self, dir: &Path) -> Result<(), String> {
        let content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>{app_name}</string>
    <key>CFBundleIdentifier</key>
    <string>{bundle_id}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{app_name}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>{version}</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>MinimumOSVersion</key>
    <string>{min_ios}</string>
    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
</dict>
</plist>"#,
            app_name = self.config.app_name,
            bundle_id = self.config.bundle_id,
            version = self.config.version,
            min_ios = self.config.min_ios_version,
        );
        fs::write(dir.join("Info.plist"), content).map_err(|e| e.to_string())
    }

    fn write_entitlements(&self, dir: &Path) -> Result<(), String> {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
</dict>
</plist>"#;
        fs::write(
            dir.join(format!("{}.entitlements", self.config.app_name)),
            content,
        )
        .map_err(|e| e.to_string())
    }

    fn write_launch_screen(&self, dir: &Path) -> Result<(), String> {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<document type="com.apple.InterfaceBuilder3.CocoaTouch.Storyboard.XIB" version="3.0">
    <scenes>
        <scene sceneID="launch">
            <objects>
                <viewController id="launchVC" sceneMemberID="viewController">
                    <view key="view" contentMode="scaleToFill" id="view">
                        <rect key="frame" x="0.0" y="0.0" width="414" height="896"/>
                        <autoresizingMask key="autoresizingMask" widthSizable="YES" heightSizable="YES"/>
                        <color key="backgroundColor" white="1" alpha="1" colorSpace="custom" customColorSpace="genericGamma22GrayColorSpace"/>
                    </view>
                </viewController>
            </objects>
        </scene>
    </scenes>
</document>"#;
        let storyboard_dir = dir.join("Base.lproj");
        fs::create_dir_all(&storyboard_dir).map_err(|e| e.to_string())?;
        fs::write(storyboard_dir.join("LaunchScreen.storyboard"), content).map_err(|e| e.to_string())
    }

    fn write_bridge_header(&self, dir: &Path) -> Result<(), String> {
        let content = format!(
            "// Bridge header for {}\n// Add Objective-C headers here\n",
            self.config.app_name
        );
        fs::write(
            dir.join(format!("{}-Bridging-Header.h", self.config.app_name)),
            content,
        )
        .map_err(|e| e.to_string())
    }

    fn write_xcode_project(&self, dir: &Path) -> Result<(), String> {
        let pbxproj_dir = dir.join(format!("{}.xcodeproj", self.config.app_name));
        fs::create_dir_all(&pbxproj_dir).map_err(|e| e.to_string())?;
        let content = format!(
            "// Xcode project for {}\n// Generated by cargo-frame\n",
            self.config.app_name
        );
        fs::write(pbxproj_dir.join("project.pbxproj"), content).map_err(|e| e.to_string())
    }
}

pub struct GradleExporter {
    config: ExportConfig,
}

impl GradleExporter {
    pub fn new(config: ExportConfig) -> Self {
        Self { config }
    }

    pub fn export(&self) -> Result<(), String> {
        let dir = self.config.output_dir.join("android");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        self.write_manifest(&dir)?;
        self.write_build_gradle(&dir)?;
        self.write_strings_xml(&dir)?;
        self.write_styles_xml(&dir)?;
        self.write_main_activity(&dir)?;
        self.write_gradle_properties(&dir)?;
        self.write_gradle_wrapper(&dir)?;

        Ok(())
    }

    fn write_manifest(&self, dir: &Path) -> Result<(), String> {
        let content = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{bundle_id}">
    
    <uses-sdk android:minSdkVersion="{min_sdk}" android:targetSdkVersion="34"/>
    
    <application
        android:allowBackup="true"
        android:label="@string/app_name"
        android:theme="@style/AppTheme">
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|keyboardHidden">
            <intent-filter>
                <action android:name="android.intent.action.MAIN"/>
                <category android:name="android.intent.category.LAUNCHER"/>
            </intent-filter>
        </activity>
    </application>
</manifest>"#,
            bundle_id = self.config.bundle_id,
            min_sdk = self.config.min_android_sdk,
        );
        let manifest_dir = dir.join("app").join("src").join("main");
        fs::create_dir_all(&manifest_dir).map_err(|e| e.to_string())?;
        fs::write(manifest_dir.join("AndroidManifest.xml"), content).map_err(|e| e.to_string())
    }

    fn write_build_gradle(&self, dir: &Path) -> Result<(), String> {
        let content = format!(
            r#"plugins {{
    id 'com.android.application'
    id 'org.jetbrains.kotlin.android'
}}

android {{
    namespace '{bundle_id}'
    compileSdk 34
    
    defaultConfig {{
        applicationId "{bundle_id}"
        minSdk {min_sdk}
        targetSdk 34
        versionCode 1
        versionName "{version}"
    }}
    
    buildTypes {{
        release {{
            minifyEnabled false
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }}
    }}
}}

dependencies {{
    implementation 'androidx.core:core-ktx:1.12.0'
    implementation 'androidx.appcompat:appcompat:1.6.1'
}}"#,
            bundle_id = self.config.bundle_id,
            min_sdk = self.config.min_android_sdk,
            version = self.config.version,
        );
        let app_dir = dir.join("app");
        fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
        fs::write(app_dir.join("build.gradle"), content).map_err(|e| e.to_string())
    }

    fn write_strings_xml(&self, dir: &Path) -> Result<(), String> {
        let content = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">{app_name}</string>
</resources>"#,
            app_name = self.config.app_name,
        );
        let values_dir = dir
            .join("app")
            .join("src")
            .join("main")
            .join("res")
            .join("values");
        fs::create_dir_all(&values_dir).map_err(|e| e.to_string())?;
        fs::write(values_dir.join("strings.xml"), content).map_err(|e| e.to_string())
    }

    fn write_styles_xml(&self, dir: &Path) -> Result<(), String> {
        let content = r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <style name="AppTheme" parent="Theme.AppCompat.Light.NoActionBar">
        <item name="android:windowFullscreen">true</item>
    </style>
</resources>"#;
        let values_dir = dir
            .join("app")
            .join("src")
            .join("main")
            .join("res")
            .join("values");
        fs::create_dir_all(&values_dir).map_err(|e| e.to_string())?;
        fs::write(values_dir.join("styles.xml"), content).map_err(|e| e.to_string())
    }

    fn write_main_activity(&self, dir: &Path) -> Result<(), String> {
        let package_path = self.config.bundle_id.replace('.', "/");
        let java_dir = dir
            .join("app")
            .join("src")
            .join("main")
            .join("java")
            .join(&package_path);
        fs::create_dir_all(&java_dir).map_err(|e| e.to_string())?;

        let content = format!(
            r#"package {bundle_id};

import android.app.Activity;
import android.os.Bundle;

public class MainActivity extends Activity {{
    static {{
        System.loadLibrary("{app_name}");
    }}

    @Override
    protected void onCreate(Bundle savedInstanceState) {{
        super.onCreate(savedInstanceState);
        // Frame will handle rendering via native surface
    }}
}}"#,
            bundle_id = self.config.bundle_id,
            app_name = self.config.app_name.to_lowercase().replace('-', "_"),
        );
        fs::write(java_dir.join("MainActivity.java"), content).map_err(|e| e.to_string())
    }

    fn write_gradle_properties(&self, dir: &Path) -> Result<(), String> {
        let content = "org.gradle.jvmargs=-Xmx2048m\nandroid.useAndroidX=true";
        fs::write(dir.join("gradle.properties"), content).map_err(|e| e.to_string())
    }

    fn write_gradle_wrapper(&self, dir: &Path) -> Result<(), String> {
        let properties = "distributionBase=GRADLE_USER_HOME\ndistributionPath=wrapper/dists\ndistributionUrl=https\\://services.gradle.org/distributions/gradle-8.4-bin.zip\n";
        let wrapper_dir = dir.join("gradle").join("wrapper");
        fs::create_dir_all(&wrapper_dir).map_err(|e| e.to_string())?;
        fs::write(
            wrapper_dir.join("gradle-wrapper.properties"),
            properties,
        )
        .map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportTarget {
    Ios,
    Android,
    All,
}

pub fn export_project(config: ExportConfig, targets: &[ExportTarget]) -> Result<(), String> {
    for target in targets {
        match target {
            ExportTarget::Ios => XcodeExporter::new(config.clone()).export()?,
            ExportTarget::Android => GradleExporter::new(config.clone()).export()?,
            ExportTarget::All => {
                XcodeExporter::new(config.clone()).export()?;
                GradleExporter::new(config.clone()).export()?;
            }
        }
    }
    Ok(())
}
