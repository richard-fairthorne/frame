use frame_core::{Constraints, Size};
use frame_layout::*;

#[test]
fn empty_node_takes_available_space() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::column());
    let constraints = Constraints::loose(Size::new(800.0, 600.0));
    let (size, children) = engine.layout_node(&node, constraints);
    assert_eq!(children.len(), 0);
    assert!(size.width <= 800.0);
    assert!(size.height <= 600.0);
}

#[test]
fn column_layout_stacks_vertically() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::column())
        .child(FlexNode::new(FlexStyle::default()).size(100.0, 50.0))
        .child(FlexNode::new(FlexStyle::default()).size(100.0, 50.0));
    let constraints = Constraints::loose(Size::new(800.0, 600.0));
    let (_size, children) = engine.layout_node(&node, constraints);
    assert_eq!(children.len(), 2);
    assert!(children[1].1.origin.y > children[0].1.origin.y);
}

#[test]
fn row_layout_stacks_horizontally() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row())
        .child(FlexNode::new(FlexStyle::default()).size(100.0, 50.0))
        .child(FlexNode::new(FlexStyle::default()).size(100.0, 50.0));
    let constraints = Constraints::loose(Size::new(800.0, 600.0));
    let (_size, children) = engine.layout_node(&node, constraints);
    assert_eq!(children.len(), 2);
    assert!(children[1].1.origin.x > children[0].1.origin.x);
}

#[test]
fn gap_adds_spacing() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::column().gap(10.0))
        .child(FlexNode::new(FlexStyle::default()).size(100.0, 50.0))
        .child(FlexNode::new(FlexStyle::default()).size(100.0, 50.0));
    let constraints = Constraints::loose(Size::new(800.0, 600.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert_eq!(children[1].1.origin.y, 60.0);
}

#[test]
fn flex_engine_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlexEngine>();
}

#[test]
fn row_layout_basic_with_leaves() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (size, children) = engine.layout_node(&node, constraints);
    assert_eq!(size, Size::new(200.0, 100.0));
    assert_eq!(children[0].1.origin.x, 0.0);
    assert_eq!(children[1].1.origin.x, 50.0);
}

#[test]
fn column_layout_basic_with_leaves() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::column().gap(0.0))
        .child(FlexNode::leaf(Size::new(80.0, 30.0)))
        .child(FlexNode::leaf(Size::new(80.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert_eq!(children[0].1.origin.y, 0.0);
    assert_eq!(children[1].1.origin.y, 30.0);
}

#[test]
fn flex_grow_distributes_space() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(
            FlexNode::new(FlexStyle::default().flex_grow(1.0))
                .child_leaf(Size::new(0.0, 30.0)),
        )
        .child(
            FlexNode::new(FlexStyle::default().flex_grow(1.0))
                .child_leaf(Size::new(0.0, 30.0)),
        );
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let free_space = 200.0_f32;
    assert!((children[0].1.size.width - free_space / 2.0).abs() < 0.01);
    assert!((children[1].1.size.width - free_space / 2.0).abs() < 0.01);
}

#[test]
fn justify_content_center() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().justify_content(JustifyContent::Center))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let expected_x = (200.0 - 50.0) / 2.0;
    assert!((children[0].1.origin.x - expected_x).abs() < 0.01);
}

#[test]
fn justify_content_end() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().justify_content(JustifyContent::End).gap(0.0))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.x - 150.0).abs() < 0.01);
}

#[test]
fn justify_content_space_between() {
    let engine = FlexEngine::new();
    let node =
        FlexNode::new(FlexStyle::row().justify_content(JustifyContent::SpaceBetween).gap(0.0))
            .child(FlexNode::leaf(Size::new(50.0, 30.0)))
            .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.x - 0.0).abs() < 0.01);
    assert!((children[1].1.origin.x - 150.0).abs() < 0.01);
}

