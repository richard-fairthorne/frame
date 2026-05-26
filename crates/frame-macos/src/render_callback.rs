use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static RENDER_CALLBACK: RefCell<Option<Rc<dyn Fn()>>> = RefCell::new(None);
}

pub fn set_render_callback(callback: Rc<dyn Fn()>) {
    RENDER_CALLBACK.with(|c| {
        *c.borrow_mut() = Some(callback);
    });
}

pub fn request_render() {
    RENDER_CALLBACK.with(|c| {
        if let Some(ref cb) = *c.borrow() {
            cb();
        }
    });
}
