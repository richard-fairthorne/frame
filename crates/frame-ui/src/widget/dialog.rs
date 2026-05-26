use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogDisposition {
    Confirmed,
    Cancelled,
    Dismissed,
}

pub type DialogCallback = Box<dyn FnOnce(DialogDisposition) + Send + Sync>;

pub struct DialogResult {
    pub disposition: DialogDisposition,
    pub input_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    Alert,
    Confirm,
    Prompt,
    Custom,
}

#[derive(Debug, Clone)]
pub struct DialogConfig {
    pub title: String,
    pub message: String,
    pub dialog_type: DialogType,
    pub confirm_label: String,
    pub cancel_label: String,
    pub placeholder: String,
    pub dismiss_on_backdrop: bool,
    pub backdrop_opacity: f32,
}

impl DialogConfig {
    pub fn alert(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            dialog_type: DialogType::Alert,
            confirm_label: "OK".into(),
            cancel_label: "Cancel".into(),
            placeholder: String::new(),
            dismiss_on_backdrop: true,
            backdrop_opacity: 0.5,
        }
    }

    pub fn confirm(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            dialog_type: DialogType::Confirm,
            confirm_label: "Confirm".into(),
            cancel_label: "Cancel".into(),
            placeholder: String::new(),
            dismiss_on_backdrop: true,
            backdrop_opacity: 0.5,
        }
    }

    pub fn prompt(title: &str, message: &str, placeholder: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            dialog_type: DialogType::Prompt,
            confirm_label: "Submit".into(),
            cancel_label: "Cancel".into(),
            placeholder: placeholder.to_string(),
            dismiss_on_backdrop: false,
            backdrop_opacity: 0.5,
        }
    }

    pub fn confirm_label(mut self, label: &str) -> Self {
        self.confirm_label = label.into();
        self
    }
    pub fn cancel_label(mut self, label: &str) -> Self {
        self.cancel_label = label.into();
        self
    }
    pub fn dismiss_on_backdrop(mut self, dismiss: bool) -> Self {
        self.dismiss_on_backdrop = dismiss;
        self
    }
    pub fn backdrop_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_opacity = opacity;
        self
    }
}

impl Default for DialogConfig {
    fn default() -> Self {
        Self::alert("", "")
    }
}

pub struct Dialog {
    config: DialogConfig,
    visible: bool,
    input_value: String,
    on_result: Option<Arc<Mutex<Option<DialogCallback>>>>,
}

impl Dialog {
    pub fn new(config: DialogConfig) -> Self {
        Self {
            config,
            visible: false,
            input_value: String::new(),
            on_result: None,
        }
    }

    pub fn on_result(
        mut self,
        callback: impl FnOnce(DialogDisposition) + Send + Sync + 'static,
    ) -> Self {
        self.on_result = Some(Arc::new(Mutex::new(Some(Box::new(callback)))));
        self
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.input_value.clear();
    }

    pub fn dismiss(&mut self, disposition: DialogDisposition) {
        self.visible = false;
        if let Some(ref cb_arc) = self.on_result {
            if let Ok(mut guard) = cb_arc.lock() {
                if let Some(cb) = guard.take() {
                    cb(disposition);
                }
            }
        }
    }

    pub fn confirm(&mut self) {
        self.dismiss(DialogDisposition::Confirmed);
    }

    pub fn cancel(&mut self) {
        self.dismiss(DialogDisposition::Cancelled);
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn title(&self) -> &str {
        &self.config.title
    }
    pub fn message(&self) -> &str {
        &self.config.message
    }
    pub fn dialog_type(&self) -> DialogType {
        self.config.dialog_type
    }
    pub fn input_value(&self) -> &str {
        &self.input_value
    }
    pub fn confirm_label(&self) -> &str {
        &self.config.confirm_label
    }
    pub fn cancel_label(&self) -> &str {
        &self.config.cancel_label
    }
    pub fn backdrop_opacity(&self) -> f32 {
        self.config.backdrop_opacity
    }
    pub fn dismiss_on_backdrop(&self) -> bool {
        self.config.dismiss_on_backdrop
    }

    pub fn set_input(&mut self, value: &str) {
        self.input_value = value.to_string();
    }
}

pub struct DialogHost {
    dialogs: Vec<Dialog>,
}

impl DialogHost {
    pub fn new() -> Self {
        Self {
            dialogs: Vec::new(),
        }
    }

    pub fn show(&mut self, dialog: Dialog) {
        let mut dialog = dialog;
        dialog.show();
        self.dialogs.push(dialog);
    }

    pub fn dismiss_top(&mut self, disposition: DialogDisposition) {
        if let Some(mut dialog) = self.dialogs.pop() {
            dialog.visible = false;
            if let Some(ref cb_arc) = dialog.on_result {
                if let Ok(mut guard) = cb_arc.lock() {
                    if let Some(cb) = guard.take() {
                        cb(disposition);
                    }
                }
            }
        }
    }

    pub fn top(&self) -> Option<&Dialog> {
        self.dialogs.last()
    }
    pub fn top_mut(&mut self) -> Option<&mut Dialog> {
        self.dialogs.last_mut()
    }
    pub fn is_showing(&self) -> bool {
        !self.dialogs.is_empty()
    }
    pub fn count(&self) -> usize {
        self.dialogs.len()
    }
    pub fn dismiss_all(&mut self) {
        while !self.dialogs.is_empty() {
            self.dismiss_top(DialogDisposition::Dismissed);
        }
    }
}

impl Default for DialogHost {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Overlay {
    visible: bool,
    opacity: f32,
    dismiss_on_tap: bool,
    on_dismiss: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            visible: false,
            opacity: 0.5,
            dismiss_on_tap: true,
            on_dismiss: None,
        }
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
    pub fn with_dismiss_on_tap(mut self, dismiss: bool) -> Self {
        self.dismiss_on_tap = dismiss;
        self
    }

    pub fn on_dismiss(mut self, f: impl FnOnce() + Send + Sync + 'static) -> Self {
        self.on_dismiss = Some(Box::new(f));
        self
    }

    pub fn show(&mut self) {
        self.visible = true;
    }
    pub fn dismiss(&mut self) {
        self.visible = false;
        if let Some(cb) = self.on_dismiss.take() {
            cb();
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn opacity(&self) -> f32 {
        self.opacity
    }
    pub fn dismiss_on_tap(&self) -> bool {
        self.dismiss_on_tap
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}
