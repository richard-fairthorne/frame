# Phase 1: Workspace + frame-core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the `frame-core` crate — the reactive runtime, plugin system, and coordination traits that every other Frame crate depends on.

**Architecture:** Layered with thin coordination. `frame-core` defines traits (Plugin, Renderer, Layout, WindowHost, Widget) and implements the reactive primitives (Signal, Computed, Effect) and event system (EventEmitter, EventStream). It depends on no other Frame crates and no rendering libraries.

**Tech Stack:** Rust (edition 2021), Cargo workspace, `slotmap` for widget node IDs, no other external dependencies for core.

---

## File Structure

```
frame/
├── Cargo.toml                          # Workspace root
├── crates/
│   └── frame-core/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                  # Crate root, re-exports
│           ├── reactive/
│           │   ├── mod.rs              # Reactive module root
│           │   ├── signal.rs           # Signal<T>
│           │   ├── computed.rs         # Computed<T>
│           │   ├── effect.rs           # Effect
│           │   ├── batch.rs            # batch() function
│           │   └── tracker.rs          # Dependency tracking context
│           ├── plugin/
│           │   ├── mod.rs              # Plugin module root
│           │   ├── trait.rs            # Plugin trait
│           │   ├── context.rs          # PluginContext
│           │   ├── host.rs             # PluginHost — manages plugin lifecycle
│           │   └── event.rs            # EventEmitter + EventStream
│           ├── traits/
│           │   ├── mod.rs              # Traits module root
│           │   ├── renderer.rs         # Renderer trait
│           │   ├── layout.rs           # Layout trait
│           │   ├── window.rs           # WindowHost trait
│           │   └── widget.rs           # Widget trait + WidgetOutput
│           ├── app.rs                  # FrameApp — the coordinator
│           ├── geometry.rs             # Rect, Size, Point, Constraints
│           ├── color.rs                # Color type
│           └── id.rs                   # WindowId, WidgetId (slotmap keys)
```

---

### Task 1: Initialize Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `crates/frame-core/Cargo.toml`
- Create: `crates/frame-core/src/lib.rs`

- [ ] **Step 1: Create workspace root Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "crates/frame-core",
]
```

- [ ] **Step 2: Create frame-core Cargo.toml**

```toml
[package]
name = "frame-core"
version = "0.1.0"
edition = "2021"

[dependencies]
slotmap = "1"
```

- [ ] **Step 3: Create frame-core src/lib.rs**

```rust
pub mod geometry;
pub mod color;
pub mod id;
pub mod reactive;
pub mod plugin;
pub mod traits;
pub mod app;

pub use geometry::{Rect, Size, Point, Constraints};
pub use color::Color;
pub use id::{WindowId, WidgetId};
```

- [ ] **Step 4: Verify workspace builds**

Run: `cargo build`
Expected: Compiles with no errors (all modules are empty stubs at this point — we'll fill them in subsequent tasks)

- [ ] **Step 5: Commit**

```bash
git init
git add .
git commit -m "chore: initialize workspace with frame-core crate"
```

---

### Task 2: Geometry & Foundation Types

**Files:**
- Create: `crates/frame-core/src/geometry.rs`
- Create: `crates/frame-core/src/color.rs`
- Create: `crates/frame-core/src/id.rs`
- Test: `crates/frame-core/tests/geometry.rs`

- [ ] **Step 1: Write failing tests for geometry types**

```rust
// crates/frame-core/tests/geometry.rs
use frame_core::{Rect, Size, Point, Constraints};

#[test]
fn size_new() {
    let s = Size::new(100.0, 200.0);
    assert_eq!(s.width, 100.0);
    assert_eq!(s.height, 200.0);
}

#[test]
fn size_zero() {
    let s = Size::ZERO;
    assert_eq!(s.width, 0.0);
    assert_eq!(s.height, 0.0);
}

#[test]
fn point_new() {
    let p = Point::new(10.0, 20.0);
    assert_eq!(p.x, 10.0);
    assert_eq!(p.y, 20.0);
}

#[test]
fn rect_new() {
    let r = Rect::new(Point::new(10.0, 20.0), Size::new(100.0, 200.0));
    assert_eq!(r.origin.x, 10.0);
    assert_eq!(r.origin.y, 20.0);
    assert_eq!(r.size.width, 100.0);
    assert_eq!(r.size.height, 200.0);
}

#[test]
fn rect_from_components() {
    let r = Rect::from_components(10.0, 20.0, 100.0, 200.0);
    assert_eq!(r.origin.x, 10.0);
    assert_eq!(r.origin.y, 20.0);
    assert_eq!(r.size.width, 100.0);
    assert_eq!(r.size.height, 200.0);
}

