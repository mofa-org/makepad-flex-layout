//! Drag-and-drop handling for the panel grid

use makepad_widgets::*;

/// Drop position calculated during drag operations.
///
/// Contains the target row/column and a rect for visual preview.
#[derive(Clone, Debug)]
pub struct DropPosition {
    /// Target row index (0, 1, 2, ...)
    pub row: usize,

    /// Target column index within the row
    pub col: usize,

    /// Rectangle for drawing drop preview overlay
    pub rect: Rect,
}

impl DropPosition {
    /// Create a new drop position
    pub fn new(row: usize, col: usize, rect: Rect) -> Self {
        Self { row, col, rect }
    }
}

/// Calculate drop position from cursor location
///
/// # Arguments
/// * `abs` - Absolute cursor position
/// * `container_rect` - The container's rectangle
/// * `rows_with_panels` - Vector of panel counts per visible row
/// * `row_to_actual` - Mapping from visual row index to actual row index
///
/// # Returns
/// `Some(DropPosition)` if cursor is within container, `None` otherwise
pub fn calculate_drop_position(
    abs: DVec2,
    container_rect: Rect,
    rows_with_panels: &[Vec<u64>],
    row_to_actual: &[usize],
) -> Option<DropPosition> {
    let num_rows = rows_with_panels.len();
    if num_rows == 0 {
        return None;
    }

    if !container_rect.contains(abs) {
        return None;
    }

    // Calculate which row the cursor is in
    let row_height = container_rect.size.y / num_rows as f64;
    let rel_y = abs.y - container_rect.pos.y;
    let visual_row = ((rel_y / row_height) as usize).min(num_rows - 1);

    // Get actual row index
    let actual_row = if visual_row < row_to_actual.len() {
        row_to_actual[visual_row]
    } else {
        visual_row
    };

    // Calculate which column within that row
    let cols_in_row = rows_with_panels.get(visual_row).map(|r| r.len()).unwrap_or(1).max(1);
    let col_width = container_rect.size.x / cols_in_row as f64;
    let rel_x = abs.x - container_rect.pos.x;
    let col = ((rel_x / col_width) as usize).min(cols_in_row);

    // Calculate the preview rectangle for this slot
    let preview_col = col.min(cols_in_row.saturating_sub(1));
    let rect = Rect {
        pos: DVec2 {
            x: container_rect.pos.x + preview_col as f64 * col_width,
            y: container_rect.pos.y + visual_row as f64 * row_height,
        },
        size: DVec2 {
            x: col_width,
            y: row_height,
        },
    };

    Some(DropPosition::new(actual_row, col, rect))
}
