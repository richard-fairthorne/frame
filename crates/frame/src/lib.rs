pub use frame_core::{
    batch, Color, Computed, Constraints, Effect, EventEmitter, EventStream, FrameApp,
    FrameAppBuilder, Layout, LayoutNode, LayoutResult, Plugin, PluginContext, PluginHost, Point,
    Rect, RenderContext, RenderSurface, Renderer, Signal, Size, Widget, WidgetId, WidgetNode,
    WidgetOutput, WindowConfig, WindowHandle, WindowHost, WindowId,
};

pub use frame_rendering::{SceneBuilder, VelloRenderer, VelloSurface};

pub use frame_layout::{
    AlignItems, FlexDirection, FlexEngine, FlexNode, FlexStyle, JustifyContent,
};

pub use frame_ui::{
    Button, Card, Column, Container, CrossAxisAlignment, Dropdown, Expanded, Flex,
    FlexDirection as UiFlexDirection, GpuWidget, Grid, Icon, IconKind, ListView, MainAxisAlignment,
    PaintWidget, Row, ScrollView, Sized, Stack, Switch, Text,
};

pub use frame_ui::{FontWeight, Style, TextStyle};

pub use frame_nav::{DeepLinkConfig, Navigator, Route, RouteMatch, RouteParams, Router};

pub use frame_input::{
    InputEvent, InputPlugin, KeyEvent, Modifiers, MouseButton, MouseEvent, TouchEvent, TouchPhase,
};

pub use frame_window::WindowManager;

pub use frame_lifecycle::{LifecycleEvent, LifecyclePlugin};

pub use frame_animation::{
    Animation, AnimationCurve, AnimationHandle, AnimationState, AnimationTarget, Spring,
    SpringConfig, SpringState, Timeline,
};

pub use frame_assets::{
    Asset, AssetBundle, AssetId, AssetManager, AssetState, FontAsset, FontId, ImageAsset,
    ImageFormat, ResourceBundle,
};

pub use frame_a11y::{
    AccessibilityAnnouncer, AccessibilityNode, AccessibilityProperties, AccessibilityRole,
    AccessibilityTree, Announcement, AnnouncementPriority,
};

pub mod theme {
    pub use frame_theme::*;
}

pub use frame_macros::rsx;
pub use frame_macros::view;

pub use frame_macros::frame_main as main;

#[cfg(target_os = "android")]
pub use android_activity::AndroidApp;

#[cfg(target_os = "android")]
pub mod android {
    use std::sync::OnceLock;

    static ANDROID_APP: OnceLock<android_activity::AndroidApp> = OnceLock::new();

    pub fn set_app(app: android_activity::AndroidApp) {
        ANDROID_APP.set(app).ok();
    }

    pub fn take_app() -> Option<android_activity::AndroidApp> {
        ANDROID_APP.take()
    }
}

#[cfg(not(target_os = "android"))]
pub mod android {
    pub struct AndroidApp;
}

mod platform {
    use frame_core::traits::widget::Widget;

    pub struct AppBuilder {
        pub title: String,
        pub width: f32,
        pub height: f32,
        pub resizable: bool,
        pub(crate) plugins: Vec<Box<dyn frame_core::Plugin>>,
    }

    impl AppBuilder {
        pub fn new() -> Self {
            Self {
                title: "Frame App".into(),
                width: 800.0,
                height: 600.0,
                resizable: true,
                plugins: Vec::new(),
            }
        }

        pub fn title(mut self, title: &str) -> Self {
            self.title = title.into();
            self
        }

        pub fn size(mut self, width: f32, height: f32) -> Self {
            self.width = width;
            self.height = height;
            self
        }

        pub fn resizable(mut self, resizable: bool) -> Self {
            self.resizable = resizable;
            self
        }

        pub fn plugin(mut self, plugin: Box<dyn frame_core::Plugin>) -> Self {
            self.plugins.push(plugin);
            self
        }
    }

