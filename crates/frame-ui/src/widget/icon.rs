use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Color, Constraints, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Check,
    Close,
    Plus,
    Minus,
    Search,
    Settings,
    Star,
    Heart,
    Menu,
    Home,
    Back,
    Forward,
}

impl IconKind {
    pub fn symbol(self) -> &'static str {
        match self {
            IconKind::ArrowLeft => "←",
            IconKind::ArrowRight => "→",
            IconKind::ArrowUp => "↑",
            IconKind::ArrowDown => "↓",
            IconKind::Check => "✓",
            IconKind::Close => "✕",
            IconKind::Plus => "+",
            IconKind::Minus => "−",
            IconKind::Search => "⌕",
            IconKind::Settings => "⚙",
            IconKind::Star => "★",
            IconKind::Heart => "♥",
            IconKind::Menu => "☰",
            IconKind::Home => "⌂",
            IconKind::Back => "◄",
            IconKind::Forward => "►",
        }
    }
}

pub struct Icon {
    kind: IconKind,
    icon_size: f32,
    color: Color,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Icon {
    pub fn new(kind: IconKind) -> Self {
        Self {
            kind,
            icon_size: 24.0,
            color: Color::BLACK,
            on_click: None,
        }
    }

    pub fn size(mut self, s: f32) -> Self {
        self.icon_size = s;
        self
    }

    pub fn color(mut self, c: Color) -> Self {
        self.color = c;
        self
    }

    pub fn on_click(mut self, cb: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(cb));
        self
    }

    pub fn kind(&self) -> IconKind {
        self.kind
    }

    pub fn icon_size(&self) -> f32 {
        self.icon_size
    }

    pub fn icon_color(&self) -> Color {
        self.color
    }
}

impl Widget for Icon {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Text {
            content: self.kind.symbol().to_string(),
            font_size: self.icon_size,
            color: self.color,
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(Size::new(self.icon_size, self.icon_size))
    }
}
