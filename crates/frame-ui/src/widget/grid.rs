use frame_core::traits::widget::{RenderContext, Widget, WidgetNode, WidgetOutput};
use frame_core::{Constraints, Size};

use crate::style::Style;

pub struct Grid {
    children: Vec<WidgetNode>,
    columns: usize,
    row_gap: f32,
    column_gap: f32,
    style: Style,
}

impl Grid {
    pub fn new(columns: usize) -> Self {
        Self {
            children: Vec::new(),
            columns: columns.max(1),
            row_gap: 0.0,
            column_gap: 0.0,
            style: Style::default(),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self.column_gap = gap;
        self
    }

    pub fn row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self
    }

    pub fn column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(WidgetNode {
            id: frame_core::WidgetId::default(),
            widget: Box::new(child),
        });
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn children(&self) -> &[WidgetNode] {
        &self.children
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn row_gap_val(&self) -> f32 {
        self.row_gap
    }

    pub fn column_gap_val(&self) -> f32 {
        self.column_gap
    }
}

impl Widget for Grid {
    fn render(&mut self, _ctx: &mut RenderContext) -> WidgetOutput {
        WidgetOutput::Children {
            children: std::mem::take(&mut self.children),
        }
    }

    fn measure(&self, constraints: Constraints) -> Size {
        if self.children.is_empty() {
            return constraints.constrain(Size::new(
                self.style.width.unwrap_or(0.0),
                self.style.height.unwrap_or(0.0),
            ));
        }

        let available_width = constraints.max().width;
        let total_column_gap = self.column_gap * (self.columns - 1) as f32;
        let cell_width = (available_width - total_column_gap) / self.columns as f32;

        let cell_constraints = Constraints::new(
            Size::new(0.0, 0.0),
            Size::new(cell_width, constraints.max().height),
        );

        let num_rows = self.children.len().div_ceil(self.columns);
        let mut row_heights = vec![0.0f32; num_rows];

        for (i, child) in self.children.iter().enumerate() {
            let row = i / self.columns;
            let size = child.widget.measure(cell_constraints);
            row_heights[row] = row_heights[row].max(size.height);
        }

        let total_height: f32 = row_heights.iter().sum::<f32>()
            + self.row_gap * num_rows.saturating_sub(1) as f32;

        constraints.constrain(Size::new(
            self.style.width.unwrap_or(available_width),
            self.style.height.unwrap_or(total_height),
        ))
    }
}
