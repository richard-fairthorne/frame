#![allow(unused_braces)]

use frame::{rsx, run_app, request_render, AppBuilder, Text, Column, Button, Color, Signal};

#[frame::main]
fn main() {
    let count = Signal::new(0);

    run_app(
        AppBuilder::new().title("Counter").size(400.0, 300.0),
        move || {
            let count = count.clone();
            Box::new(rsx! {
                <Column gap=20.0>
                    <Text size=28.0>"Counter App"</Text>
                    <Text size=22.0>{format!("Count: {}", count.get())}</Text>
                    <Button
                        label="+ Increment"
                        on_click={
                            let count = count.clone();
                            move || { count.set(count.get() + 1); request_render(); }
                        }
                        background=Color::from_u8(0, 122, 255, 255)
                        padding=12.0
                    />
                    <Button
                        label="- Decrement"
                        on_click={
                            let count = count.clone();
                            move || { count.set(count.get() - 1); request_render(); }
                        }
                        background=Color::from_u8(255, 59, 48, 255)
                        padding=12.0
                    />
                </Column>
            })
        },
    );
}