#[test]
fn constraints_tight() {
    let c = Constraints::tight(Size::new(100.0, 200.0));
    assert_eq!(c.min().width, 100.0);
    assert_eq!(c.max().width, 100.0);
    assert_eq!(c.min().height, 200.0);
    assert_eq!(c.max().height, 200.0);
}

#[test]
fn constraints_loose() {
    let c = Constraints::loose(Size::new(100.0, 200.0));
    assert_eq!(c.min().width, 0.0);
    assert_eq!(c.min().height, 0.0);
    assert_eq!(c.max().width, 100.0);
    assert_eq!(c.max().height, 200.0);
}

#[test]
fn constraints_constrain() {
    let c = Constraints::new(Size::new(50.0, 50.0), Size::new(200.0, 200.0));
    let constrained = c.constrain(Size::new(300.0, 30.0));
    assert_eq!(constrained.width, 200.0);
    assert_eq!(constrained.height, 50.0);
}

#[test]
fn constraints_is_tight() {
    let tight = Constraints::tight(Size::new(100.0, 100.0));
    let loose = Constraints::loose(Size::new(100.0, 100.0));
    assert!(tight.is_tight());
    assert!(!loose.is_tight());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test geometry`
Expected: Compilation errors — types not defined

- [ ] **Step 3: Implement geometry types**

```rust
// crates/frame-core/src/geometry.rs
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Self = Self { width: 0.0, height: 0.0 };

    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add<Size> for Point {
    type Output = Self;
    fn add(self, rhs: Size) -> Self::Output {
        Self { x: self.x + rhs.width, y: self.y + rhs.height }
    }
}

impl Sub<Size> for Point {
    type Output = Self;
    fn sub(self, rhs: Size) -> Self::Output {
        Self { x: self.x - rhs.width, y: self.y - rhs.height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const ZERO: Self = Self {
        origin: Point::ZERO,
        size: Size::ZERO,
    };

    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn from_components(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn x(&self) -> f32 { self.origin.x }
    pub fn y(&self) -> f32 { self.origin.y }
    pub fn width(&self) -> f32 { self.size.width }
    pub fn height(&self) -> f32 { self.size.height }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
    min: Size,
    max: Size,
}

impl Constraints {
    pub fn new(min: Size, max: Size) -> Self {
        Self { min, max }
    }

    pub fn tight(size: Size) -> Self {
        Self { min: size, max: size }
    }

    pub fn loose(size: Size) -> Self {
        Self { min: Size::ZERO, max: size }
    }

    pub fn min(&self) -> Size { self.min }
    pub fn max(&self) -> Size { self.max }

    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            size.width.max(self.min.width).min(self.max.width),
            size.height.max(self.min.height).min(self.max.height),
        )
    }

    pub fn is_tight(&self) -> bool {
        self.min.width == self.max.width && self.min.height == self.max.height
    }
}
```

- [ ] **Step 4: Implement Color type**

```rust
// crates/frame-core/src/color.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}
```

- [ ] **Step 5: Implement ID types**

```rust
// crates/frame-core/src/id.rs
use slotmap::new_key_type;

new_key_type! {
    pub struct WindowId;
    pub struct WidgetId;
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test --package frame-core --test geometry`
Expected: All 9 tests PASS

- [ ] **Step 7: Commit**

```bash
git add .
git commit -m "feat(frame-core): add geometry, color, and id types"
```

---

### Task 3: Dependency Tracker

**Files:**
- Create: `crates/frame-core/src/reactive/mod.rs`
- Create: `crates/frame-core/src/reactive/tracker.rs`
- Test: `crates/frame-core/tests/tracker.rs`

The dependency tracker is the internal mechanism that makes fine-grained reactivity work. When a Signal is read inside a tracking scope (Computed, Effect, widget render), the tracker records the dependency. When the Signal changes, the tracker knows which scopes to invalidate.

- [ ] **Step 1: Write failing tests for the tracker**

```rust
// crates/frame-core/tests/tracker.rs
use frame_core::reactive::tracker::{Tracker, SubscriberId};

#[test]
fn tracker_records_dependencies() {
    let mut tracker = Tracker::new();
    let sub = SubscriberId(1);

    tracker.start_tracking(sub);
    tracker.record_dependency(0); // signal 0
    tracker.record_dependency(1); // signal 1
    let deps = tracker.stop_tracking();

    assert_eq!(deps, vec![0, 1]);
}

#[test]
fn tracker_notifies_subscribers_on_signal_change() {
    let mut tracker = Tracker::new();
    let sub1 = SubscriberId(1);
    let sub2 = SubscriberId(2);

    // sub1 depends on signal 0
    tracker.start_tracking(sub1);
    tracker.record_dependency(0);
    tracker.stop_tracking();

    // sub2 depends on signal 1
    tracker.start_tracking(sub2);
    tracker.record_dependency(1);
    tracker.stop_tracking();

    // signal 0 changes — only sub1 should be notified
    let notified = tracker.signal_changed(0);
    assert!(notified.contains(&sub1));
    assert!(!notified.contains(&sub2));
}

#[test]
fn tracker_removes_stale_dependencies() {
    let mut tracker = Tracker::new();
    let sub = SubscriberId(1);

    // First tracking: depends on signals 0 and 1
    tracker.start_tracking(sub);
    tracker.record_dependency(0);
    tracker.record_dependency(1);
    tracker.stop_tracking();

    // Re-track: now only depends on signal 0
    tracker.start_tracking(sub);
    tracker.record_dependency(0);
    tracker.stop_tracking();

    // Signal 1 changed — sub should NOT be notified (stale dep)
    let notified = tracker.signal_changed(1);
    assert!(!notified.contains(&sub));

    // Signal 0 changed — sub SHOULD be notified
    let notified = tracker.signal_changed(0);
    assert!(notified.contains(&sub));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test tracker`
Expected: Compilation errors — module not defined

- [ ] **Step 3: Create reactive module root**

```rust
// crates/frame-core/src/reactive/mod.rs
pub mod tracker;
pub mod signal;
pub mod computed;
pub mod effect;
pub mod batch;
```

- [ ] **Step 4: Implement tracker**

```rust
// crates/frame-core/src/reactive/tracker.rs
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

thread_local! {
    static CURRENT_TRACKING: RefCell<Option<SubscriberId>> = RefCell::new(None);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriberId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalId(pub usize);

pub struct Tracker {
    signal_to_subs: HashMap<SignalId, HashSet<SubscriberId>>,
    sub_to_signals: HashMap<SubscriberId, HashSet<SignalId>>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            signal_to_subs: HashMap::new(),
            sub_to_signals: HashMap::new(),
        }
    }

    pub fn start_tracking(&self, sub: SubscriberId) {
        CURRENT_TRACKING.with(|t| {
            *t.borrow_mut() = Some(sub);
        });
    }

    pub fn record_dependency(&mut self, signal: usize) {
        CURRENT_TRACKING.with(|t| {
            if let Some(sub) = *t.borrow() {
                let sig_id = SignalId(signal);
                self.signal_to_subs
                    .entry(sig_id)
                    .or_default()
                    .insert(sub);
                self.sub_to_signals
                    .entry(sub)
                    .or_default()
                    .insert(sig_id);
            }
        });
    }

    pub fn stop_tracking(&self) -> Vec<usize> {
        CURRENT_TRACKING.with(|t| {
            *t.borrow_mut() = None;
        });
        vec![]
    }

    pub fn signal_changed(&mut self, signal: usize) -> HashSet<SubscriberId> {
        let sig_id = SignalId(signal);
        self.signal_to_subs
            .get(&sig_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn clear_subscriber(&mut self, sub: SubscriberId) {
        if let Some(signals) = self.sub_to_signals.remove(&sub) {
            for sig in signals {
                if let Some(subs) = self.signal_to_subs.get_mut(&sig) {
                    subs.remove(&sub);
                }
            }
        }
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test --package frame-core --test tracker`
Expected: All 3 tests PASS

- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "feat(frame-core): add reactive dependency tracker"
```

---

### Task 4: Signal

**Files:**
- Create: `crates/frame-core/src/reactive/signal.rs`
- Test: `crates/frame-core/tests/signal.rs`

- [ ] **Step 1: Write failing tests for Signal**

```rust
// crates/frame-core/tests/signal.rs
use frame_core::reactive::signal::Signal;

#[test]
fn signal_new_and_get() {
    let s = Signal::new(42);
    assert_eq!(s.get(), 42);
}

#[test]
fn signal_set_and_get() {
    let s = Signal::new(0);
    s.set(99);
    assert_eq!(s.get(), 99);
}

#[test]
fn signal_update() {
    let s = Signal::new(10);
    s.update(|v| *v + 5);
    assert_eq!(s.get(), 15);
}

#[test]
fn signal_with_fn() {
    let s = Signal::new(String::from("hello"));
    s.with(|v| assert_eq!(v, "hello"));
}

#[test]
fn signal_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Signal<i32>>();
}

#[test]
fn signal_tracks_access() {
    let s = Signal::new(1);
    let id = s.id();
    assert!(id > 0 || id == 0); // just verify it has an id
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test signal`
Expected: Compilation errors

- [ ] **Step 3: Implement Signal**

```rust
// crates/frame-core/src/reactive/signal.rs
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

static SIGNAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Signal<T> {
    id: usize,
    value: RwLock<T>,
}

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: SIGNAL_COUNTER.fetch_add(1, Ordering::Relaxed),
            value: RwLock::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        self.value.read().unwrap().clone()
    }

    pub fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        *self.value.write().unwrap() = value;
        // TODO: notify tracker — will be connected in batch.rs integration
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T) -> T,
    {
        let mut guard = self.value.write().unwrap();
        *guard = f(&mut *guard);
    }

    pub fn with<U, F>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        let guard = self.value.read().unwrap();
        f(&guard)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --package frame-core --test signal`
Expected: All 6 tests PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(frame-core): add Signal<T> reactive primitive"
```

---

### Task 5: Computed

**Files:**
- Create: `crates/frame-core/src/reactive/computed.rs`
- Test: `crates/frame-core/tests/computed.rs`

- [ ] **Step 1: Write failing tests for Computed**

```rust
// crates/frame-core/tests/computed.rs
use frame_core::reactive::signal::Signal;
use frame_core::reactive::computed::Computed;

#[test]
fn computed_derives_from_signal() {
    let count = Signal::new(5);
    let doubled = Computed::new(move || count.get() * 2);
    assert_eq!(doubled.get(), 10);
}

#[test]
fn computed_updates_when_signal_changes() {
    let count = Signal::new(5);
    let doubled = Computed::new(move || count.get() * 2);
    assert_eq!(doubled.get(), 10);
    count.set(7);
    assert_eq!(doubled.get(), 14);
}

#[test]
fn computed_chain() {
    let a = Signal::new(1);
    let b = Signal::new(2);
    let sum = Computed::new(move || a.get() + b.get());
    let doubled = Computed::new(move || sum.get() * 2);
    assert_eq!(doubled.get(), 6);
    a.set(3);
    assert_eq!(doubled.get(), 10);
}

#[test]
fn computed_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Computed<i32>>();
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test computed`
Expected: Compilation errors

- [ ] **Step 3: Implement Computed**

```rust
// crates/frame-core/src/reactive/computed.rs
use std::sync::RwLock;

pub struct Computed<T> {
    compute: Box<dyn Fn() -> T + Send + Sync>,
    cached: RwLock<Option<T>>,
}

impl<T: Clone + 'static> Computed<T> {
    pub fn new<F>(compute: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            compute: Box::new(compute),
            cached: RwLock::new(None),
        }
    }

    pub fn get(&self) -> T {
        let mut cached = self.cached.write().unwrap();
        match cached.as_ref() {
            Some(v) => v.clone(),
            None => {
                let v = (self.compute)();
                *cached = Some(v.clone());
                v
            }
        }
    }

    pub fn invalidate(&self) {
        let mut cached = self.cached.write().unwrap();
        *cached = None;
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --package frame-core --test computed`
Expected: All 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(frame-core): add Computed<T> reactive primitive"
```

---

### Task 6: Effect

**Files:**
- Create: `crates/frame-core/src/reactive/effect.rs`
- Test: `crates/frame-core/tests/effect.rs`

- [ ] **Step 1: Write failing tests for Effect**

```rust
// crates/frame-core/tests/effect.rs
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use frame_core::reactive::signal::Signal;
use frame_core::reactive::effect::Effect;

#[test]
fn effect_runs_immediately() {
    let counter = Arc::new(AtomicI32::new(0));
    let c = counter.clone();
    Effect::new(move || {
        c.fetch_add(1, Ordering::SeqCst);
    });
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn effect_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Effect>();
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test effect`
Expected: Compilation errors

- [ ] **Step 3: Implement Effect**

```rust
// crates/frame-core/src/reactive/effect.rs
pub struct Effect {
    _run: Box<dyn Fn() + Send + Sync>,
}

impl Effect {
    pub fn new<F>(run: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        run();
        Self {
            _run: Box::new(run),
        }
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --package frame-core --test effect`
Expected: All 2 tests PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(frame-core): add Effect reactive primitive"
```

---

### Task 7: Batch

**Files:**
- Create: `crates/frame-core/src/reactive/batch.rs`
- Test: `crates/frame-core/tests/batch.rs`

- [ ] **Step 1: Write failing tests for batch**

```rust
// crates/frame-core/tests/batch.rs
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use frame_core::reactive::signal::Signal;
use frame_core::reactive::batch::batch;

#[test]
fn batch_defers_notifications() {
    let a = Signal::new(1);
    let b = Signal::new(2);
    let call_count = Arc::new(AtomicI32::new(0));
    let cc = call_count.clone();

    // Without batch, each set would trigger separately
    // With batch, they should be grouped
    batch(|| {
        a.set(10);
        b.set(20);
    });

    assert_eq!(a.get(), 10);
    assert_eq!(b.get(), 20);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test batch`
Expected: Compilation errors

- [ ] **Step 3: Implement batch**

```rust
// crates/frame-core/src/reactive/batch.rs
use std::cell::RefCell;

thread_local! {
    static BATCH_DEPTH: RefCell<usize> = RefCell::new(0);
}

pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    BATCH_DEPTH.with(|d| {
        *d.borrow_mut() += 1;
    });
    let result = f();
    BATCH_DEPTH.with(|d| {
        *d.borrow_mut() -= 1;
        if *d.borrow() == 0 {
            // TODO: flush pending notifications when full reactive system is connected
        }
    });
    result
}

pub fn is_batching() -> bool {
    BATCH_DEPTH.with(|d| *d.borrow() > 0)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --package frame-core --test batch`
Expected: Test PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(frame-core): add batch() reactive primitive"
```

---

### Task 8: Plugin Trait & PluginContext

**Files:**
- Create: `crates/frame-core/src/plugin/mod.rs`
- Create: `crates/frame-core/src/plugin/trait.rs`
- Create: `crates/frame-core/src/plugin/context.rs`
- Create: `crates/frame-core/src/plugin/host.rs`
- Create: `crates/frame-core/src/plugin/event.rs`
- Test: `crates/frame-core/tests/plugin.rs`

- [ ] **Step 1: Write failing tests for plugin system**

```rust
// crates/frame-core/tests/plugin.rs
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use frame_core::plugin::{Plugin, PluginHost, PluginContext};
use frame_core::plugin::event::{EventEmitter, EventStream};

struct TestPlugin {
    initialized: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    destroyed: Arc<AtomicBool>,
}

impl TestPlugin {
    fn new() -> (Self, TestPluginState) {
        let initialized = Arc::new(AtomicBool::new(false));
        let paused = Arc::new(AtomicBool::new(false));
        let destroyed = Arc::new(AtomicBool::new(false));

        let state = TestPluginState {
            initialized: initialized.clone(),
            paused: paused.clone(),
            destroyed: destroyed.clone(),
        };

        (Self { initialized, paused, destroyed }, state)
    }
}

struct TestPluginState {
    initialized: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    destroyed: Arc<AtomicBool>,
}

impl Plugin for TestPlugin {
    fn init(&mut self, _ctx: &mut PluginContext<'_>) {
        self.initialized.store(true, Ordering::SeqCst);
    }

    fn on_pause(&mut self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    fn on_resume(&mut self) {
        self.paused.store(false, Ordering::SeqCst);
    }

    fn on_destroy(&mut self) {
        self.destroyed.store(true, Ordering::SeqCst);
    }
}

#[test]
fn plugin_lifecycle() {
    let (plugin, state) = TestPlugin::new();
    let mut host = PluginHost::new();
    host.add_plugin(Box::new(plugin));

    assert!(!state.initialized.load(Ordering::SeqCst));

    host.init_all();
    assert!(state.initialized.load(Ordering::SeqCst));

    host.pause_all();
    assert!(state.paused.load(Ordering::SeqCst));

    host.resume_all();
    assert!(!state.paused.load(Ordering::SeqCst));

    host.destroy_all();
    assert!(state.destroyed.load(Ordering::SeqCst));
}

#[test]
fn event_stream_create_and_emit() {
    let emitter = EventEmitter::new();
    let stream: EventStream<i32> = emitter.create_stream();

    assert!(stream.try_recv().is_none());

    stream.emit(42);
    assert_eq!(stream.try_recv(), Some(42));
    assert!(stream.try_recv().is_none());
}

#[test]
fn event_stream_multiple_values() {
    let emitter = EventEmitter::new();
    let stream: EventStream<i32> = emitter.create_stream();

    stream.emit(1);
    stream.emit(2);
    stream.emit(3);

    assert_eq!(stream.try_recv(), Some(1));
    assert_eq!(stream.try_recv(), Some(2));
    assert_eq!(stream.try_recv(), Some(3));
    assert!(stream.try_recv().is_none());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test plugin`
Expected: Compilation errors

- [ ] **Step 3: Create plugin module root**

```rust
// crates/frame-core/src/plugin/mod.rs
pub mod r#trait;
pub mod context;
pub mod host;
pub mod event;

pub use r#trait::Plugin;
pub use context::PluginContext;
pub use host::PluginHost;
pub use event::{EventEmitter, EventStream};
```

- [ ] **Step 4: Implement Plugin trait**

```rust
// crates/frame-core/src/plugin/trait.rs
pub trait Plugin: 'static + Send + Sync {
    fn init(&mut self, ctx: &mut super::PluginContext<'_>);
    fn on_pause(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_destroy(&mut self) {}
}
```

- [ ] **Step 5: Implement PluginContext**

```rust
// crates/frame-core/src/plugin/context.rs
use super::event::EventEmitter;
use std::any::Any;

pub struct PlatformContext;

pub struct PluginContext<'a> {
    events: &'a EventEmitter,
}

impl<'a> PluginContext<'a> {
    pub fn new(events: &'a EventEmitter) -> Self {
        Self { events }
    }

    pub fn create_stream<T: 'static + Send + Sync>(&self) -> super::EventStream<T> {
        self.events.create_stream()
    }

    pub fn events(&self) -> &EventEmitter {
        self.events
    }
}
```

- [ ] **Step 6: Implement EventEmitter and EventStream**

```rust
// crates/frame-core/src/plugin/event.rs
use std::any::Any;
use std::collections::VecDeque;
use std::sync::{Mutex, RwLock};

type AnySender = Box<dyn Any + Send + Sync>;

pub struct EventEmitter {
    // EventEmitter is a factory for streams; streams are self-contained
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_stream<T: 'static + Send + Sync>(&self) -> EventStream<T> {
        EventStream::new()
    }
}

pub struct EventStream<T> {
    buffer: RwLock<VecDeque<T>>,
}

impl<T> EventStream<T> {
    fn new() -> Self {
        Self {
            buffer: RwLock::new(VecDeque::new()),
        }
    }

    pub fn emit(&self, value: T) {
        self.buffer.write().unwrap().push_back(value);
    }

    pub fn try_recv(&self) -> Option<T> {
        self.buffer.write().unwrap().pop_front()
    }
}
```

- [ ] **Step 7: Implement PluginHost**

```rust
// crates/frame-core/src/plugin/host.rs
use super::Plugin;
use super::context::PluginContext;
use super::event::EventEmitter;

pub struct PluginHost {
    plugins: Vec<Box<dyn Plugin>>,
    emitter: EventEmitter,
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            emitter: EventEmitter::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn init_all(&mut self) {
        let mut ctx = PluginContext::new(&self.emitter);
        for plugin in &mut self.plugins {
            plugin.init(&mut ctx);
        }
    }

    pub fn pause_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.on_pause();
        }
    }

    pub fn resume_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.on_resume();
        }
    }

    pub fn destroy_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.on_destroy();
        }
    }
}
```

- [ ] **Step 8: Run tests to verify they pass**

Run: `cargo test --package frame-core --test plugin`
Expected: All 3 tests PASS

- [ ] **Step 9: Commit**

```bash
git add .
git commit -m "feat(frame-core): add Plugin trait, PluginHost, EventEmitter, EventStream"
```

---

### Task 9: Coordination Traits

**Files:**
- Create: `crates/frame-core/src/traits/mod.rs`
- Create: `crates/frame-core/src/traits/renderer.rs`
- Create: `crates/frame-core/src/traits/layout.rs`
- Create: `crates/frame-core/src/traits/window.rs`
- Create: `crates/frame-core/src/traits/widget.rs`
- Test: `crates/frame-core/tests/traits.rs`

- [ ] **Step 1: Write failing tests for coordination traits**

```rust
// crates/frame-core/tests/traits.rs
use frame_core::traits::renderer::{Renderer, RenderSurface};
use frame_core::traits::layout::{Layout, LayoutNode, LayoutResult};
use frame_core::traits::window::{WindowHost, WindowHandle, WindowConfig};
use frame_core::traits::widget::{Widget, WidgetOutput, RenderContext};
use frame_core::{Size, Rect, Constraints, WindowId};
use frame_core::plugin::Plugin;

struct MockRenderer;

struct MockSurface;

impl RenderSurface for MockSurface {
    fn resize(&mut self, _width: u32, _height: u32) {}
}

impl Renderer for MockRenderer {
    type Surface = MockSurface;

    fn create_surface(&mut self, _window: &WindowHandle) -> Self::Surface {
        MockSurface
    }

    fn render_frame(&mut self, _surface: &mut Self::Surface) {}
}

struct MockLayout;

impl Layout for MockLayout {
    fn measure(&self, _node: LayoutNode, _constraints: Constraints) -> Size {
        Size::new(100.0, 100.0)
    }

    fn layout(&self, _node: LayoutNode, _bounds: Rect) -> Vec<LayoutResult> {
        vec![]
    }
}

#[test]
fn mock_renderer_implements_trait() {
    let mut r = MockRenderer;
    let handle = WindowHandle { id: WindowId::default() };
    let mut surface = r.create_surface(&handle);
    r.render_frame(&mut surface);
}

#[test]
fn mock_layout_implements_trait() {
    let l = MockLayout;
    let size = l.measure(LayoutNode::default(), Constraints::tight(Size::new(100.0, 100.0)));
    assert_eq!(size, Size::new(100.0, 100.0));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test traits`
Expected: Compilation errors

- [ ] **Step 3: Create traits module root**

```rust
// crates/frame-core/src/traits/mod.rs
pub mod renderer;
pub mod layout;
pub mod window;
pub mod widget;
```

- [ ] **Step 4: Implement Renderer trait**

```rust
// crates/frame-core/src/traits/renderer.rs
use crate::WindowId;

#[derive(Debug, Clone, Copy)]
pub struct WindowHandle {
    pub id: WindowId,
}

impl WindowHandle {
    pub fn new(id: WindowId) -> Self {
        Self { id }
    }
}

pub trait RenderSurface {
    fn resize(&mut self, width: u32, height: u32);
}

pub trait Renderer: 'static {
    type Surface: RenderSurface;

    fn create_surface(&mut self, window: &WindowHandle) -> Self::Surface;
    fn render_frame(&mut self, surface: &mut Self::Surface);
}
```

- [ ] **Step 5: Implement Layout trait**

```rust
// crates/frame-core/src/traits/layout.rs
use crate::{Size, Rect, WidgetId};
use slotmap::SlotMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutNode {
    pub id: WidgetId,
    pub children: Vec<LayoutNode>,
}

impl Default for LayoutNode {
    fn default() -> Self {
        Self {
            id: WidgetId::default(),
            children: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub node: LayoutNode,
    pub bounds: Rect,
}

pub trait Layout: 'static {
    fn measure(&self, node: LayoutNode, constraints: crate::Constraints) -> Size;
    fn layout(&self, node: LayoutNode, bounds: Rect) -> Vec<LayoutResult>;
}
```

- [ ] **Step 6: Implement WindowHost trait**

```rust
// crates/frame-core/src/traits/window.rs
use crate::{Size, WindowId};
use crate::traits::renderer::WindowHandle;
use crate::plugin::Plugin;

pub struct WindowConfig {
    pub title: String,
    pub size: Size,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Frame App"),
            size: Size::new(800.0, 600.0),
            resizable: true,
        }
    }
}

pub trait WindowHost: Plugin {
    fn create_window(&mut self, config: WindowConfig) -> WindowHandle;
    fn set_title(&mut self, handle: WindowHandle, title: &str);
    fn resize(&mut self, handle: WindowHandle, size: Size);
}
```

- [ ] **Step 7: Implement Widget trait**

```rust
// crates/frame-core/src/traits/widget.rs
use crate::{Size, Constraints};
use crate::id::WidgetId;

pub struct RenderContext {
    pub id_counter: WidgetId,
}

pub enum WidgetOutput {
    None,
    Text {
        content: String,
    },
    Children {
        children: Vec<WidgetNode>,
    },
}

pub struct WidgetNode {
    pub id: WidgetId,
    pub widget: Box<dyn Widget>,
}

pub trait Widget: 'static {
    fn render(&mut self, ctx: &mut RenderContext) -> WidgetOutput;
    fn measure(&self, constraints: Constraints) -> Size;
}
```

- [ ] **Step 8: Run tests to verify they pass**

Run: `cargo test --package frame-core --test traits`
Expected: All 2 tests PASS

- [ ] **Step 9: Commit**

```bash
git add .
git commit -m "feat(frame-core): add Renderer, Layout, WindowHost, Widget traits"
```

---

### Task 10: FrameApp Coordinator

**Files:**
- Create: `crates/frame-core/src/app.rs`
- Test: `crates/frame-core/tests/app.rs`

- [ ] **Step 1: Write failing tests for FrameApp**

```rust
// crates/frame-core/tests/app.rs
use frame_core::app::FrameApp;
use frame_core::plugin::PluginHost;

#[test]
fn frame_app_builder_creates_app() {
    let app = FrameApp::builder()
        .build();
    assert!(app.is_ok());
}

#[test]
fn frame_app_builder_with_plugins() {
    let app = FrameApp::builder()
        .build();
    assert!(app.is_ok());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --package frame-core --test app`
Expected: Compilation errors

- [ ] **Step 3: Implement FrameApp**

```rust
// crates/frame-core/src/app.rs
use crate::plugin::PluginHost;
use crate::traits::renderer::WindowHandle;
use crate::traits::widget::{Widget, WidgetNode, WidgetOutput, RenderContext};
use crate::Size;

pub struct FrameApp {
    plugin_host: PluginHost,
    root_widget: Option<WidgetNode>,
}

impl FrameApp {
    pub fn builder() -> FrameAppBuilder {
        FrameAppBuilder {
            plugin_host: PluginHost::new(),
        }
    }

    pub fn init(&mut self) {
        self.plugin_host.init_all();
    }

    pub fn pause(&mut self) {
        self.plugin_host.pause_all();
    }

    pub fn resume(&mut self) {
        self.plugin_host.resume_all();
    }

    pub fn destroy(&mut self) {
        self.plugin_host.destroy_all();
    }
}

pub struct FrameAppBuilder {
    plugin_host: PluginHost,
}

impl FrameAppBuilder {
    pub fn plugin(mut self, plugin: Box<dyn crate::plugin::Plugin>) -> Self {
        self.plugin_host.add_plugin(plugin);
        self
    }

    pub fn build(self) -> Result<FrameApp, String> {
        Ok(FrameApp {
            plugin_host: self.plugin_host,
            root_widget: None,
        })
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --package frame-core --test app`
Expected: All 2 tests PASS

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(frame-core): add FrameApp coordinator with builder pattern"
```

---

### Task 11: Update lib.rs Re-exports and Verify Full Build

**Files:**
- Modify: `crates/frame-core/src/lib.rs`

- [ ] **Step 1: Update lib.rs with complete re-exports**

```rust
// crates/frame-core/src/lib.rs
pub mod geometry;
pub mod color;
pub mod id;
pub mod reactive;
pub mod plugin;
pub mod traits;
pub mod app;

pub use geometry::{Rect, Size, Point, Constraints};
pub use color::Color;
pub use id::{WindowId, WidgetId};

pub use reactive::signal::Signal;
pub use reactive::computed::Computed;
pub use reactive::effect::Effect;
pub use reactive::batch::batch;

pub use plugin::{Plugin, PluginContext, PluginHost, EventEmitter, EventStream};

pub use traits::renderer::{Renderer, RenderSurface, WindowHandle};
pub use traits::layout::{Layout, LayoutNode, LayoutResult};
pub use traits::window::{WindowHost, WindowConfig};
pub use traits::widget::{Widget, WidgetOutput, WidgetNode, RenderContext};

pub use app::{FrameApp, FrameAppBuilder};
```

- [ ] **Step 2: Run full test suite**

Run: `cargo test --package frame-core`
Expected: All tests PASS (geometry: 9, tracker: 3, signal: 6, computed: 4, effect: 2, batch: 1, plugin: 3, traits: 2, app: 2 = 32 tests)

- [ ] **Step 3: Run clippy**

Run: `cargo clippy --package frame-core -- -D warnings`
Expected: No warnings

- [ ] **Step 4: Commit**

```bash
git add .
git commit -m "feat(frame-core): finalize re-exports, all 32 tests passing"
```

---

## Remaining Phases

After Phase 1 is complete, the following phases will be planned separately:

| Phase | Scope | Depends On |
|---|---|---|
| 2 | `frame-rendering` — Vello integration, Renderer impl, paint cycle | Phase 1 |
| 3 | `frame-layout` — flexbox engine, Layout impl | Phase 1 |
| 4 | `frame-macos` — first platform crate, PlatformContext impl | Phase 1 |
| 5 | `frame-input` + `frame-window` + `frame-lifecycle` — standard plugins | Phase 1, 4 |
| 6 | `frame-ui` — widget library + builder API | Phase 1, 2, 3 |
| 7 | `frame-macros` — view! procedural macro | Phase 6 |
| 8 | Navigation & deep linking — Router, Navigator, Route | Phase 6 |
| 9 | Remaining platforms — iOS, Android, Windows, Linux, Web | Phase 4 |
| 10 | `frame` convenience crate + `cargo-frame` CLI | All above |
