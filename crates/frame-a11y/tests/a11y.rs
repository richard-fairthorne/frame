use frame_a11y::*;

#[test]
fn role_properties() {
    assert!(AccessibilityRole::Button.is_interactive());
    assert!(!AccessibilityRole::Text.is_interactive());
    assert!(AccessibilityRole::List.is_container());
    assert!(AccessibilityRole::Navigation.is_landmark());
}

#[test]
fn properties_builder() {
    let props = AccessibilityProperties::new(AccessibilityRole::Button)
        .label("Submit")
        .hint("Press to submit form")
        .enabled(true);

    assert_eq!(props.label.as_deref(), Some("Submit"));
    assert_eq!(props.hint.as_deref(), Some("Press to submit form"));
    assert!(props.enabled);
}

#[test]
fn properties_state_description() {
    let props = AccessibilityProperties::new(AccessibilityRole::Checkbox)
        .checked(true)
        .focused(true);
    let state = props.state_description();
    assert!(state.contains("checked"));
    assert!(state.contains("focused"));
}

#[test]
fn tree_add_nodes() {
    use frame_core::{Rect, Point, Size};

    let mut tree = AccessibilityTree::new();
    let root = tree.add_node(
        AccessibilityProperties::new(AccessibilityRole::Main).label("Main content"),
        Rect::new(Point::ZERO, Size::new(800.0, 600.0)),
        None,
    );

    let _button = tree.add_node(
        AccessibilityProperties::new(AccessibilityRole::Button).label("Click me"),
        Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 40.0)),
        Some(root),
    );

    assert_eq!(tree.node_count(), 2);
    assert_eq!(tree.interactive_count(), 1);
    assert_eq!(tree.children_of(root).len(), 1);
}

#[test]
fn tree_remove_node() {
    use frame_core::{Rect, Point, Size};

    let mut tree = AccessibilityTree::new();
    let root = tree.add_node(AccessibilityProperties::new(AccessibilityRole::Main), Rect::new(Point::ZERO, Size::new(800.0, 600.0)), None);
    let child = tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), Some(root));

    tree.remove_node(child);
    assert_eq!(tree.node_count(), 1);
    assert_eq!(tree.children_of(root).len(), 0);
}

#[test]
fn tree_find_by_label() {
    use frame_core::{Rect, Point, Size};

    let mut tree = AccessibilityTree::new();
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button).label("Submit"), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), None);
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button).label("Cancel"), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), None);

    let results = tree.find_by_label("Submit");
    assert_eq!(results.len(), 1);
}

#[test]
fn tree_find_by_role() {
    use frame_core::{Rect, Point, Size};

    let mut tree = AccessibilityTree::new();
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button).label("A"), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), None);
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Text).label("B"), Rect::new(Point::ZERO, Size::new(100.0, 20.0)), None);

    let buttons = tree.find_by_role(AccessibilityRole::Button);
    assert_eq!(buttons.len(), 1);
}

#[test]
fn tree_focus_order() {
    use frame_core::{Rect, Point, Size};

    let mut tree = AccessibilityTree::new();
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Text).label("Label"), Rect::new(Point::ZERO, Size::new(100.0, 20.0)), None);
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button).label("A"), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), None);
    tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button).label("B"), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), None);

    assert_eq!(tree.focus_order().len(), 2);
}

#[test]
fn tree_update_bounds() {
    use frame_core::{Rect, Point, Size};

    let mut tree = AccessibilityTree::new();
    let id = tree.add_node(AccessibilityProperties::new(AccessibilityRole::Button), Rect::new(Point::ZERO, Size::new(100.0, 40.0)), None);

    tree.update_bounds(id, Rect::new(Point::new(10.0, 10.0), Size::new(200.0, 50.0)));
    let node = tree.get(id).unwrap();
    assert_eq!(node.bounds.origin.x, 10.0);
}

#[test]
fn announcer_basic() {
    let announcer = AccessibilityAnnouncer::new();
    announcer.announce("Hello");
    announcer.announce_with_priority("Alert!", AnnouncementPriority::Assertive);

    assert!(announcer.has_pending());
    let announcements = announcer.drain();
    assert_eq!(announcements.len(), 2);
    assert!(!announcer.has_pending());
}

#[test]
fn announcer_clone_shared() {
    let announcer = AccessibilityAnnouncer::new();
    let cloned = announcer.clone();
    cloned.announce("From clone");
    assert!(announcer.has_pending());
}
