use crate::params::RouteParams;

#[derive(Debug, Clone)]
pub struct Route {
    pub pattern: String,
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
enum Segment {
    Literal(String),
    Param(String),
    Wildcard,
}

impl Route {
    pub fn new(pattern: impl Into<String>) -> Self {
        let pattern = pattern.into();
        let segments = Self::parse_pattern(&pattern);
        Self { pattern, segments }
    }

    fn parse_pattern(pattern: &str) -> Vec<Segment> {
        pattern
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s == "*" {
                    Segment::Wildcard
                } else if let Some(name) = s.strip_prefix(':') {
                    Segment::Param(name.to_string())
                } else {
                    Segment::Literal(s.to_string())
                }
            })
            .collect()
    }

    pub fn matches(&self, path: &str) -> Option<RouteParams> {
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if path_segments.len() != self.segments.len() {
            if let Some(Segment::Wildcard) = self.segments.last() {
                if path_segments.len() >= self.segments.len() - 1 {
                    let mut params = RouteParams::new();
                    for (i, seg) in self.segments.iter().enumerate() {
                        if i >= path_segments.len() {
                            break;
                        }
                        if let Segment::Param(name) = seg {
                            params.insert(name.clone(), path_segments[i]);
                        }
                    }
                    return Some(params);
                }
            }
            return None;
        }

        let mut params = RouteParams::new();
        for (seg, path_seg) in self.segments.iter().zip(path_segments.iter()) {
            match seg {
                Segment::Literal(lit) => {
                    if lit != *path_seg {
                        return None;
                    }
                }
                Segment::Param(name) => {
                    params.insert(name.clone(), *path_seg);
                }
                Segment::Wildcard => {}
            }
        }
        Some(params)
    }
}
