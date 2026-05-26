use frame_nav::{DeepLinkConfig, Navigator, Router};

#[test]
fn router_matches_exact_path() {
    let router = Router::new().route("/").route("/users").route("/about");
    let m = router.resolve("/users").unwrap();
    assert_eq!(m.route_index, 1);
}

#[test]
fn router_matches_root() {
    let router = Router::new().route("/").route("/users");
    let m = router.resolve("/").unwrap();
    assert_eq!(m.route_index, 0);
}

#[test]
fn router_no_match() {
    let router = Router::new().route("/").route("/users");
    assert!(router.resolve("/nonexistent").is_none());
}

#[test]
fn router_extracts_params() {
    let router = Router::new().route("/users/:id");
    let m = router.resolve("/users/42").unwrap();
    assert_eq!(m.params.get("id"), Some("42"));
}

#[test]
fn router_multiple_params() {
    let router = Router::new().route("/products/:category/:id");
    let m = router.resolve("/products/electronics/123").unwrap();
    assert_eq!(m.params.get("category"), Some("electronics"));
    assert_eq!(m.params.get("id"), Some("123"));
}

#[test]
fn navigator_push_and_back() {
    let router = Router::new().route("/").route("/users").route("/users/:id");
    let nav = Navigator::new(router);
    assert_eq!(nav.current(), "/");
    nav.push("/users");
    assert_eq!(nav.current(), "/users");
    nav.push("/users/42");
    assert_eq!(nav.current(), "/users/42");
    nav.back();
    assert_eq!(nav.current(), "/users");
}

#[test]
fn navigator_replace() {
    let router = Router::new().route("/").route("/users");
    let nav = Navigator::new(router);
    nav.push("/users");
    nav.replace("/");
    assert_eq!(nav.current(), "/");
}

#[test]
fn navigator_resolve_current() {
    let router = Router::new().route("/").route("/users/:id");
    let nav = Navigator::new(router);
    nav.push("/users/99");
    let m = nav.resolve_current().unwrap();
    assert_eq!(m.params.get("id"), Some("99"));
}

#[test]
fn deep_link_config() {
    let config = DeepLinkConfig::new("myapp")
        .domain("myapp.com")
        .domain("app.myapp.com");
    assert_eq!(config.scheme, "myapp");
    assert_eq!(config.domains.len(), 2);
}
