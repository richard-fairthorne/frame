use crate::{Point, Rect};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InteractionId(u64);

#[derive(Debug, Clone)]
pub enum InteractionEvent {
    Tap { id: InteractionId, position: Point },
    DoubleTap { id: InteractionId, position: Point },
    LongPress { id: InteractionId, position: Point },
    Hover { id: InteractionId, position: Point },
    Focus { id: InteractionId },
    Blur { id: InteractionId },
}

pub type Callback = Box<dyn FnMut(InteractionEvent) + Send + Sync>;

pub struct InteractionRegistry {
    hit_areas: HashMap<InteractionId, Rect>,
    callbacks: HashMap<InteractionId, Callback>,
    next_id: u64,
    hover_target: Option<InteractionId>,
    focus_target: Option<InteractionId>,
}

impl InteractionRegistry {
    pub fn new() -> Self {
        Self {
            hit_areas: HashMap::new(),
            callbacks: HashMap::new(),
            next_id: 1,
            hover_target: None,
            focus_target: None,
        }
    }

    pub fn register(&mut self, area: Rect) -> InteractionId {
        let id = InteractionId(self.next_id);
        self.next_id += 1;
        self.hit_areas.insert(id, area);
        id
    }

    pub fn set_callback(&mut self, id: InteractionId, callback: Callback) {
        self.callbacks.insert(id, callback);
    }

    pub fn hit_test(&self, point: Point) -> Option<InteractionId> {
        for (&id, area) in &self.hit_areas {
            if area.contains(point) {
                return Some(id);
            }
        }
        None
    }

    pub fn dispatch_event(&mut self, event: InteractionEvent) -> bool {
        let id = match &event {
            InteractionEvent::Tap { id, .. } => *id,
            InteractionEvent::DoubleTap { id, .. } => *id,
            InteractionEvent::LongPress { id, .. } => *id,
            InteractionEvent::Hover { id, .. } => *id,
            InteractionEvent::Focus { id } => *id,
            InteractionEvent::Blur { id } => *id,
        };
        if let Some(callback) = self.callbacks.get_mut(&id) {
            callback(event);
            true
        } else {
            false
        }
    }

    pub fn process_mouse_input(&mut self, x: f32, y: f32, pressed: bool) -> Vec<InteractionEvent> {
        let point = Point::new(x, y);
        let mut events = Vec::new();

        let new_target = self.hit_test(point);

        if new_target != self.hover_target {
            if let Some(old_id) = self.hover_target {
                events.push(InteractionEvent::Hover { id: old_id, position: point });
            }
            if let Some(new_id) = new_target {
                events.push(InteractionEvent::Hover { id: new_id, position: point });
            }
            self.hover_target = new_target;
        }

        if pressed {
            if let Some(id) = new_target {
                events.push(InteractionEvent::Tap { id, position: point });
                self.focus_target = Some(id);
            }
        }

        events
    }
}

impl Default for InteractionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
