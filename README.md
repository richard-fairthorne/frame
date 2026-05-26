# Frame

A full-stack, multiplatform app framework in Rust with GPU-native rendering via [Vello](https://github.com/linebender/vello), fine-grained reactivity, and interactive widgets — targeting all tier-1 platforms from day one.

> **Warning:** Frame is pre-0.1. The API surface is **not stable** and will change. Do not use in production.

## Tutorial

### 1. Create a new app

```bash
cargo new my-app && cd my-app
```

Add Frame to `Cargo.toml`:

```toml
[dependencies]
frame = { path = "../path/to/frame/crates/frame" }
```

### 2. Write your first app

Replace `src/main.rs` with:

```rust
#![allow(unused_braces)]

use frame::{rsx, run_app, AppBuilder, Text, Column, Color};

#[frame::main]
fn main() {
    run_app(
        AppBuilder::new().title("My App").size(400.0, 300.0),
        || Box::new(rsx! {
            <Column gap=16.0>
                <Text size=32.0 color=Color::BLACK>"Hello, Frame!"</Text>
            </Column>
        }),
    );
}
```

Run it:

```bash
cargo run                    # macOS / Linux / Windows
cargo frame run --target web # Web
```

### 3. Add interactivity

```rust
#![allow(unused_braces)]

use frame::{rsx, run_app, request_render, AppBuilder, Text, Column, Button, Color, Signal};

#[frame::main]
fn main() {
    let count = Signal::new(0);

    run_app(
        AppBuilder::new().title("Counter").size(400.0, 300.0),
        move || {
            let count = count.clone();
            Box::new(rsx! {
                <Column gap=20.0>
                    <Text size=28.0>"Counter"</Text>
                    <Text size=22.0>{format!("Count: {}", count.get())}</Text>
                    <Button
                        label="+ Increment"
                        on_click={
                            let count = count.clone();
                            move || { count.set(count.get() + 1); request_render(); }
                        }
                        background=Color::from_u8(0, 122, 255, 255)
                        padding=12.0
                    />
                </Column>
            })
        },
    );
}
```

### 4. Add navigation

```rust
use frame::{Router, Navigator, DeepLinkConfig};

let router = Router::new()
    .route("/")
    .route("/users/:id")
    .route("/settings/:section");

let navigator = Navigator::new(router);
navigator.push("/users/42");

match navigator.resolve_current() {
    Some(route) => println!("Matched route {}", route.route_index),
    None => println!("No match"),
}
```

### Key concepts

- **`#[frame::main]`** — Attribute macro that generates the correct platform entry point. One `main.rs` compiles on all 6 platforms.
- **`run_app(builder, factory)`** — Starts the app. `factory` is a closure that returns your root widget. Called each frame to rebuild the widget tree.
- **`rsx!`** — JSX-like syntax for declaring widget trees. Angle-bracket components, brace-delimited expressions.
- **`Signal<T>`** — Reactive state. Call `.get()` to read, `.set()` to write. Changes trigger a re-render via `request_render()`.
- **Widgets** — `Text`, `Column`, `Row`, `Button`, `Container`, `Stack`, `ScrollView`, `TextField`, `Slider`, `Checkbox`, `Switch`, `Grid`, `Card`, `ListView`, `Icon`, `Image`, and more.
- **`Box::new(rsx! { ... })`** — The `rsx!` macro produces a widget value. Wrap in `Box::new()` to return `Box<dyn Widget>` from the factory closure.

## Architecture

- **GPU-native rendering** — Vello draws every pixel via wgpu (Metal, Vulkan, D3D12, WebGPU)
- **Fine-grained reactivity** — SolidJS-style signals: `Signal`, `Computed`, `Effect` with automatic dependency tracking
- **Three UI syntaxes** — `view!` (brace-based), `rsx!` (JSX via rstml), builder API — all interoperable
- **Real platform APIs** — Not a winit abstraction. Each platform crate talks directly to the OS (NSWindow, Win32, GDK, UIKit, NativeActivity, Web APIs)
- **Plugin system** — Everything is a plugin. Frame's own subsystems use the same `Plugin` trait as third-party code
- **Deep links** — URL-first routing with per-platform OS registration

## Platform Support

| Platform | Entry | Window | DPR | Render | Input | Click | Resize | Plugins | Deep Links | Native Views | Quit |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| **macOS** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Android** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **iOS** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Windows** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Linux** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Web** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

## Crate Structure

```
frame-core          Reactive runtime, traits, Plugin system
frame-rendering     Vello integration, paint pipeline, text shaping
frame-layout        CSS-compliant flexbox
frame-ui            29 built-in widgets
frame-macros        view!, rsx!, #[frame::main]
frame-nav           Router, Navigator, deep links
frame-animation     Easing curves, spring physics, Tween, Timeline
frame-theme         Material-inspired design tokens, light/dark
frame-assets        Asset manager, image/font loading
frame-a11y          Accessibility tree, roles, announcer
frame-test          Test harness, widget tester
frame-macos         NSWindow, Metal, AppKit
frame-ios           UIWindow, UIKit, Metal
frame-android       NativeActivity, Vulkan
frame-windows       Win32, Direct3D
frame-linux         GTK, X11
frame-web           Canvas, WebGPU
frame               Single dependency (cfg-gated)
cargo-frame         CLI tool
```

## CLI

```bash
cargo frame run --target macos       # Build + run on macOS
cargo frame run --target android     # Build + run on Android
cargo frame run --target web         # Build + serve as web app
cargo frame build --target all       # Build all targets
cargo frame generate-association-files  # Deep link files
```

## Reactivity

```rust
let count = Signal::new(0);

let doubled = Computed::new(move || count.get() * 2);

Effect::new(move || {
    println!("Count is now: {}", count.get());
});

batch(|| {
    count.set(1);
    count.set(2);
});
```

## License

[AGPL-3.0-or-later](https://github.com/richard-fairthorne/frame/blob/main/LICENSE)
