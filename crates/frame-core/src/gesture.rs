use crate::{Point, Rect};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureState {
    Possible,
    Began,
    Changed,
    Ended,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone)]
pub enum GestureEvent {
    Tap { position: Point },
    DoubleTap { position: Point },
    LongPress { position: Point },
    Pan { position: Point, delta: Point, state: GestureState },
    Pinch { center: Point, scale: f32, state: GestureState },
}

pub struct TapRecognizer {
    area: Rect,
    state: GestureState,
    last_tap: Option<Instant>,
}

impl TapRecognizer {
    pub fn new(area: Rect) -> Self {
        Self {
            area,
            state: GestureState::Possible,
            last_tap: None,
        }
    }

    pub fn process(&mut self, position: Point, pressed: bool) -> Vec<GestureEvent> {
        let mut events = Vec::new();

        if !self.area.contains(position) {
            self.state = GestureState::Failed;
            return events;
        }

        if pressed && self.state == GestureState::Possible {
            self.state = GestureState::Began;
        } else if !pressed && self.state == GestureState::Began {
            self.state = GestureState::Possible;

            let now = Instant::now();

            if let Some(last) = self.last_tap {
                if now.duration_since(last).as_millis() < 300 {
                    events.push(GestureEvent::DoubleTap { position });
                    self.last_tap = None;
                    return events;
                }
            }

            events.push(GestureEvent::Tap { position });
            self.last_tap = Some(now);
        }

        events
    }

    pub fn reset(&mut self) {
        self.state = GestureState::Possible;
    }
}

pub struct LongPressRecognizer {
    area: Rect,
    state: GestureState,
    press_start: Option<Instant>,
    threshold_ms: u64,
}

impl LongPressRecognizer {
    pub fn new(area: Rect) -> Self {
        Self {
            area,
            state: GestureState::Possible,
            press_start: None,
            threshold_ms: 500,
        }
    }

    pub fn with_threshold(mut self, ms: u64) -> Self {
        self.threshold_ms = ms;
        self
    }

    pub fn process(&mut self, position: Point, pressed: bool) -> Vec<GestureEvent> {
        let mut events = Vec::new();

        if pressed && self.area.contains(position) && self.press_start.is_none() {
            self.press_start = Some(Instant::now());
            self.state = GestureState::Possible;
        } else if let Some(start) = self.press_start {
            if !pressed {
                self.press_start = None;
                self.state = GestureState::Failed;
            } else if start.elapsed().as_millis() as u64 >= self.threshold_ms
                && self.state == GestureState::Possible
            {
                self.state = GestureState::Began;
                events.push(GestureEvent::LongPress { position });
            }
        }

        events
    }

    pub fn reset(&mut self) {
        self.state = GestureState::Possible;
        self.press_start = None;
    }
}

pub struct PanRecognizer {
    area: Rect,
    state: GestureState,
    start_position: Option<Point>,
    last_position: Option<Point>,
}

impl PanRecognizer {
    pub fn new(area: Rect) -> Self {
        Self {
            area,
            state: GestureState::Possible,
            start_position: None,
            last_position: None,
        }
    }

    pub fn process(&mut self, position: Point, pressed: bool) -> Vec<GestureEvent> {
        let mut events = Vec::new();

        if pressed && self.area.contains(position) && self.start_position.is_none() {
            self.start_position = Some(position);
            self.last_position = Some(position);
            self.state = GestureState::Possible;
        } else if let Some(start) = self.start_position {
            if pressed {
                let delta = Point::new(
                    position.x - self.last_position.unwrap().x,
                    position.y - self.last_position.unwrap().y,
                );

                if self.state == GestureState::Possible {
                    let dx = position.x - start.x;
                    let dy = position.y - start.y;
                    if dx * dx + dy * dy > 25.0 {
                        self.state = GestureState::Began;
                    }
                }

                if self.state == GestureState::Began || self.state == GestureState::Changed {
                    self.state = GestureState::Changed;
                    events.push(GestureEvent::Pan {
                        position,
                        delta,
                        state: GestureState::Changed,
                    });
                }

                self.last_position = Some(position);
            } else {
                if self.state == GestureState::Began || self.state == GestureState::Changed {
                    self.state = GestureState::Ended;
                    let delta = Point::new(
                        position.x - self.last_position.unwrap().x,
                        position.y - self.last_position.unwrap().y,
                    );
                    events.push(GestureEvent::Pan {
                        position,
                        delta,
                        state: GestureState::Ended,
                    });
                }
                self.start_position = None;
                self.last_position = None;
                self.state = GestureState::Possible;
            }
        }

        events
    }

    pub fn reset(&mut self) {
        self.state = GestureState::Possible;
        self.start_position = None;
        self.last_position = None;
    }
}
