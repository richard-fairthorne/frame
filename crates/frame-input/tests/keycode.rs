use frame_input::KeyCode;

#[test]
fn keycode_from_macos_virtual_key() {
    assert_eq!(KeyCode::from_u32(0x00), KeyCode::A);
    assert_eq!(KeyCode::from_u32(0x24), KeyCode::Enter);
    assert_eq!(KeyCode::from_u32(0x35), KeyCode::Escape);
    assert_eq!(KeyCode::from_u32(0x31), KeyCode::Space);
    assert_eq!(KeyCode::from_u32(0x7C), KeyCode::Right);
    assert_eq!(KeyCode::from_u32(0xFF), KeyCode::Unknown);
}
