#![allow(unused_braces)]

use frame::{rsx, run_app, AppBuilder, Text, Column, Color};

#[frame::main]
fn main() {
    run_app(
        AppBuilder::new().title("Hello World").size(400.0, 300.0),
        || {
            Box::new(rsx! {
                <Column gap=16.0>
                    <Text size=32.0 color=Color::BLACK>"Hello, Frame!"</Text>
                    <Text size=16.0 color=Color::from_u8(100, 100, 100, 255)>"A cross-platform app framework in Rust"</Text>
                </Column>
            })
        },
    );
}
