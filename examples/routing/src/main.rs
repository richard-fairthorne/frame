#![allow(unused_braces)]

use frame::{rsx, run_app, AppBuilder, Text, Column, Row, Button, Color, Container, Router, Navigator};
use std::rc::Rc;

#[frame::main]
fn main() {
    let router = Router::new()
        .route("/")
        .route("/users/:id")
        .route("/settings");

    let navigator = Rc::new(Navigator::new(router));

    run_app(
        AppBuilder::new().title("Routing").size(400.0, 500.0),
        move || {
            let nav = navigator.clone();
            Box::new(rsx! {
                <Column gap=16.0>
                    <Text size=24.0>"Navigation Demo"</Text>
                    <Row gap=8.0>
                        <Button label="Home" on_click={
                            let nav = nav.clone();
                            move || { nav.push("/"); }
                        } padding=8.0 />
                        <Button label="User 42" on_click={
                            let nav = nav.clone();
                            move || { nav.push("/users/42"); }
                        } padding=8.0 />
                        <Button label="Settings" on_click={
                            let nav = nav.clone();
                            move || { nav.push("/settings"); }
                        } padding=8.0 />
                    </Row>
                    <Container padding=16.0 background=Color::from_u8(240, 240, 240, 255)>
                        <Text size=16.0>{
                            match nav.resolve_current() {
                                Some(r) => format!("Route #{}", r.route_index),
                                None => "No route matched".into(),
                            }
                        }</Text>
                    </Container>
                </Column>
            })
        },
    );
}
