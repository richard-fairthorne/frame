use frame::{view, Signal, Effect, Text, Column, Button, Color, Constraints, Size, WindowConfig, Widget};

fn main() {
    let count = Signal::new(0i32);
    let name = Signal::new("Frame".to_string());

    Effect::new({
        let count = count.clone();
        let name = name.clone();
        move || {
            let c = count.get();
            let n = name.get_cloned();
            println!("Counter: {c}, Name: {n}");
        }
    });

    count.set(1);
    count.set(2);

    let column = view! {
        Column {
            gap: 16.0,
            Text {
                text: "Hello, Frame!",
                size: 24.0,
                color: Color::BLACK,
            }
            Button {
                label: "Click me",
                on_click: || {
                    println!("Button clicked!");
                },
            }
        }
    };

    let constraints = Constraints::tight(Size::new(400.0, 300.0));
    let layout_size = column.measure(constraints);
    println!("Layout size: {}x{}", layout_size.width, layout_size.height);

    let _window_config = WindowConfig {
        title: "Hello Frame".into(),
        size: Size::new(400.0, 300.0),
        resizable: true,
    };

    println!("Hello World example complete.");
    println!("Full window rendering requires platform runtime (run on macOS with 'cargo frame run').");
}