#[test]
fn justify_content_space_around() {
    let engine = FlexEngine::new();
    let node =
        FlexNode::new(FlexStyle::row().justify_content(JustifyContent::SpaceAround).gap(0.0))
            .child(FlexNode::leaf(Size::new(50.0, 30.0)))
            .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let free = 200.0 - 100.0;
    let spacing = free / 2.0;
    assert!((children[0].1.origin.x - spacing / 2.0).abs() < 0.01);
    assert!((children[1].1.origin.x - (spacing / 2.0 + 50.0 + spacing)).abs() < 0.01);
}

#[test]
fn justify_content_space_evenly() {
    let engine = FlexEngine::new();
    let node =
        FlexNode::new(FlexStyle::row().justify_content(JustifyContent::SpaceEvenly).gap(0.0))
            .child(FlexNode::leaf(Size::new(50.0, 30.0)))
            .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let free = 200.0 - 100.0;
    let spacing = free / 3.0;
    assert!((children[0].1.origin.x - spacing).abs() < 0.01);
    assert!((children[1].1.origin.x - (spacing + 50.0 + spacing)).abs() < 0.01);
}

#[test]
fn align_items_center() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().align_items(AlignItems::Center))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let expected_y = (100.0 - 30.0) / 2.0;
    assert!((children[0].1.origin.y - expected_y).abs() < 0.01);
}

#[test]
fn align_items_end() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().align_items(AlignItems::End))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let expected_y = 100.0 - 30.0;
    assert!((children[0].1.origin.y - expected_y).abs() < 0.01);
}

#[test]
fn align_items_stretch() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().align_items(AlignItems::Stretch).gap(0.0))
        .child(
            FlexNode::new(FlexStyle::default().flex_grow(1.0))
                .child_leaf(Size::new(0.0, 0.0)),
        );
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.size.height - 100.0).abs() < 0.01);
}

#[test]
fn align_self_overrides_parent() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().align_items(AlignItems::Start))
        .child(
            FlexNode::new(FlexStyle::default().align_self(AlignSelf::Center))
                .size(50.0, 30.0),
        );
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let expected_y = (100.0 - 30.0) / 2.0;
    assert!((children[0].1.origin.y - expected_y).abs() < 0.01);
}

#[test]
fn gap_between_items() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(10.0))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.x - 0.0).abs() < 0.01);
    assert!((children[1].1.origin.x - 60.0).abs() < 0.01);
    assert!((children[2].1.origin.x - 120.0).abs() < 0.01);
}

#[test]
fn padding_offsets_children() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().padding(16.0).gap(0.0))
        .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.x - 16.0).abs() < 0.01);
    assert!((children[0].1.origin.y - 16.0).abs() < 0.01);
}

#[test]
fn row_reverse_layout() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(
        FlexStyle::default()
            .direction(FlexDirection::RowReverse)
            .gap(0.0),
    )
    .child(FlexNode::leaf(Size::new(50.0, 30.0)))
    .child(FlexNode::leaf(Size::new(50.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.x - 150.0).abs() < 0.01);
    assert!((children[1].1.origin.x - 100.0).abs() < 0.01);
}

#[test]
fn column_reverse_layout() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(
        FlexStyle::default()
            .direction(FlexDirection::ColumnReverse)
            .gap(0.0),
    )
    .child(FlexNode::leaf(Size::new(80.0, 30.0)))
    .child(FlexNode::leaf(Size::new(80.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.y - 70.0).abs() < 0.01);
    assert!((children[1].1.origin.y - 40.0).abs() < 0.01);
}

#[test]
fn flex_shrink_when_overflowing() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(FlexNode::new(FlexStyle::default().flex_shrink(1.0)).size(150.0, 30.0))
        .child(FlexNode::new(FlexStyle::default().flex_shrink(1.0)).size(150.0, 30.0));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let overflow = 300.0 - 200.0;
    let shrink_per_item = overflow / 2.0;
    assert!(
        (children[0].1.size.width - (150.0 - shrink_per_item)).abs() < 0.01,
        "expected {}, got {}",
        150.0 - shrink_per_item,
        children[0].1.size.width
    );
    assert!(
        (children[1].1.size.width - (150.0 - shrink_per_item)).abs() < 0.01,
        "expected {}, got {}",
        150.0 - shrink_per_item,
        children[1].1.size.width
    );
}

