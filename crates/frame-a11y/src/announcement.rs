use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnouncementPriority {
    Low,
    Medium,
    High,
    Assertive,
}

#[derive(Debug, Clone)]
pub struct Announcement {
    pub message: String,
    pub priority: AnnouncementPriority,
}

impl Announcement {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string(), priority: AnnouncementPriority::Medium }
    }

    pub fn with_priority(mut self, priority: AnnouncementPriority) -> Self {
        self.priority = priority;
        self
    }
}

pub struct AccessibilityAnnouncer {
    pending: Arc<Mutex<Vec<Announcement>>>,
}

impl AccessibilityAnnouncer {
    pub fn new() -> Self {
        Self { pending: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn announce(&self, message: &str) {
        self.announce_with_priority(message, AnnouncementPriority::Medium);
    }

    pub fn announce_with_priority(&self, message: &str, priority: AnnouncementPriority) {
        self.pending.lock().unwrap().push(Announcement::new(message).with_priority(priority));
    }

    pub fn drain(&self) -> Vec<Announcement> {
        std::mem::take(&mut self.pending.lock().unwrap())
    }

    pub fn has_pending(&self) -> bool {
        !self.pending.lock().unwrap().is_empty()
    }
}

impl Clone for AccessibilityAnnouncer {
    fn clone(&self) -> Self {
        Self { pending: Arc::clone(&self.pending) }
    }
}

impl Default for AccessibilityAnnouncer {
    fn default() -> Self { Self::new() }
}
