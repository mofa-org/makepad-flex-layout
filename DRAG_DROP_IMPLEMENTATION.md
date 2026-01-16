# Drag-and-Drop Implementation for Makepad Flex Layout

This document describes the implementation of drag-and-drop functionality for sub-windows in a Makepad-based flex layout system.

## Overview

The implementation allows users to drag windows between rows, causing the layout to dynamically reconfigure. When a window is moved:
- The source row shrinks (fewer windows)
- The target row expands (more windows)
- Rows with no windows are automatically hidden

## Architecture

### Data Model

```rust
/// Row assignments - each row contains a list of window IDs in order
/// This is the source of truth for layout
#[rust]
row_assignments: [Vec<usize>; 3],
```

Instead of a flat `window_order: Vec<usize>`, we use a per-row structure where each of the 3 rows maintains its own list of window IDs. This enables true physical window movement between rows.

### Key Components

#### 1. SubWindow Widget (`src/app.rs`)

The `SubWindow` widget represents an individual draggable window panel.

**Drag Detection:**
```rust
// Handle drag on drag handle or title bar
match event.hits(cx, drag_handle.area()) {
    Hit::FingerDown(fe) => {
        self.is_dragging = false;
        self.drag_start = fe.abs;
    }
    Hit::FingerMove(fe) => {
        let dist = (fe.abs - self.drag_start).length();
        if !self.is_dragging && dist > 10.0 {
            self.is_dragging = true;
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SubWindowAction::StartDrag(self.window_id),
            );
        }
    }
    Hit::FingerUp(_) => {
        self.is_dragging = false;
    }
    _ => {}
}
```

**Key features:**
- Drag threshold of 10 pixels prevents accidental drags
- Emits `SubWindowAction::StartDrag(window_id)` when drag starts
- Supports dragging from both the drag handle icon and the title bar

**Visual Updates:**
```rust
fn apply_visual_update(&mut self, cx: &mut Cx2d) {
    if !self.needs_visual_update {
        return;
    }
    self.needs_visual_update = false;

    // Apply color based on window_id
    let colors = [/* 9 distinct colors */];
    let color = colors[id % colors.len()];

    self.view.apply_over(cx, live! {
        draw_bg: { color: (color) }
    });

    // Update title and content labels
    self.view.label(id!(title_bar.title)).set_text(cx, &title);
    self.view.label(id!(content.content_label)).set_text(cx, &content);
}
```

The `needs_visual_update` flag pattern defers visual updates to `draw_walk`, ensuring they happen at the right time in Makepad's rendering pipeline.

#### 2. ContentArea Widget

The `ContentArea` manages the grid of windows and handles drop operations.

**Capture Override for Drag Tracking:**
```rust
match event.hits_with_capture_overload(cx, self.view.area(), self.dragging_window.is_some()) {
    Hit::FingerMove(fe) if self.dragging_window.is_some() => {
        // Update drop preview
        if let Some(pos) = self.find_drop_position(cx, fe.abs) {
            self.drop_state = Some(pos);
        }
        self.view.redraw(cx);
    }
    Hit::FingerUp(fe) => {
        if let Some(dragged_id) = self.dragging_window {
            self.handle_drop(cx, fe.abs, dragged_id);
        }
        self.dragging_window = None;
        self.drop_state = None;
    }
    _ => {}
}
```

The `hits_with_capture_overload` function allows the ContentArea to receive mouse events even when the cursor moves outside the original drag target.

#### 3. Drop Position Detection

```rust
fn find_drop_position(&self, cx: &Cx, abs: DVec2) -> Option<DropPosition> {
    // Get visible windows per row
    let rows_with_windows: Vec<Vec<usize>> = (0..3)
        .map(|r| self.visible_windows_in_row(r))
        .filter(|row| !row.is_empty())
        .collect();

    // Calculate which row based on cursor Y position
    let row_height = container_rect.size.y / num_rows as f64;
    let visual_row = ((rel_y / row_height) as usize).min(num_rows - 1);

    // Map visual row to actual row index (accounting for hidden rows)
    // ...

    // Calculate column within that row
    let cols_in_row = rows_with_windows[visual_row].len().max(1);
    let col_width = container_rect.size.x / cols_in_row as f64;
    let col = ((rel_x / col_width) as usize).min(cols_in_row);

    Some(DropPosition { row: actual_row, col, rect })
}
```

