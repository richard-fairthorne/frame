use frame_core::traits::renderer::{Renderer, RenderSurface, WindowHandle};
use frame_core::traits::layout::{Layout, LayoutNode, LayoutResult};
use frame_core::traits::window::{WindowHost, WindowConfig};
use frame_core::traits::widget::{Widget, WidgetOutput, RenderContext};
use frame_core::{Size, Rect, Constraints, WindowId};
use frame_core::plugin::Plugin;

struct MockSurface;

impl RenderSurface for MockSurface {
    fn resize(&mut self, _width: u32, _height: u32) {}
}

struct MockRenderer;

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