#[test]
fn flex_basis_overrides_explicit_size() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(FlexNode::new(FlexStyle::default().flex_basis(80.0)).size(50.0, 30.0));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!(
        (children[0].1.size.width - 80.0).abs() < 0.01,
        "expected 80.0, got {}",
        children[0].1.size.width
    );
}

#[test]
fn flex_wrap_splits_into_lines() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().wrap(FlexWrap::Wrap).gap(0.0))
        .child(FlexNode::leaf(Size::new(120.0, 30.0)))
        .child(FlexNode::leaf(Size::new(120.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert_eq!(children.len(), 2);
    assert!(
        children[1].1.origin.y > children[0].1.origin.y,
        "second item should be on a new line"
    );
}

#[test]
fn flex_wrap_no_wrap_keeps_single_line() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().wrap(FlexWrap::NoWrap).gap(0.0))
        .child(FlexNode::leaf(Size::new(120.0, 30.0)))
        .child(FlexNode::leaf(Size::new(120.0, 30.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert_eq!(children.len(), 2);
    assert!(
        (children[0].1.origin.y - children[1].1.origin.y).abs() < 0.01,
        "items should be on the same line"
    );
}

#[test]
fn margin_offsets_item() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(FlexNode::new(FlexStyle::default().margin(10.0)).size(50.0, 30.0));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.origin.x - 10.0).abs() < 0.01);
    assert!((children[0].1.origin.y - 10.0).abs() < 0.01);
}

#[test]
fn nested_flex_layout() {
    let inner = FlexNode::new(FlexStyle::row())
        .child(FlexNode::leaf(Size::new(20.0, 20.0)))
        .child(FlexNode::leaf(Size::new(20.0, 20.0)));

    let outer = FlexNode::new(FlexStyle::column())
        .child(FlexNode::leaf(Size::new(80.0, 20.0)))
        .child(inner);

    let engine = FlexEngine::new();
    let (_, children) = engine.layout_node(&outer, Constraints::tight(Size::new(200.0, 100.0)));
    assert_eq!(children.len(), 2);
    assert!((children[0].1.size.width - 80.0).abs() < 0.01);
}

#[test]
fn min_size_constraint() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(FlexNode::new(FlexStyle::default().flex_shrink(1.0)).size(100.0, 30.0).min_size(60.0, 0.0))
        .child(FlexNode::new(FlexStyle::default().flex_shrink(1.0)).size(100.0, 30.0));
    let constraints = Constraints::tight(Size::new(150.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!(
        children[0].1.size.width >= 60.0,
        "min_size should be respected: got {}",
        children[0].1.size.width
    );
}

#[test]
fn mixed_flex_grow_proportions() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::row().gap(0.0))
        .child(FlexNode::new(FlexStyle::default().flex_grow(1.0)).child_leaf(Size::new(0.0, 30.0)))
        .child(FlexNode::new(FlexStyle::default().flex_grow(2.0)).child_leaf(Size::new(0.0, 30.0)))
        .child(FlexNode::new(FlexStyle::default().flex_grow(1.0)).child_leaf(Size::new(0.0, 30.0)));
    let constraints = Constraints::tight(Size::new(400.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    assert!((children[0].1.size.width - 100.0).abs() < 0.01);
    assert!((children[1].1.size.width - 200.0).abs() < 0.01);
    assert!((children[2].1.size.width - 100.0).abs() < 0.01);
}

#[test]
fn column_with_justify_center() {
    let engine = FlexEngine::new();
    let node = FlexNode::new(FlexStyle::column().justify_content(JustifyContent::Center).gap(0.0))
        .child(FlexNode::leaf(Size::new(80.0, 20.0)));
    let constraints = Constraints::tight(Size::new(200.0, 100.0));
    let (_, children) = engine.layout_node(&node, constraints);
    let expected_y = (100.0 - 20.0) / 2.0;
    assert!((children[0].1.origin.y - expected_y).abs() < 0.01);
}
