# Frame — Multiplatform App Framework Design

## Overview

Frame is a full-stack, cross-platform application framework written in Rust. It targets all major platforms — iOS, Android, macOS, Windows, Linux, and the Web — simultaneously from day one. Frame provides GPU-native rendering via Vello, a JSX-like UI syntax, fine-grained reactivity, and a plugin system where everything (including Frame's own internals) is a regular Cargo crate implementing the Plugin trait.

Frame is designed for Rust-first developers. It leans into Rust's type system, lifetimes, and zero-cost abstractions rather than hiding them.

## Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Framework type | Full-stack (UI + app services) | Like Flutter — one framework for everything |
| UI syntax | view! macro + builder API (both) | Macro for ergonomics, builder as the underlying expansion target |
| State management | Fine-grained reactive primitives (SolidJS-style) | Only affected widgets re-render; maximum performance |
| Rendering | GPU-native via Vello, with native view escape hatch | Consistent cross-platform rendering; platform views when needed |
| Plugin system | Rust crates implementing a Plugin trait | Standard Cargo ecosystem; no special loading mechanism |
| Target audience | Rust-first developers | Leverage Rust idioms, not fight them |
| Platform priority | All tier-1 simultaneously | Platform-agnostic architecture from day one |
| Async/runtime | Framework owns the main thread event loop | OS requires it; hidden from the developer behind reactivity |

## Crate Structure

```
frame/
├── crates/
│   ├── frame-core/           # Reactive runtime, Plugin trait, coordination traits
│   ├── frame-rendering/      # Vello integration, implements Renderer trait
│   ├── frame-layout/         # Flexbox, grid, implements Layout trait
│   ├── frame-input/          # Keyboard, touch, mouse — Plugin
│   ├── frame-window/         # Window management — Plugin + WindowHost trait
│   ├── frame-lifecycle/      # App lifecycle — Plugin
│   ├── frame-ui/             # Widget library + view! macro + builder API
│   ├── frame-macros/         # Procedural macros (view!, etc.)
│   ├── frame/
│   │   ├── ios/              # iOS PlatformContext
│   │   ├── android/          # Android PlatformContext
│   │   ├── windows/          # Windows PlatformContext
│   │   ├── linux/            # Linux PlatformContext
│   │   ├── macos/            # macOS PlatformContext
│   │   └── web/              # Web PlatformContext
│   └── frame/                # Convenience re-export crate (single dependency)
├── Cargo.toml                # Workspace root
└── examples/
    └── hello-world/          # Minimal example app
```

### Dependency Graph

```
frame-ui ──→ frame-core
          ──→ frame-rendering
          ──→ frame-layout
          ──→ frame-input
          ──→ frame-macros

frame-rendering ──→ frame-core, vello
frame-layout ──→ frame-core
frame-input ──→ frame-core (Plugin)
frame-window ──→ frame-core (Plugin)
frame-lifecycle ──→ frame-core (Plugin)

frame-ios ──→ frame-core (provides PlatformContext)
frame-android ──→ frame-core
frame-windows ──→ frame-core
frame-linux ──→ frame-core
frame-macos ──→ frame-core
frame-web ──→ frame-core

frame (convenience) ──→ everything, with cfg-gated platform crates
```

### Dependency Rules

- `frame-core` depends on nothing except std and small utility crates (no Vello, no platform code)
- Platform crates depend only on `frame-core`
- Third-party plugin crates depend on `frame-core` + relevant platform crate(s)
- `frame-ui` ties rendering, layout, and input together
- App developers specify one dependency: `frame = "0.1"` — the convenience crate uses `cfg` to compile only the matching platform

## Reactive Runtime (`frame-core`)

Fine-grained reactivity where only the exact widgets that depend on changed state re-render.

### Primitives

```rust
// A reactive value. When it changes, anything tracking it re-evaluates.
let count = Signal::new(0);

// A computed value derived from signals. Auto-updates when dependencies change.
let label = Computed::new(move || format!("Count: {}", count.get()));

// A side effect that runs when tracked signals change.
Effect::new(move || {
    println!("{}", count.get());
});

// Batch multiple signal changes into a single update cycle.
batch(|| {
    count.set(1);
    other_signal.set(2);
});
```

### Integration with Widget Tree

```rust
let name = Signal::new(String::from("World"));

view! {
    <Column>
        <Text>{Computed::new(move || format!("Hello, {}!", name.get()))}</Text>
        <Button on_click={move || name.set(String::from("Frame"))}>
            {"Change name"}
        </Button>
    </Column>
}
```

When `name` changes, only the `Text` widget re-evaluates. `Column` and `Button` are unaffected.

### Internal Model

- **Signal** — holds a value, maintains a subscriber list. `.get()` in a tracking scope (Computed, Effect, widget render) establishes a dependency. `.set()` triggers subscribers.
- **Computed** — lazily evaluated, cached. Re-evaluates only when a dependency changed since last access.
- **Effect** — eagerly evaluated side effect. Runs immediately, re-runs when dependencies change.
- **batch()** — defers subscriber notifications until the closure completes. Prevents cascade updates.

### Update Cycle

1. Signal changes (via `.set()` or batch commit)
2. Dependent Computeds are marked dirty (not re-evaluated yet)
3. Effects scheduled for re-run
4. Rendering requests a frame from the compositor
5. On frame: dirty widgets re-evaluate, produce new render tree fragments
6. Vello renders only the changed regions

No global state — the reactive runtime is owned by the Frame application instance. Signals are `Send + Sync` for cross-thread use, but UI updates are always marshalled to the main thread.

## Core Traits (`frame-core`)

### Plugin Trait — The Universal Interface

```rust
pub trait Plugin: 'static + Send + Sync {
    fn init(&mut self, ctx: &mut PluginContext<'_>);
    fn on_pause(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_destroy(&mut self) {}
}
```

Everything is a plugin, including Frame's own subsystems (input, window, lifecycle). Third-party plugins and Frame internals use the same trait. There is no privileged code path.

### PluginContext — Platform Access + Reactive Integration

```rust
pub struct PluginContext<'a> {
    inner: &'a mut PlatformContext,
    events: &'a EventEmitter,
}

impl<'a> PluginContext<'a> {
    pub fn create_stream<T: 'static + Send + Sync>(&self) -> EventStream<T>;
    pub fn platform(&self) -> &PlatformContext;
}
```

PlatformContext is defined per-platform. See Platform Integration section for the full trait definition.

Plugins talk directly to the platform via `PluginContext`. Frame does not route platform events. The plugin registers its own callbacks with the OS and receives them directly.

### Renderer Trait — Implemented by `frame-rendering`

```rust
pub trait Renderer: 'static {
    type Surface;

    fn create_surface(&mut self, window: &WindowHandle) -> Self::Surface;
    fn render(&mut self, scene: &Scene, surface: &mut Self::Surface);
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;
}
```

### Layout Trait — Implemented by `frame-layout`

```rust
pub trait Layout: 'static {
    fn measure(&self, node: LayoutNode, constraints: Constraints) -> Size;
    fn layout(&self, node: LayoutNode, bounds: Rect) -> Vec<(LayoutNode, Rect)>;
}
```

### WindowHost Trait — Implemented by `frame-window`

```rust
pub trait WindowHost: Plugin {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle;
    fn set_title(&mut self, handle: WindowHandle, title: &str);
    fn resize(&mut self, handle: WindowHandle, size: Size);
}
```

### Widget Trait — Implemented by `frame-ui` Widgets

```rust
pub trait Widget: 'static {
    fn render(&mut self, ctx: &mut RenderContext) -> WidgetOutput;
    fn measure(&self, constraints: Constraints) -> Size;
}

pub enum WidgetOutput {
    None,
    Text { content: String, style: TextStyle },
    Paint { paint_fn: Box<dyn FnMut(&mut Scene)> },
    Gpu { gpu_fn: Box<dyn FnMut(&mut GpuContext)> },
    Children { children: Vec<WidgetNode> },
}
```

### FrameApp — The Coordinator

```rust
struct FrameApp {
    plugins: Vec<Box<dyn Plugin>>,
    renderer: Box<dyn Renderer>,
    layout: Box<dyn Layout>,
    window_host: Box<dyn WindowHost>,
    event_emitter: EventEmitter,
    root_widget: Option<WidgetNode>,
}
```

## UI Layer (`frame-ui` + `frame-macros`)

Three ways to express UI, all interoperable:

### 1. view! Macro — JSX-like Ergonomics

```rust
let count = Signal::new(0);
let color = Signal::new(Color::BLUE);

view! {
    <Column gap={16.0} padding={24.0}>
        <Text size={32.0} color={color.get()}>
            {Computed::new(move || format!("Count: {}", count.get()))}
        </Text>
        <Row gap={8.0}>
            <Button on_click={move || count.set(count.get() + 1)}>
                {"+"}
            </Button>
            <Button on_click={move || count.set(count.get() - 1)}>
                {"-"}
            </Button>
        </Row>
    </Column>
}
```

### 2. Builder API — What the Macro Expands To

```rust
Column::new()
    .gap(16.0)
    .padding(24.0)
    .child(
        Text::new(Computed::new(move || format!("Count: {}", count.get())))
            .size(32.0)
            .color(color.get())
    )
    .child(
        Row::new()
            .gap(8.0)
            .child(
                Button::new("+")
                    .on_click(move || count.set(count.get() + 1))
            )
            .child(
                Button::new("-")
                    .on_click(move || count.set(count.get() - 1))
            )
    )
```

### 3. Custom Widgets — Implement Widget Directly

```rust
struct Spinner {
    progress: Signal<f32>,
}

impl Widget for Spinner {
    fn render(&mut self, ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Paint {
            paint_fn: Box::new(|scene| {
                let progress = self.progress.get();
                // Draw arc into Vello scene
            }),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        Size::new(48.0, 48.0)
    }
}
```

### Macro Rules

- Element names map to widget types: `<Column>` → `Column::new()`
- Attributes map to builder methods: `gap={16.0}` → `.gap(16.0)`
- String children `{"text"}` become `Text::new("text")`
- Expression children `{signal.get()}` are tracked reactively
- Event handlers `on_click={|| ...}` are closures
- Custom components work: `<MyWidget prop={value} />`

### Widget Spectrum

| Widget type | What you implement | Use case |
|---|---|---|
| Standard widget | `Widget` trait with `WidgetOutput::Children` | Normal UI |
| Custom 2D painting | `Widget` trait with `WidgetOutput::Paint` | Custom charts, vector graphics |
| Raw GPU widget | `Widget` trait with `WidgetOutput::Gpu` | 3D, shaders, GPU compute, game rendering |

### Built-in Widgets

| Category | Widgets |
|---|---|
| Layout | `Column`, `Row`, `Stack`, `Flex`, `Grid`, `Expanded`, `Sized` |
| Display | `Text`, `Image`, `Icon` |
| Input | `Button`, `TextField`, `Checkbox`, `Slider`, `Switch`, `Dropdown` |
| Container | `Container`, `Card`, `ScrollView`, `ListView` |
| Navigation | `Navigator`, `Route` |
| Custom | `PaintWidget`, `GpuWidget` (escape hatches) |
| Platform | `NativeView` (embeds platform-native view) |

## Navigation & Deep Linking

### Route Definitions — URL-first

Every route is a URL pattern. Every route automatically works as a deep link on every platform.

```rust
let router = Router::new()
    .route("/", home_page)
    .route("/users/:id", user_page)
    .route("/products/:category/:id", product_page)
    .nest("/admin", admin_router);
```

### Usage in Views

```rust
view! {
    <Navigator router={router}>
        <Route path="/" view={Home} />
        <Route path="/users/:id" view={UserPage} />
        <Route path="/products/:category/:id" view={ProductPage} />
    </Navigator>
}
```

### Programmatic Navigation

```rust
let navigator = Navigator::current();
navigator.push("/users/42");
navigator.replace("/login");
navigator.back();
```

### Deep Link Architecture

The router IS the deep link handler. The platform crate handles OS-specific registration; when a deep link arrives, it passes the URL to the router.

| Platform | Mechanism | What the Platform Crate Does |
|---|---|---|
| iOS | Universal Links (`https://yourdomain.com/path`) via `apple-app-site-association` file. Custom schemes (`myapp://`) via Info.plist. | Registers Associated Domains, receives `NSUserActivity` or `application(_:continue:restorationHandler:)`, passes URL to router |
| Android | App Links (`https://yourdomain.com/path`) via `assetlinks.json` with signing key fingerprint. Custom schemes via intent filters. | Configures intent filter in manifest with `autoVerify="true"`, receives `onNewIntent()`, passes URI to router |
| macOS | Universal Links — same as iOS (Associated Domains, `apple-app-site-association`) | Registers domains, receives callbacks, passes URL to router |
| Windows | Protocol activation — registers URI scheme (`myapp://`) in app manifest. No domain-based linking. | Registers protocol in manifest, receives `OnActivated` with `ProtocolActivatedEventArgs`, passes URI to router |
| Linux | URI scheme handler via `.desktop` file (`MimeType=x-scheme-handler/myapp;`). No domain-based linking. | Generates `.desktop` file, receives URI via `gtk_application_open()`, passes to router |
| Web | Native — browser URL bar IS the deep link. History API for push/replace. | Maps URL bar to router, uses `popstate` for back navigation |

### Deep Link Configuration

```rust
let config = DeepLinkConfig {
    scheme: "myapp",
    domains: vec!["myapp.com", "app.myapp.com"],
};

let app = FrameApp::builder()
    .deep_links(config)
    .router(router)
    .run(root_view);
```

### Association File Generation

```
cargo frame generate-association-files
```

Outputs:
- `apple-app-site-association` — host at `https://myapp.com/.well-known/apple-app-site-association`
- `assetlinks.json` — host at `https://myapp.com/.well-known/assetlinks.json`

### Deep Link Flow

```
User taps https://myapp.com/users/42
        │
        ▼
OS recognizes domain (association file verified)
        │
        ▼
Platform crate receives URL via OS callback
        │
        ▼
Platform crate calls router.resolve("/users/42")
        │
        ▼
Router matches "/users/:id", extracts id=42
        │
        ▼
UserPage widget renders with user_id=42
```

If the app is not running, the OS launches it, the platform crate receives the URL during `init()`, and the router navigates to the correct page on first render.

## Rendering Pipeline (`frame-rendering`)

### Paint Cycle

```
Signal changes
     │
     ▼
Dirty widgets marked (fine-grained — only affected subtrees)
     │
     ▼
Layout pass (frame-layout measures and positions dirty subtrees)
     │
     ▼
Paint pass — widgets produce render commands
     │
     ├─ Standard widgets → Vello Scene commands (fill, stroke, text, image)
     ├─ PaintWidget      → Custom Vello Scene commands
     └─ GpuWidget        → Direct wgpu render pass
     │
     ▼
Compositing — Vello renders Scene + GPU layers composited into final frame
     │
     ▼
Present to surface
```

### Frame Scheduling

```rust
enum VsyncMode {
    SyncToDisplay,  // Sync to display refresh rate (default)
    OnDemand,       // Only re-render when dirty (saves GPU on static UI)
}
```

If no Signal changed, no frame is rendered. When a Signal changes, only widgets in its dependency graph are re-evaluated.

### VelloRenderer Implementation

```rust
pub struct VelloRenderer {
    context: vello::RenderContext,
    scenes: HashMap<WindowId, vello::Scene>,
    gpu_widgets: Vec<GpuLayer>,
}

impl Renderer for VelloRenderer {
    type Surface = VelloSurface;

    fn create_surface(&mut self, window: &WindowHandle) -> Self::Surface;
    fn render(&mut self, scene: &Scene, surface: &mut Self::Surface);
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;
}
```

### Native View Embedding

When a widget needs a platform-native view (map, video player, web content), Frame carves a transparent hole in its render surface and lets the platform place a native view behind it.

```rust
view! {
    <Column>
        <Text>{"Here's a map:"}</Text>
        <NativeView
            kind={NativeViewKind::Map}
            params={MapParams::new().center(37.7749, -122.4194).zoom(14)}
            style={Style::new().width(300.0).height(400.0)}
        />
    </Column>
}
```

How it works:
- Frame tells the platform crate the rect coordinates of the `NativeView`
- The platform crate creates and positions the native view at those coordinates
- Frame's Vello render skips that region (transparent)
- The native view renders independently, composited by the OS window system
- Resize and reposition are handled reactively — layout changes update the native view's frame

### Thread Model

```
Main thread:          UI events → reactive updates → layout → paint commands
Render thread:        Vello scene rendering → GPU submission → present
Platform thread(s):   OS callbacks, plugin work (marshalled to main thread for UI)
```

The main thread produces scene descriptions; a dedicated render thread submits them to the GPU. This keeps the UI responsive during heavy rendering.

## Platform Integration

### Platform Crate Responsibilities

| Responsibility | iOS | Android | macOS | Windows | Linux | Web |
|---|---|---|---|---|---|---|
| App entry point | `UIApplicationMain` | `android.app.Activity` | `NSApplicationMain` | `WinMain` | `gtk_application_run` | WASM entry |
| Window creation | `UIWindow` | Activity window | `NSWindow` | `HWND` | `GtkWindow` | `<canvas>` element |
| Input events | UIKit touch/gesture | MotionEvent/KeyEvent | NSEvent | WM_INPUT | GdkEvent | JS event listeners |
| Lifecycle | `UIApplicationDelegate` | Activity lifecycle | `NSApplicationDelegate` | WM_ACTIVATE | `GtkApplication` | Page Visibility API |
| Deep link receipt | `NSUserActivity` | `onNewIntent` | `NSUserActivity` | `OnActivated` | `gtk_application_open` | URL bar / popstate |
| Native view embed | UIView hierarchy | View hierarchy | NSView hierarchy | HWND child | GtkWidget packing | DOM overlay |

### App Entry — Entrypoint Macro

```rust
#[frame::main]
fn main() {
    let app = FrameApp::builder()
        .plugin(MyPlugin::new())
        .router(router)
        .run(root_view);
}
```

Each platform crate expands `#[frame::main]` to the appropriate OS entry point:
- iOS: `UIApplicationMain` with FrameAppDelegate
- Android: JNI_OnLoad → registers Activity subclass
- macOS: `NSApplicationMain` with FrameAppDelegate
- Windows: `WinMain`
- Linux: `gtk_application_run`
- Web: `wasm_bindgen(start)` → creates canvas, initializes wgpu via WebGPU

### PlatformContext Trait

```rust
pub trait PlatformContext {
    fn create_window(&mut self, config: WindowConfig) -> PlatformWindowHandle;
    fn embed_native_view(&mut self, view: NativeViewRequest) -> NativeViewHandle;
    fn update_native_view_rect(&mut self, handle: &NativeViewHandle, rect: Rect);
    fn remove_native_view(&mut self, handle: &NativeViewHandle);
    fn on_deep_link(&mut self, handler: Box<dyn Fn(Uri) + Send + Sync>);
    fn run_on_main_thread(&self, task: Box<dyn FnOnce() + Send>);
    fn raw(&self) -> &dyn Any;
}
```

### Single Dependency for App Developers

```toml
[dependencies]
frame = "0.1"
```

The convenience crate uses `cfg` conditional compilation to include only the matching platform crate. No per-target configuration needed in the app's Cargo.toml.

### CLI Tool

```
cargo frame run --target ios        # Build + run on iOS simulator/device
cargo frame run --target android    # Build + run on Android emulator/device
cargo frame run --target macos      # Build + run as macOS app
cargo frame run --target windows    # Build + run as Windows app
cargo frame run --target linux      # Build + run as Linux app
cargo frame run --target web        # Build + serve as web app
cargo frame build --target all      # Build all targets
cargo frame generate-association-files  # Deep link association files
```

## Plugin Architecture

### Philosophy

Everything is a plugin. Frame's own subsystems (input, window, lifecycle) are plugins. Third-party functionality is plugins. Same trait, same mechanism.

### Plugin-to-Platform Communication

Plugins talk directly to the platform via `PluginContext`. Frame does not route platform events. The plugin registers its own callbacks with the OS and receives them through the platform crate's bridge objects.

On iOS, the platform crate (`frame-ios`) creates Objective-C delegate objects with Rust method bodies using the `objc` crate. When iOS calls the delegate, the bridge routes the call into the Rust closure. The plugin author writes only Rust.

Same pattern on every platform:
- iOS/macOS: `objc` crate creates delegate objects
- Android: JNI + small Java shim shipped with platform crate
- Web: `wasm-bindgen` passes Rust closures to browser APIs
- Windows: `windows-rs` implements COM interfaces in Rust

The plugin author needs to understand platform concepts (Intents, BroadcastReceivers, PKPushRegistry) but expresses them entirely in Rust.

### Example Plugin

```rust
struct CameraPlugin {
    camera_stream: EventStream<CameraFrame>,
}

impl Plugin for CameraPlugin {
    fn init(&mut self, ctx: &mut PluginContext<'_>) {
        let frames = ctx.create_stream::<CameraFrame>();
        self.camera_stream = frames.clone();

        ctx.platform().start_camera(move |frame| {
            frames.emit(frame);
        });
    }

    fn on_pause(&mut self) {
        // Platform-specific: pause camera
    }

    fn on_resume(&mut self) {
        // Platform-specific: resume camera
    }

    fn on_destroy(&mut self) {
        // Platform-specific: release camera
    }
}
```

### Event Flow

1. Plugin calls `ctx.platform().start_camera()` during `init()`
2. Platform crate creates OS-specific camera session (AVCaptureSession on iOS, Camera2 on Android)
3. OS delivers frames to the bridge object
4. Bridge object calls the Rust closure
5. Closure emits frame into EventStream
6. Reactive system tracks which widgets depend on the stream
7. Only affected widgets re-render
