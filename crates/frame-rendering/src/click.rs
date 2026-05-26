use std::cell::RefCell;
use std::rc::Rc;

use frame_core::{Point, Rect};

struct ClickTarget {
    rect: Rect,
    callback: Rc<dyn Fn()>,
}

thread_local! {
    static CLICK_TARGETS: RefCell<Vec<ClickTarget>> = const { RefCell::new(Vec::new()) };
}

pub fn clear_click_targets() {
    CLICK_TARGETS.with(|t| t.borrow_mut().clear());
}

pub fn register_click_target(rect: Rect, callback: Rc<dyn Fn()>) {
    CLICK_TARGETS.with(|t| {
        t.borrow_mut().push(ClickTarget { rect, callback });
    });
}

pub fn dispatch_click(point: Point) -> bool {
    let callback = CLICK_TARGETS.with(|t| {
        let targets = t.borrow();
        for target in targets.iter() {
            if target.rect.contains(point) {
                return Some(target.callback.clone());
            }
        }
        None
    });
    if let Some(cb) = callback {
        cb();
        true
    } else {
        false
    }
}
