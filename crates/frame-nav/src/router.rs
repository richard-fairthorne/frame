use crate::params::RouteParams;
use crate::route::Route;

pub struct RouteMatch {
    pub route_index: usize,
    pub params: RouteParams,
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn route(mut self, pattern: &str) -> Self {
        self.routes.push(Route::new(pattern));
        self
    }

    pub fn resolve(&self, path: &str) -> Option<RouteMatch> {
        for (i, route) in self.routes.iter().enumerate() {
            if let Some(params) = route.matches(path) {
                return Some(RouteMatch {
                    route_index: i,
                    params,
                });
            }
        }
        None
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
