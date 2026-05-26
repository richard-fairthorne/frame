# Frame

A full-stack, multiplatform app framework in Rust with GPU-native rendering via [Vello](https://github.com/linebender/vello), fine-grained reactivity, and interactive widgets — targeting all tier-1 platforms from day one.

> **Warning:** Frame is pre-0.1. The API surface is **not stable** and will change. Do not use in production.

## Quick Start

```toml
[dependencies]
frame = "0.1"
```

```rust
use frame::prelude::*;

fn main() {
    frame::run_app(
        AppBuilder::new().title("My App").size(800.0, 600.0),
        || {
            let count = Signal::new(0);
            rsx! {
                <Column gap={8.0}>
                    <Text size={24.0}>"Count: {count.get()}"</Text>
                    <Button label="Increment" on_click={move || count.set(count.get() + 1)} />
                </Column>
            }
        },
    );
}
```

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

// Automatic dependency tracking
let doubled = Computed::new(move || count.get() * 2);

// Side effects re-run when dependencies change
Effect::new(move || {
    println!("Count is now: {}", count.get());
});

// Batched updates — only one re-render
batch(|| {
    count.set(1);
    count.set(2); // intermediate value skipped
});
```

## License

[AGPL-3.0-or-later](https://github.com/richard-fairthorne/frame/blob/main/LICENSE)