    impl Default for AppBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    pub fn run_app<F>(mut builder: AppBuilder, root_factory: F)
    where
        F: Fn() -> Box<dyn Widget> + 'static,
    {
        let plugins = std::mem::take(&mut builder.plugins);

        #[cfg(target_os = "macos")]
        {
            let mut mb = frame_macos::AppBuilder::new()
                .title(&builder.title)
                .size(builder.width, builder.height)
                .resizable(builder.resizable);
            for plugin in plugins {
                mb = mb.plugin(plugin);
            }
            frame_macos::run_app(mb, root_factory);
        }

        #[cfg(target_os = "linux")]
        {
            let mut lb = frame_linux::AppBuilder::new()
                .title(&builder.title)
                .size(builder.width, builder.height)
                .resizable(builder.resizable);
            for plugin in plugins {
                lb = lb.plugin(plugin);
            }
            frame_linux::run_app(lb, root_factory);
        }

        #[cfg(target_os = "ios")]
        {
            let mut ib = frame_ios::AppBuilder::new()
                .title(&builder.title)
                .size(builder.width, builder.height)
                .resizable(builder.resizable);
            for plugin in plugins {
                ib = ib.plugin(plugin);
            }
            frame_ios::run_app(ib, root_factory);
        }

        #[cfg(target_os = "android")]
        {
            match crate::android::take_app() {
                Some(app) => {
                    let mut ab = frame_android::AppBuilder::new()
                        .title(&builder.title)
                        .size(builder.width, builder.height);
                    for plugin in plugins {
                        ab = ab.plugin(plugin);
                    }
                    frame_android::run_app_with(app, ab, root_factory);
                }
                None => {
                    eprintln!("frame: Android app not initialized. Use #[frame::main] or call frame::android::set_app().");
                    std::process::exit(1);
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let mut wb = frame_windows::AppBuilder::new()
                .title(&builder.title)
                .size(builder.width, builder.height)
                .resizable(builder.resizable);
            for plugin in plugins {
                wb = wb.plugin(plugin);
            }
            frame_windows::run_app(wb, root_factory);
        }

        #[cfg(target_arch = "wasm32")]
        {
            let mut wb = frame_web::AppBuilder::new()
                .title(&builder.title)
                .size(builder.width, builder.height);
            for plugin in plugins {
                wb = wb.plugin(plugin);
            }
            frame_web::run_app(wb, root_factory);
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "ios", target_os = "android", target_os = "windows", target_arch = "wasm32")))]
        {
            let _ = (builder, root_factory, plugins);
            unimplemented!("Frame: platform not yet supported.");
        }
    }

    pub fn request_render() {
        #[cfg(target_os = "macos")]
        {
            frame_macos::request_render();
        }

        #[cfg(target_os = "linux")]
        {
        }

        #[cfg(target_os = "android")]
        {
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
        {
        }
    }

    #[allow(dead_code)]
    pub fn init() {
        #[cfg(target_os = "macos")]
        {}

        #[cfg(target_os = "ios")]
        {}

        #[cfg(target_os = "android")]
        {}

        #[cfg(target_os = "windows")]
        {}

        #[cfg(target_os = "linux")]
        {}

        #[cfg(target_arch = "wasm32")]
        {}
    }
}

pub use platform::{request_render, run_app, AppBuilder};

#[cfg(target_os = "macos")]
pub use frame_macos::{MacosApp, MacosPlatform, MacosWindowManager};

#[cfg(target_os = "ios")]
pub use frame_ios::{IosApp, IosPlatform, IosWindowManager};

#[cfg(target_os = "android")]
pub use frame_android::{
    run_app_with as android_run_app_with, AndroidPlatform, AndroidWindowManager,
    AppBuilder as AndroidAppBuilder, NativeWindow,
};

#[cfg(target_os = "windows")]
pub use frame_windows::{WindowsPlatform, WindowsWindowManager, WgpuSurfaceProvider, AppBuilder as WindowsAppBuilder};

#[cfg(target_os = "linux")]
pub use frame_linux::{LinuxPlatform, LinuxWindowManager};

#[cfg(target_arch = "wasm32")]
pub use frame_web::{WebPlatform, WebWindowManager};
