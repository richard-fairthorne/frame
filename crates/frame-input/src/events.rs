#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum KeyCode {
    #[default]
    Unknown,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,
    Enter, Escape, Backspace, Tab, Space,
    Left, Right, Up, Down,
    Shift, Control, Alt, Meta,
}

impl KeyCode {
    pub fn from_u32(code: u32) -> Self {
        match code {
            0x00 => KeyCode::A,
            0x01 => KeyCode::S,
            0x02 => KeyCode::D,
            0x03 => KeyCode::F,
            0x04 => KeyCode::H,
            0x05 => KeyCode::G,
            0x06 => KeyCode::Z,
            0x07 => KeyCode::X,
            0x08 => KeyCode::C,
            0x09 => KeyCode::V,
            0x24 => KeyCode::Enter,
            0x35 => KeyCode::Escape,
            0x31 => KeyCode::Space,
            0x7C => KeyCode::Right,
            0x7B => KeyCode::Left,
            0x7D => KeyCode::Down,
            0x7E => KeyCode::Up,
            _ => KeyCode::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: String,
    pub key_code: KeyCode,
    pub pressed: bool,
    pub modifiers: Modifiers,
}

impl KeyEvent {
    pub fn key_down(key_code: KeyCode) -> Self {
        Self {
            key: String::new(),
            key_code,
            pressed: true,
            modifiers: Modifiers::default(),
        }
    }

    pub fn key_up(key_code: KeyCode) -> Self {
        Self {
            key: String::new(),
            key_code,
            pressed: false,
            modifiers: Modifiers::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub x: f32,
    pub y: f32,
    pub phase: TouchPhase,
    pub id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub button: MouseButton,
    pub pressed: bool,
}

impl MouseEvent {
    pub fn pressed(button: MouseButton, x: f32, y: f32) -> Self {
        Self { x, y, button, pressed: true }
    }

    pub fn released(button: MouseButton, x: f32, y: f32) -> Self {
        Self { x, y, button, pressed: false }
    }

    pub fn moved(x: f32, y: f32) -> Self {
        Self { x, y, button: MouseButton::None, pressed: false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    None,
    Left,
    Right,
    Middle,
}

impl MouseButton {
    pub fn from_u8(button: u8) -> Self {
        match button {
            0 => MouseButton::Left,
            1 => MouseButton::Right,
            2 => MouseButton::Middle,
            _ => MouseButton::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Key(KeyEvent),
    Touch(TouchEvent),
    Mouse(MouseEvent),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}
