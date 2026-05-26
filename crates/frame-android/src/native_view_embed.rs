use frame_core::{NativeViewHandle, NativeViewKind, NativeViewRequest, Rect};
use std::collections::HashMap;

struct ManagedView {
    rect: Rect,
    kind: NativeViewKind,
}

pub struct NativeViewManager {
    next_id: u64,
    views: HashMap<u64, ManagedView>,
}

impl NativeViewManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            views: HashMap::new(),
        }
    }

    pub fn embed(&mut self, request: NativeViewRequest) -> NativeViewHandle {
        let id = self.next_id;
        self.next_id += 1;

        // Android native view embedding requires JNI interop to create Java View
        // objects (android.view.View and subclasses) and add them to the activity's
        // view hierarchy. Since the framework uses NativeActivity (not a standard
        // Android Activity with XML layouts), there is no direct access to a
        // ViewGroup parent.
        //
        // To implement real native views on Android, the following is needed:
        //
        // 1. Obtain the NativeActivity's JNI environment via `ndk::context::AndroidContext`
        // 2. Call getActivity() to get the Activity instance
        // 3. Call getWindow().getDecorView().findViewById(android.R.id.content)
        //    to get the content FrameLayout
        // 4. Create a Java View subclass via JNI (e.g., new WebView(context))
        // 5. Create a FrameLayout.LayoutParams with the requested rect
        // 6. Call ViewGroup.addView() with the layout params
        //
        // For WebView specifically, additional setup is required:
        //   - WebView must be initialized on the UI thread
        //   - A WebChromeClient and WebViewClient should be set
        //   - JavaScript interface bridging may be needed
        //
        // For other native views (MapView, AdView, etc.), the same JNI pattern
        // applies but with the appropriate View subclass constructor.
        //
        // This is kept as a stub that tracks view metadata for now. Real JNI-based
        // view creation should be added when the framework supports a JNI bridge.

        self.views.insert(
            id,
            ManagedView {
                rect: request.rect,
                kind: request.kind,
            },
        );
        NativeViewHandle { id }
    }

    pub fn update_rect(&mut self, handle: &NativeViewHandle, rect: Rect) {
        if let Some(view) = self.views.get_mut(&handle.id) {
            view.rect = rect;
        }
    }

    pub fn remove(&mut self, handle: &NativeViewHandle) {
        self.views.remove(&handle.id);
    }
}

impl Default for NativeViewManager {
    fn default() -> Self {
        Self::new()
    }
}
