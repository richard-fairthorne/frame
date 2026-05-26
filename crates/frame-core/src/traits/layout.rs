use crate::{Size, Rect, WidgetId};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LayoutNode {
    pub id: WidgetId,
    pub children: Vec<LayoutNode>,
}

#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub node: LayoutNode,
    pub bounds: Rect,
}

pub trait Layout: 'static {
    fn measure(&self, node: LayoutNode, constraints: crate::Constraints) -> Size;
    fn layout(&self, node: LayoutNode, bounds: Rect) -> Vec<LayoutResult>;
}
