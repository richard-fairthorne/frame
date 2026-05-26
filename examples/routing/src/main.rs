use frame::{Router, Navigator, RouteMatch, DeepLinkConfig};

fn main() {
    let router = Router::new()
        .route("/")
        .route("/users/:id")
        .route("/users/:id/posts/:slug")
        .route("/settings/:section");

    let navigator = Navigator::new(router);

    let paths = vec![
        "/",
        "/users/42",
        "/users/42/posts/hello-world",
        "/settings/profile",
    ];

    for path in paths {
        navigator.push(path);
        match navigator.resolve_current() {
            Some(RouteMatch { route_index, params }) => {
                println!("Navigated to '{}' -> route index {} with {} param(s)",
                    path, route_index,
                    if params.is_empty() { 0 } else { 1 }
                );
                for key in &["id", "slug", "section"] {
                    if let Some(val) = params.get(key) {
                        println!("  param '{}' = '{}'", key, val);
                    }
                }
            }
            None => println!("Failed to resolve '{}'", path),
        }
    }

    let deep_link_config = DeepLinkConfig::new("myapp")
        .domain("example.com");

    println!("\nDeep link config: scheme={}, domain={:?}",
        deep_link_config.scheme,
        deep_link_config.domains.first().unwrap_or(&String::new())
    );

    println!("\nRouting example complete.");
}
