use cargo_frame::hot_reload::HotReloadMessage;

#[test]
fn serialize_reload() {
    let msg = HotReloadMessage::Reload;
    assert_eq!(msg.serialize(), b"RELOAD\n");
}

#[test]
fn serialize_deserialize_roundtrip() {
    let messages = vec![
        HotReloadMessage::Reload,
        HotReloadMessage::FullRebuild,
        HotReloadMessage::ThemeChanged,
        HotReloadMessage::Shutdown,
        HotReloadMessage::AssetChanged {
            path: "img.png".into(),
        },
    ];

    for msg in messages {
        let serialized = msg.serialize();
        let deserialized = HotReloadMessage::deserialize(&serialized);
        assert_eq!(deserialized, Some(msg));
    }
}

#[test]
fn deserialize_invalid() {
    assert_eq!(HotReloadMessage::deserialize(b"INVALID"), None);
    assert_eq!(HotReloadMessage::deserialize(b""), None);
}
