use crate::node::FlexNode;
use crate::style::{AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent};
use frame_core::traits::layout::{Layout, LayoutNode, LayoutResult};
use frame_core::{Constraints, Rect, Size};

struct ResolvedItem {
    index: usize,
    main_size: f32,
    cross_size: f32,
}

struct ResolvedLine {
    items: Vec<ResolvedItem>,
    cross_size: f32,
}

pub struct FlexEngine;

impl FlexEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn layout_node(
        &self,
        node: &FlexNode,
        constraints: Constraints,
    ) -> (Size, Vec<(usize, Rect)>) {
        let available = constraints.max();
        let padding = node.style.padding;
        let inner_width = (available.width - padding * 2.0).max(0.0);
        let inner_height = (available.height - padding * 2.0).max(0.0);

        if node.children.is_empty() {
            let size = node
                .explicit_size
                .unwrap_or(Size::new(inner_width, inner_height));
            return (constraints.constrain(size), vec![]);
        }

        let is_row = matches!(
            node.style.direction,
            FlexDirection::Row | FlexDirection::RowReverse
        );
        let is_reverse = matches!(
            node.style.direction,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        );

        let main_available = if is_row { inner_width } else { inner_height };
        let cross_available = if is_row { inner_height } else { inner_width };

        let hypothetical_main: Vec<f32> = node
            .children
            .iter()
            .map(|child| self.hypothetical_main(child, is_row))
            .collect();

        let lines = self.collect_flex_lines(node, &hypothetical_main, main_available);
        let is_single_line = lines.len() == 1;

        let resolved_lines: Vec<ResolvedLine> = lines
            .into_iter()
            .map(|line| {
                self.resolve_line(
                    node,
                    &line,
                    &hypothetical_main,
                    is_row,
                    main_available,
                    cross_available,
                    is_single_line,
                )
            })
            .collect();

        let total_cross: f32 = resolved_lines.iter().map(|l| l.cross_size).sum();
        let container_cross = if cross_available > 0.0 {
            cross_available
        } else {
            total_cross
        };

        let mut result = Vec::new();
        let mut cross_cursor = 0.0f32;

        for line in &resolved_lines {
            let alignment_cross = if is_single_line && cross_available > 0.0 {
                container_cross
            } else {
                line.cross_size
            };

            let used_main: f32 = line.items.iter().map(|i| i.main_size).sum();
            let (mut main_cursor, step) = self.compute_main_positioning(
                node.style.justify_content,
                used_main,
                line.items.len(),
                node.style.gap,
                main_available,
            );

            for item in &line.items {
                let child = &node.children[item.index];
                let effective_align = self.effective_align(child, node.style.align_items);
                let cross_offset =
                    self.compute_cross_offset(item.cross_size, alignment_cross, effective_align);

                let margin = child.style.margin;

                let (x, y) = if is_row {
                    let mx = if is_reverse {
                        main_available - main_cursor - item.main_size
                    } else {
                        main_cursor
                    };
                    (
                        padding + margin + mx,
                        padding + margin + cross_cursor + cross_offset,
                    )
                } else {
                    let my = if is_reverse {
                        main_available - main_cursor - item.main_size
                    } else {
                        main_cursor
                    };
                    (
                        padding + margin + cross_cursor + cross_offset,
                        padding + margin + my,
                    )
                };

                let (w, h) = if is_row {
                    (item.main_size, item.cross_size)
                } else {
                    (item.cross_size, item.main_size)
                };

                result.push((item.index, Rect::from_components(x, y, w, h)));
                main_cursor += item.main_size + step;
            }

            cross_cursor += line.cross_size;
        }

        let total_main_used: f32 = resolved_lines
            .iter()
            .flat_map(|l| l.items.iter())
            .map(|i| i.main_size)
            .sum::<f32>()
            + node.style.gap * (node.children.len().saturating_sub(1) as f32);

        let max_cross: f32 = resolved_lines
            .iter()
            .map(|l| l.cross_size)
            .fold(0.0_f32, f32::max);

        let self_size = node.explicit_size.unwrap_or(if is_row {
            Size::new(total_main_used, max_cross)
        } else {
            Size::new(cross_available.max(max_cross), total_main_used)
        });

        (constraints.constrain(self_size), result)
    }

    fn hypothetical_main(&self, child: &FlexNode, is_row: bool) -> f32 {
        if let Some(basis) = child.style.flex_basis {
            return basis;
        }
        if let Some(size) = child.explicit_size {
            return if is_row { size.width } else { size.height };
        }
        0.0
    }

    fn collect_flex_lines(
        &self,
        node: &FlexNode,
        hypothetical_main: &[f32],
        main_available: f32,
    ) -> Vec<Vec<usize>> {
        if node.style.wrap == FlexWrap::NoWrap {
            return vec![(0..node.children.len()).collect()];
        }

        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let mut accumulated = 0.0f32;
        let gap = node.style.gap;

        for (i, &hyp) in hypothetical_main.iter().enumerate() {
            let needed = if current_line.is_empty() {
                hyp
            } else {
                accumulated + gap + hyp
            };
            if needed > main_available && !current_line.is_empty() {
                lines.push(std::mem::take(&mut current_line));
                accumulated = hyp;
            } else {
                accumulated = needed;
            }
            current_line.push(i);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(vec![]);
        }

        lines
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_line(
        &self,
        node: &FlexNode,
        line_indices: &[usize],
        hypothetical_main: &[f32],
        is_row: bool,
        main_available: f32,
        cross_available: f32,
        is_single_line: bool,
    ) -> ResolvedLine {
        let gap = node.style.gap;
        let total_gap = gap * (line_indices.len().saturating_sub(1) as f32);
        let available_for_items = (main_available - total_gap).max(0.0);

        let mut items: Vec<ResolvedItem> = line_indices
            .iter()
            .map(|&idx| {
                let child = &node.children[idx];
                let base = child.style.flex_basis.unwrap_or(hypothetical_main[idx]);
                let cross = self.natural_cross_size(child, is_row);
                ResolvedItem {
                    index: idx,
                    main_size: base,
                    cross_size: cross,
                }
            })
            .collect();

        let total_base: f32 = items.iter().map(|i| i.main_size).sum();
        let free_space = available_for_items - total_base;

        if free_space > 0.0 {
            let total_grow: f32 = items
                .iter()
                .map(|i| node.children[i.index].style.flex_grow)
                .sum();
            if total_grow > 0.0 {
                for item in &mut items {
                    let grow = node.children[item.index].style.flex_grow;
                    if grow > 0.0 {
                        item.main_size += (grow / total_grow) * free_space;
                    }
                }
            }
        } else if free_space < 0.0 {
            let total_scaled_shrink: f32 = items
                .iter()
                .map(|i| {
                    let shrink = node.children[i.index].style.flex_shrink;
                    shrink * i.main_size
                })
                .sum();
            if total_scaled_shrink > 0.0 {
                for item in &mut items {
                    let shrink = node.children[item.index].style.flex_shrink;
                    let scaled = shrink * item.main_size;
                    if scaled > 0.0 {
                        item.main_size =
                            (item.main_size + (scaled / total_scaled_shrink) * free_space)
                                .max(0.0);
                    }
                }
            }
        }

        for item in &mut items {
            let child = &node.children[item.index];
            if let Some(min) = &child.min_size {
                let min_main = if is_row { min.width } else { min.height };
                item.main_size = item.main_size.max(min_main);
            }
            if let Some(max) = &child.max_size {
                let max_main = if is_row { max.width } else { max.height };
                item.main_size = item.main_size.min(max_main);
            }
        }

        let natural_line_cross: f32 = items.iter().map(|i| i.cross_size).fold(0.0_f32, f32::max);
        let line_cross = if is_single_line && cross_available > 0.0 {
            cross_available
        } else {
            natural_line_cross
        };

        for item in &mut items {
            let child = &node.children[item.index];
            let align = self.effective_align(child, node.style.align_items);
            if align == AlignItems::Stretch && child.explicit_size.is_none() {
                item.cross_size = line_cross;
            }
        }

        let final_cross = items
            .iter()
            .map(|i| i.cross_size)
            .fold(0.0_f32, f32::max)
            .max(natural_line_cross);

        ResolvedLine {
            items,
            cross_size: final_cross,
        }
    }

    fn natural_cross_size(&self, child: &FlexNode, is_row: bool) -> f32 {
        if let Some(size) = child.explicit_size {
            return if is_row { size.height } else { size.width };
        }
        0.0
    }

    fn effective_align(&self, child: &FlexNode, parent_align: AlignItems) -> AlignItems {
        match child.style.align_self {
            AlignSelf::Auto => parent_align,
            AlignSelf::Start => AlignItems::Start,
            AlignSelf::Center => AlignItems::Center,
            AlignSelf::End => AlignItems::End,
            AlignSelf::Stretch => AlignItems::Stretch,
        }
    }

    fn compute_main_positioning(
        &self,
        justify: JustifyContent,
        used_main: f32,
        item_count: usize,
        gap: f32,
        main_available: f32,
    ) -> (f32, f32) {
        match justify {
            JustifyContent::Start => (0.0, gap),
            JustifyContent::End => {
                let total_gap = gap * item_count.saturating_sub(1) as f32;
                let free = (main_available - used_main - total_gap).max(0.0);
                (free, gap)
            }
            JustifyContent::Center => {
                let total_gap = gap * item_count.saturating_sub(1) as f32;
                let free = main_available - used_main - total_gap;
                (free / 2.0, gap)
            }
            JustifyContent::SpaceBetween => {
                if item_count <= 1 {
                    (0.0, 0.0)
                } else {
                    let free = main_available - used_main;
                    (0.0, free / (item_count - 1) as f32)
                }
            }
            JustifyContent::SpaceAround => {
                if item_count == 0 {
                    (0.0, 0.0)
                } else {
                    let free = main_available - used_main;
                    let spacing = free / item_count as f32;
                    (spacing / 2.0, spacing)
                }
            }
            JustifyContent::SpaceEvenly => {
                let free = main_available - used_main;
                let spacing = free / (item_count + 1) as f32;
                (spacing, spacing)
            }
        }
    }

    fn compute_cross_offset(
        &self,
        item_cross: f32,
        line_cross: f32,
        align: AlignItems,
    ) -> f32 {
        match align {
            AlignItems::Start | AlignItems::Stretch => 0.0,
            AlignItems::Center => ((line_cross - item_cross) / 2.0).max(0.0),
            AlignItems::End => (line_cross - item_cross).max(0.0),
        }
    }
}

impl Default for FlexEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout for FlexEngine {
    fn measure(&self, _node: LayoutNode, constraints: Constraints) -> Size {
        constraints.constrain(constraints.max())
    }

    fn layout(&self, _node: LayoutNode, _bounds: Rect) -> Vec<LayoutResult> {
        vec![]
    }
}
