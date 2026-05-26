#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotReloadMessage {
    Reload,
    FullRebuild,
    AssetChanged { path: String },
    ThemeChanged,
    Shutdown,
}

impl HotReloadMessage {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            HotReloadMessage::Reload => b"RELOAD\n".to_vec(),
            HotReloadMessage::FullRebuild => b"FULL_REBUILD\n".to_vec(),
            HotReloadMessage::AssetChanged { path } => {
                format!("ASSET_CHANGED:{}\n", path).into_bytes()
            }
            HotReloadMessage::ThemeChanged => b"THEME_CHANGED\n".to_vec(),
            HotReloadMessage::Shutdown => b"SHUTDOWN\n".to_vec(),
        }
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        let s = std::str::from_utf8(data).ok()?.trim();
        match s {
            "RELOAD" => Some(HotReloadMessage::Reload),
            "FULL_REBUILD" => Some(HotReloadMessage::FullRebuild),
            "THEME_CHANGED" => Some(HotReloadMessage::ThemeChanged),
            "SHUTDOWN" => Some(HotReloadMessage::Shutdown),
            s if s.starts_with("ASSET_CHANGED:") => Some(HotReloadMessage::AssetChanged {
                path: s[14..].into(),
            }),
            _ => None,
        }
    }
}
