use crate::style::FlexStyle;
use frame_core::Size;

#[derive(Debug, Clone)]
pub struct FlexNode {
    pub style: FlexStyle,
    pub children: Vec<FlexNode>,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub explicit_size: Option<Size>,
}

impl FlexNode {
    pub fn new(style: FlexStyle) -> Self {
        Self {
            style,
            children: Vec::new(),
            min_size: None,
            max_size: None,
            explicit_size: None,
        }
    }

    pub fn leaf(size: Size) -> Self {
        Self {
            style: FlexStyle::default(),
            children: Vec::new(),
            min_size: None,
            max_size: None,
            explicit_size: Some(size),
        }
    }

    pub fn child(mut self, child: FlexNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn child_leaf(mut self, size: Size) -> Self {
        self.children.push(Self::leaf(size));
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.explicit_size = Some(Size::new(width, height));
        self
    }

    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.min_size = Some(Size::new(width, height));
        self
    }

    pub fn max_size(mut self, width: f32, height: f32) -> Self {
        self.max_size = Some(Size::new(width, height));
        self
    }
}
