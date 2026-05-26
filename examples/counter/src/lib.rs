#![allow(unused_braces)]

use std::sync::{Arc, Mutex, OnceLock};

use frame::{rsx, run_app, request_render, AppBuilder, Text, Column, Button, Color, Widget};

static COUNT: OnceLock<Arc<Mutex<i32>>> = OnceLock::new();

fn count() -> &'static Arc<Mutex<i32>> {
    COUNT.get_or_init(|| Arc::new(Mutex::new(0i32)))
}

fn make_counter_widget() -> Box<dyn Widget> {
    let count_for_ui = count().clone();
    let count_for_inc = count().clone();
    let count_for_dec = count().clone();

    Box::new(
        rsx! {
            <Column gap=20.0>
                <Text size=28.0>"Counter App"</Text>
                <Text size=22.0>{format!("Count: {}", *count_for_ui.lock().unwrap())}</Text>
                <Button
                    label="+ Increment"
                    on_click={
                        let count = count_for_inc.clone();
                        move || {
                            *count.lock().unwrap() += 1;
                            request_render();
                        }
                    }
                    background=Color::from_u8(0, 122, 255, 255)
                    padding=12.0
                />
                <Button
                    label="- Decrement"
                    on_click={
                        let count = count_for_dec.clone();
                        move || {
                            *count.lock().unwrap() -= 1;
                            request_render();
                        }
                    }
                    background=Color::from_u8(255, 59, 48, 255)
                    padding=12.0
                />
            </Column>
        },
    )
}

#[cfg(not(target_os = "android"))]
pub fn run() {
    frame::run_app(
        frame::AppBuilder::new().title("Counter").size(400.0, 300.0),
        make_counter_widget,
    );
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let builder = frame_android::AppBuilder::new()
        .title("Counter")
        .size(400.0, 300.0);

    frame_android::run_app_with(app, builder, make_counter_widget);
}
