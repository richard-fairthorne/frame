use frame::rsx;
use frame::*;

#[test]
fn rsx_text_widget() {
    let text = rsx! {
        <Text size={16.0}>"Hello"</Text>
    };
    let size = text.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
    assert!(size.height > 0.0);
}

#[test]
fn rsx_column_with_children() {
    let col = rsx! {
        <Column gap={8.0}>
            <Text>"Line 1"</Text>
            <Text>"Line 2"</Text>
        </Column>
    };
    let size = col.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.height > 0.0);
}

#[test]
fn rsx_self_closing() {
    let text = rsx! {
        <Text text={"Hello"} size={16.0} />
    };
    let size = text.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0);
}

#[test]
fn rsx_nested_layout() {
    let layout = rsx! {
        <Column gap={12.0}>
            <Container padding={8.0} background={Color::from_u8(240, 240, 240, 255)}>
                <Text size={20.0}>"Header"</Text>
            </Container>
            <Row gap={8.0}>
                <Text>"Left"</Text>
                <Text>"Right"</Text>
            </Row>
        </Column>
    };
    let size = layout.measure(Constraints::loose(Size::new(800.0, 600.0)));
    assert!(size.width > 0.0 && size.height > 0.0);
}

#[test]
fn rsx_button_with_callback() {
    let _btn = rsx! {
        <Button label="Click me" on_click={|| {}} padding={8.0} />
    };
}
