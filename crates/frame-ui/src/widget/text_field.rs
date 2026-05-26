use frame_core::traits::widget::{RenderContext, Widget, WidgetOutput};
use frame_core::{Constraints, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Idle,
    Focused,
    Editing,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct TextFieldStyle {
    pub font_size: f32,
    pub padding: f32,
    pub border_width: f32,
    pub border_radius: f32,
    pub cursor_width: f32,
}

impl Default for TextFieldStyle {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            padding: 8.0,
            border_width: 1.0,
            border_radius: 4.0,
            cursor_width: 2.0,
        }
    }
}

type ChangeCallback = Box<dyn FnMut(&str) + Send + Sync>;
type SubmitCallback = Box<dyn FnMut(&str) + Send + Sync>;

pub struct TextField {
    value: String,
    placeholder: String,
    state: InputState,
    cursor_position: usize,
    max_length: Option<usize>,
    on_change: Option<ChangeCallback>,
    on_submit: Option<SubmitCallback>,
    style: TextFieldStyle,
}

impl TextField {
    pub fn new(placeholder: &str) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.to_string(),
            state: InputState::Idle,
            cursor_position: 0,
            max_length: None,
            on_change: None,
            on_submit: None,
            style: TextFieldStyle::default(),
        }
    }

    pub fn value(mut self, val: &str) -> Self {
        self.value = val.to_string();
        self.cursor_position = val.len();
        self
    }

    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.style.font_size = size;
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.style.padding = p;
        self
    }

    pub fn border_radius(mut self, r: f32) -> Self {
        self.style.border_radius = r;
        self
    }

    pub fn on_change(mut self, f: impl FnMut(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn on_submit(mut self, f: impl FnMut(&str) + Send + Sync + 'static) -> Self {
        self.on_submit = Some(Box::new(f));
        self
    }

    pub fn text(&self) -> &str {
        &self.value
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn state(&self) -> InputState {
        self.state
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn set_text(&mut self, text: &str) {
        self.value = text.to_string();
        self.cursor_position = self.value.len();
    }

    pub fn insert_char(&mut self, ch: char) {
        if let Some(max) = self.max_length {
            if self.value.len() >= max {
                return;
            }
        }
        self.value.insert(self.cursor_position, ch);
        self.cursor_position += 1;
        if let Some(ref mut cb) = self.on_change {
            cb(&self.value);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.value.remove(self.cursor_position);
            if let Some(ref mut cb) = self.on_change {
                cb(&self.value);
            }
        }
    }

    pub fn delete(&mut self) {
        if self.cursor_position < self.value.len() {
            self.value.remove(self.cursor_position);
            if let Some(ref mut cb) = self.on_change {
                cb(&self.value);
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.value.len();
    }

    pub fn focus(&mut self) {
        self.state = InputState::Focused;
    }

    pub fn blur(&mut self) {
        self.state = InputState::Idle;
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        if disabled {
            self.state = InputState::Disabled;
        }
    }

    pub fn is_disabled(&self) -> bool {
        self.state == InputState::Disabled
    }

    pub fn submit(&mut self) {
        if let Some(ref mut cb) = self.on_submit {
            cb(&self.value);
        }
    }
}

impl Widget for TextField {
    fn measure(&self, constraints: Constraints) -> Size {
        let char_width = self.style.font_size * 0.6;
        let min_width = 200.0_f32.max(self.style.padding * 2.0 + char_width * 10.0);
        let height = self.style.padding * 2.0 + self.style.font_size * 1.2;
        let width = constraints.constrain(Size::new(min_width, height)).width;
        Size::new(width, height)
    }

    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::None
    }
}
