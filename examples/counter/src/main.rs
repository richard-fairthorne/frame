#![allow(unused_braces)]

use std::sync::{Arc, Mutex, OnceLock};

use frame::{rsx, run_app, request_render, AppBuilder, Text, Column, Button, Color};

static COUNT: OnceLock<Arc<Mutex<i32>>> = OnceLock::new();

fn count() -> &'static Arc<Mutex<i32>> {
    COUNT.get_or_init(|| Arc::new(Mutex::new(0i32)))
}

#[frame::main]
fn main() {
    let count_for_ui = count().clone();
    let count_for_inc = count().clone();
    let count_for_dec = count().clone();

    run_app(
        AppBuilder::new().title("Counter").size(400.0, 300.0),
        move || {
            let count_for_ui = count_for_ui.clone();
            let count_for_inc = count_for_inc.clone();
            let count_for_dec = count_for_dec.clone();

            Box::new(
                rsx! {
                    <Column gap=20.0>
                        <Text size=28.0>"Counter App"</Text>
                        <Text size=22.0>{format!("Count: {}", *count_for_ui.lock().unwrap())}</Text>
                        <Button
                            label="+ Increment"
                            on_click={
                                move || {
                                    *count_for_inc.lock().unwrap() += 1;
                                    request_render();
                                }
                            }
                            background=Color::from_u8(0, 122, 255, 255)
                            padding=12.0
                        />
                        <Button
                            label="- Decrement"
                            on_click={
                                move || {
                                    *count_for_dec.lock().unwrap() -= 1;
                                    request_render();
                                }
                            }
                            background=Color::from_u8(255, 59, 48, 255)
                            padding=12.0
                        />
                    </Column>
                },
            )
        },
    );
}
