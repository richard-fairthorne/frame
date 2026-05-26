use frame_core::traits::widget::Widget;
use frame_core::{Constraints, Point, Size};
use frame_ui::{Column, ScrollView, ScrollDirection, ScrollMetrics, Text};

#[test]
fn scroll_view_basic_measure() {
    let sv = ScrollView::new()
        .direction(ScrollDirection::Vertical)
        .child(Text::new("Content").size(16.0));

    let size = sv.measure(Constraints::tight(Size::new(300.0, 400.0)));
    assert!(size.width <= 300.0);
    assert!(size.height <= 400.0);
}

#[test]
fn scroll_view_scroll_by() {
    let mut sv = ScrollView::new().direction(ScrollDirection::Vertical);

    sv.scroll_metrics = Some(ScrollMetrics::new(
        Size::new(300.0, 1000.0),
        Size::new(300.0, 400.0),
    ));

    sv.scroll_by(Point::new(0.0, 100.0));
    assert_eq!(sv.scroll_offset().y, 100.0);
}

#[test]
fn scroll_view_clamp_offset() {
    let mut sv = ScrollView::new();
    sv.scroll_metrics = Some(ScrollMetrics::new(
        Size::new(300.0, 1000.0),
        Size::new(300.0, 400.0),
    ));

    sv.scroll_to(Point::new(0.0, 50000.0));
    assert_eq!(sv.scroll_offset().y, 600.0);

    sv.scroll_to(Point::new(0.0, -100.0));
    assert_eq!(sv.scroll_offset().y, 0.0);
}

#[test]
fn scroll_view_apply_pan() {
    let mut sv = ScrollView::new();
    sv.scroll_metrics = Some(ScrollMetrics::new(
        Size::new(300.0, 1000.0),
        Size::new(300.0, 400.0),
    ));

    sv.apply_pan(Point::new(0.0, 50.0));
    assert_eq!(sv.scroll_offset().y, 50.0);
}

#[test]
fn scroll_view_inertia() {
    let mut sv = ScrollView::new();
    sv.scroll_metrics = Some(ScrollMetrics::new(
        Size::new(300.0, 1000.0),
        Size::new(300.0, 400.0),
    ));

    sv.set_velocity(Point::new(0.0, 20.0));

    let mut scrolled = true;
    for _ in 0..200 {
        scrolled = sv.tick_inertia();
        if !scrolled {
            break;
        }
    }
    assert!(!scrolled);
}

#[test]
fn scroll_view_horizontal() {
    let sv = ScrollView::new()
        .direction(ScrollDirection::Horizontal)
        .child(Text::new("Wide content").size(16.0));

    let _size = sv.measure(Constraints::tight(Size::new(300.0, 400.0)));
}

#[test]
fn scroll_metrics_can_scroll() {
    let metrics = ScrollMetrics::new(
        Size::new(300.0, 1000.0),
        Size::new(300.0, 400.0),
    );
    assert!(metrics.can_scroll_vertically());
    assert!(!metrics.can_scroll_horizontally());
}

#[test]
fn scroll_metrics_fraction() {
    let mut metrics = ScrollMetrics::new(
        Size::new(300.0, 1000.0),
        Size::new(300.0, 400.0),
    );
    assert_eq!(metrics.scroll_fraction_y(), 0.0);
    metrics.set_offset(Point::new(0.0, 300.0));
    assert!((metrics.scroll_fraction_y() - 0.5).abs() < 0.01);
}

#[test]
fn scroll_view_with_column_child() {
    let mut col = Column::new().gap(8.0);
    for i in 0..10 {
        col = col.child(Text::new(format!("Line {}", i)).size(16.0));
    }

    let sv = ScrollView::new()
        .direction(ScrollDirection::Vertical)
        .child(col);

    let size = sv.measure(Constraints::tight(Size::new(300.0, 400.0)));
    assert!(size.width <= 300.0);
}

#[test]
fn scroll_view_no_child_returns_viewport() {
    let sv = ScrollView::new();
    let size = sv.measure(Constraints::tight(Size::new(300.0, 400.0)));
    assert_eq!(size.width, 300.0);
    assert_eq!(size.height, 400.0);
}

#[test]
fn scroll_view_default() {
    let sv = ScrollView::default();
    assert!(sv.scroll_metrics.is_none());
    assert_eq!(sv.scroll_offset(), Point::ZERO);
}

#[test]
fn scroll_view_at_edge_when_no_metrics() {
    let sv = ScrollView::new();
    assert!(sv.is_at_edge());
}

#[test]
fn scroll_view_direction_filters_scroll() {
    let mut sv = ScrollView::new().direction(ScrollDirection::Vertical);
    sv.scroll_metrics = Some(ScrollMetrics::new(
        Size::new(1000.0, 1000.0),
        Size::new(300.0, 400.0),
    ));

    sv.scroll_by(Point::new(50.0, 50.0));
    assert_eq!(sv.scroll_offset().x, 0.0);
    assert_eq!(sv.scroll_offset().y, 50.0);
}