This calculates the drop target based on cursor position, accounting for:
- Variable number of visible rows
- Variable number of windows per row
- Mapping visual row index to actual row index (0, 1, or 2)

#### 4. Handle Drop Operation

```rust
fn handle_drop(&mut self, cx: &mut Cx, abs: DVec2, dragged_window_id: usize) {
    let Some(drop_pos) = self.find_drop_position(cx, abs) else { return };
    let Some((src_row, src_col)) = self.find_window_row(dragged_window_id) else { return };

    // Remove window from source row
    self.row_assignments[src_row].remove(src_col);

    // Calculate insert position in target row
    let visible_in_target = self.visible_windows_in_row(target_row).len();
    let insert_col = target_col.min(visible_in_target);

    // Adjust for same-row moves
    let insert_col = if src_row == target_row && target_col > src_col {
        insert_col.saturating_sub(1)
    } else {
        insert_col
    };

    // Insert window at target position
    self.row_assignments[target_row].insert(insert_col, dragged_window_id);

    self.needs_layout_update = true;
    self.view.redraw(cx);
}
```

#### 5. Apply Row Layout

```rust
fn apply_row_layout(&mut self, cx: &mut Cx) {
    const SLOTS_PER_ROW: usize = 9;

    // Hide all slots first
    for row_idx in 0..3 {
        for slot_idx in 0..SLOTS_PER_ROW {
            self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                visible: false, width: 0, height: 0
            });
        }
    }

    // Configure each row based on its assigned windows
    for row_idx in 0..3 {
        let windows_in_row = &visible_per_row[row_idx];

        if windows_in_row.is_empty() {
            // Hide empty rows
            self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                visible: false, height: 0
            });
        } else {
            // Show row with Fill height
            self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                visible: true, height: Fill
            });

            // Show slots for windows in this row
            for (slot_idx, &window_id) in windows_in_row.iter().enumerate() {
                self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                    visible: true, width: Fill, height: Fill
                });
                self.view.sub_window(row_slot_ids[row_idx][slot_idx])
                    .set_window_id(cx, window_id);
            }
        }
    }
}
```

### Live Design Structure

```rust
window_container = <View> {
    width: Fill
    height: Fill
    flow: Down

    // Row 1: up to 9 windows
    row1 = <View> {
        width: Fill
        height: Fill
        flow: Right

        s1_1 = <SubWindow> { width: Fill, height: Fill }
        s1_2 = <SubWindow> { width: Fill, height: Fill }
        // ... up to s1_9
    }

    // Row 2 and Row 3 similar structure
}
```

Each row has 9 slots, allowing maximum flexibility. Unused slots are hidden with `width: 0, height: 0`.

## Visual Feedback

### Drop Preview
```rust
#[live]
drop_preview: DrawColor,

// In draw_walk:
if let Some(ref pos) = self.drop_state {
    self.drop_preview.draw_abs(cx, pos.rect);
}
```

A semi-transparent overlay shows where the window will be dropped.

### Window Colors
Each window has a distinct color based on its ID:
- Window 1: Red
- Window 2: Green
- Window 3: Blue
- Window 4: Yellow
- Window 5: Magenta
- Window 6: Cyan
- Window 7: Orange
- Window 8: Purple
- Window 9: Light Green

## Key Makepad Patterns Used

1. **`apply_over`**: Runtime style changes for visibility, sizing, and colors
2. **`hits_with_capture_overload`**: Capture mouse events during drag operations
3. **`widget_action`**: Communication between widgets via actions
4. **`needs_visual_update` flag**: Defer updates to draw phase
5. **`set_text`**: Update label content dynamically

## Limitations

- Maximum 3 rows
- Maximum 9 windows per row (27 total slots)
- Platform drag (`cx.start_dragging()`) not used due to macOS limitations

## Future Improvements

- Add animation during window movement
- Support for more rows (dynamic row creation)
- Keyboard shortcuts for window arrangement
- Persist layout state
