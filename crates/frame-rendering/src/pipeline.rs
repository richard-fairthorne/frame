use frame_core::traits::widget::{LayoutDirection, RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Point, Rect};
use crate::click;
use crate::scene::SceneBuilder;
use crate::text::{TextShaper, draw_text_layout};

pub struct PaintResult {
    pub scene: vello::Scene,
    pub dirty: bool,
}

pub fn paint_widget_tree(
    widget: &mut dyn Widget,
    constraints: Constraints,
    scale_factor: f32,
) -> PaintResult {
    click::clear_click_targets();

    let size = widget.measure(constraints);

    let mut ctx = RenderContext {
        id_counter: frame_core::WidgetId::default(),
    };
    let output = widget.render(&mut ctx);

    let mut builder = SceneBuilder::new();
    let mut shaper = TextShaper::new();
    paint_output(output, &mut builder, &mut shaper, Rect::new(Point::ZERO, size));

    let mut scene = builder.into_scene();
    if scale_factor != 1.0 {
        let mut scaled = vello::Scene::new();
        scaled.append(&scene, Some(vello::kurbo::Affine::scale(scale_factor as f64)));
        scene = scaled;
    }

    PaintResult {
        scene,
        dirty: true,
    }
}

fn paint_output(
    output: WidgetOutput,
    builder: &mut SceneBuilder,
    shaper: &mut TextShaper,
    bounds: Rect,
) {
    match output {
        WidgetOutput::None => {}
        WidgetOutput::Text { content, font_size, color } => {
            let layout = shaper.shape(&content, font_size, Some(bounds.width()));
            let scene = builder.scene_mut();
            draw_text_layout(
                scene,
                &layout,
                bounds.x() as f64,
                bounds.y() as f64,
                color,
            );
        }
        WidgetOutput::Container { background, border_radius, padding, children, on_click } => {
            if let Some(cb) = on_click {
                click::register_click_target(bounds, cb);
            }
            builder.paint_container(bounds, background, border_radius, padding);
            let inner_bounds = if padding > 0.0 {
                Rect::new(
                    Point::new(bounds.x() + padding, bounds.y() + padding),
                    frame_core::Size::new(
                        (bounds.width() - padding * 2.0).max(0.0),
                        (bounds.height() - padding * 2.0).max(0.0),
                    ),
                )
            } else {
                bounds
            };
            for mut child in children {
                let child_size = child.widget.measure(
                    Constraints::loose(inner_bounds.size),
                );
                let child_bounds = Rect::new(inner_bounds.origin, child_size);
                let mut child_ctx = RenderContext {
                    id_counter: frame_core::WidgetId::default(),
                };
                let child_output = child.widget.render(&mut child_ctx);
                paint_output(child_output, builder, shaper, child_bounds);
            }
        }
        WidgetOutput::Children { children } => {
            let mut cursor_y = bounds.y();
            for mut child in children {
                let child_size = child.widget.measure(
                    Constraints::loose(bounds.size),
                );
                let child_bounds = Rect::new(
                    Point::new(bounds.x(), cursor_y),
                    child_size,
                );
                let mut child_ctx = RenderContext {
                    id_counter: frame_core::WidgetId::default(),
                };
                let child_output = child.widget.render(&mut child_ctx);
                paint_output(child_output, builder, shaper, child_bounds);
                cursor_y += child_size.height;
            }
        }
        WidgetOutput::Paint { mut paint_fn, rect } => {
            let _ = rect;
            let scene = builder.scene_mut();
            (paint_fn)(scene as &mut dyn std::any::Any);
        }
        WidgetOutput::Gpu { .. } => {
        }
        WidgetOutput::Layout { direction, gap, children } => {
            let is_row = matches!(direction, LayoutDirection::Horizontal);
            let count = children.len();
            let mut cursor = if is_row { bounds.x() } else { bounds.y() };
            for (i, mut child) in children.into_iter().enumerate() {
                let child_size = child.widget.measure(
                    Constraints::loose(bounds.size),
                );
                let child_bounds = if is_row {
                    Rect::new(Point::new(cursor, bounds.y()), child_size)
                } else {
                    Rect::new(Point::new(bounds.x(), cursor), child_size)
                };
                let mut child_ctx = RenderContext {
                    id_counter: frame_core::WidgetId::default(),
                };
                let child_output = child.widget.render(&mut child_ctx);
                paint_output(child_output, builder, shaper, child_bounds);
                let advance = if is_row { child_size.width } else { child_size.height };
                cursor += advance;
                if i + 1 < count {
                    cursor += gap;
                }
            }
        }
    }
}
