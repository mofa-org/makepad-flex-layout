//! Panel grid widget - container for draggable panels
//!
//! Manages a grid of Panel widgets with drag-and-drop support.

use std::cell::RefCell;
use makepad_widgets::*;
use crate::panel::PanelAction;
use crate::panel::panel::PanelWidgetExt;
use crate::grid::drop_handler::{DropPosition, calculate_drop_position};
use crate::grid::layout_state::LayoutState;
use crate::theme::get_global_dark_mode;

// Thread-local storage for pending layout state (used when set_layout_state is called before first draw)
thread_local! {
    static PENDING_LAYOUT: RefCell<Option<LayoutState>> = RefCell::new(None);
    static PENDING_RESET: RefCell<bool> = RefCell::new(false);
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Import Panel widget - must use crate path for cross-module visibility
    use crate::panel::panel::Panel;

    // ========================================
    // PANEL GRID WIDGET
    // ========================================

    pub PanelGrid = {{PanelGrid}} {
        width: Fill
        height: Fill
        padding: 0
        cursor: Default

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Light: slate-200, Dark: slate-900
                let light = vec4(0.886, 0.910, 0.941, 1.0);
                let dark = vec4(0.059, 0.090, 0.165, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        // Drop preview overlay
        drop_preview: {
            draw_depth: 10.0
            color: #4080c080
        }

        // Container with explicit row structure for precise layout
        // Each row has 9 slots to allow all panels in one row if desired
        window_container = <View> {
            width: Fill
            height: Fill
            flow: Down

            // Row 1: up to 9 panels
            row1 = <View> {
                width: Fill
                height: Fill
                flow: Right

                s1_1 = <Panel> { width: Fill, height: Fill }
                s1_2 = <Panel> { width: Fill, height: Fill }
                s1_3 = <Panel> { width: Fill, height: Fill }
                s1_4 = <Panel> { width: Fill, height: Fill }
                s1_5 = <Panel> { width: Fill, height: Fill }
                s1_6 = <Panel> { width: Fill, height: Fill }
                s1_7 = <Panel> { width: Fill, height: Fill }
                s1_8 = <Panel> { width: Fill, height: Fill }
                s1_9 = <Panel> { width: Fill, height: Fill }
            }

            // Row 2: up to 9 panels
            row2 = <View> {
                width: Fill
                height: Fill
                flow: Right

                s2_1 = <Panel> { width: Fill, height: Fill }
                s2_2 = <Panel> { width: Fill, height: Fill }
                s2_3 = <Panel> { width: Fill, height: Fill }
                s2_4 = <Panel> { width: Fill, height: Fill }
                s2_5 = <Panel> { width: Fill, height: Fill }
                s2_6 = <Panel> { width: Fill, height: Fill }
                s2_7 = <Panel> { width: Fill, height: Fill }
                s2_8 = <Panel> { width: Fill, height: Fill }
                s2_9 = <Panel> { width: Fill, height: Fill }
            }

            // Row 3: up to 9 panels
            row3 = <View> {
                width: Fill
                height: Fill
                flow: Right

                s3_1 = <Panel> { width: Fill, height: Fill }
                s3_2 = <Panel> { width: Fill, height: Fill }
                s3_3 = <Panel> { width: Fill, height: Fill }
                s3_4 = <Panel> { width: Fill, height: Fill }
                s3_5 = <Panel> { width: Fill, height: Fill }
                s3_6 = <Panel> { width: Fill, height: Fill }
                s3_7 = <Panel> { width: Fill, height: Fill }
                s3_8 = <Panel> { width: Fill, height: Fill }
                s3_9 = <Panel> { width: Fill, height: Fill }
            }
        }
    }
}

// ============================================================================
// PANEL GRID WIDGET IMPLEMENTATION
// ============================================================================

/// Container widget managing a grid of Panel widgets with drag-and-drop support.
///
/// ## Layout Model
/// Uses `LayoutState` with `row_assignments: Vec<Vec<String>>` as the source of truth.
/// Each row maintains its own list of panel IDs, enabling true physical movement
/// of panels between rows.
///
/// ## Slot System
/// Each row has 9 pre-defined slots (s1_1 through s1_9, etc.). Panels are
/// assigned to slots dynamically based on row_assignments. Unused slots are
/// hidden with `width: 0, height: 0`.
#[derive(Live, LiveHook, Widget)]
pub struct PanelGrid {
    #[deref]
    view: View,

    #[live]
    drop_preview: DrawColor,

    /// Maximum number of rows
    #[live]
    max_rows: usize,

    /// Maximum slots per row
    #[live]
    max_slots_per_row: usize,

    #[rust]
    layout_state: LayoutState,

    #[rust]
    initialized: bool,

    #[rust]
    needs_layout_update: bool,

    /// Currently dragging panel ID (semantic string ID)
    #[rust]
    dragging_panel: Option<String>,

    /// Current drop target position
    #[rust]
    drop_state: Option<DropPosition>,
}

/// Helper to convert string panel ID to LiveId
fn panel_id_to_live_id(panel_id: &str) -> LiveId {
    LiveId::from_str_lc(panel_id)
}

impl Widget for PanelGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let mut layout_changed = false;

        // Handle Panel actions
        for action in actions.iter() {
            match action.as_widget_action().cast::<PanelAction>() {
                PanelAction::Close(id) => {
                    // Find panel by LiveId and close it
                    if let Some(panel_id) = self.find_panel_by_live_id(id) {
                        self.close_panel(cx, &panel_id);
                        layout_changed = true;
                    }
                }
                PanelAction::Maximize(id) => {
                    if let Some(panel_id) = self.find_panel_by_live_id(id) {
                        self.toggle_maximize(cx, &panel_id);
                        layout_changed = true;
                    }
                }
                PanelAction::Fullscreen(_) => {
                    // Fullscreen is handled by FooterGrid, not main PanelGrid
                }
                PanelAction::StartDrag(id) => {
                    if let Some(panel_id) = self.find_panel_by_live_id(id) {
                        self.dragging_panel = Some(panel_id);
                    }
                }
                PanelAction::EndDrag(id, abs) => {
                    // Complete the drop operation
                    if let Some(panel_id) = self.find_panel_by_live_id(id) {
                        if self.dragging_panel.as_deref() == Some(&panel_id) {
                            self.handle_drop(cx, abs, &panel_id);
                            layout_changed = true;
                        }
                    }
                    self.dragging_panel = None;
                    self.drop_state = None;
                    self.view.redraw(cx);
                }
                PanelAction::LayoutChanged(_) | PanelAction::FooterLayoutChanged(_) | PanelAction::ResetLayout => {
                    // Ignore - we emit these or handle via thread-local
                }
                PanelAction::None => {}
            }
        }

        // Handle internal drag via hits on the view
        match event.hits_with_capture_overload(cx, self.view.area(), self.dragging_panel.is_some()) {
            Hit::FingerMove(fe) if self.dragging_panel.is_some() => {
                // Update drop preview based on cursor position
                if let Some(pos) = self.find_drop_position(cx, fe.abs) {
                    self.drop_state = Some(pos);
                } else {
                    self.drop_state = None;
                }
                self.view.redraw(cx);
            }
            Hit::FingerUp(_) => {
                // Clear state on any FingerUp as fallback
                // (actual drop is handled via EndDrag action from Panel)
                self.dragging_panel = None;
                self.drop_state = None;
                self.view.redraw(cx);
            }
            _ => {}
        }

        // Emit layout changed action if needed
        if layout_changed {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                PanelAction::LayoutChanged(self.layout_state.clone()),
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Apply global theme on every draw
        let dm = get_global_dark_mode();
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });

        // Check for pending reset
        let should_reset = PENDING_RESET.with(|p| {
            let val = *p.borrow();
            if val { *p.borrow_mut() = false; }
            val
        });
        if should_reset {
            self.layout_state = LayoutState::default();
            self.needs_layout_update = true;
        }

        // Initialize on first draw
        if !self.initialized {
            self.initialized = true;

            // Check for pending layout from set_layout_state called before first draw
            let pending = PENDING_LAYOUT.with(|p| p.borrow_mut().take());
            if let Some(state) = pending {
                self.layout_state = state;
            } else {
                self.layout_state = LayoutState::default();
            }
            self.needs_layout_update = true;
        }

        // Apply layout before drawing
        if self.needs_layout_update {
            self.needs_layout_update = false;
            self.apply_row_layout(cx);
        }

        // Draw the main view
        let result = self.view.draw_walk(cx, scope, walk);

        // Draw drop preview overlay if dragging
        if let Some(ref pos) = self.drop_state {
            self.drop_preview.draw_abs(cx, pos.rect);
        }

        result
    }
}

impl PanelGrid {
    /// Get the current layout state
    pub fn layout_state(&self) -> &LayoutState {
        &self.layout_state
    }

    /// Set layout state (for restoring from persistence)
    pub fn set_layout_state(&mut self, cx: &mut Cx, state: LayoutState) {
        self.layout_state = state;
        self.initialized = true;
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Find panel string ID by LiveId (reverse lookup through visible panels)
    fn find_panel_by_live_id(&self, id: LiveId) -> Option<String> {
        // Check all visible panels for matching LiveId
        for panel_id in &self.layout_state.visible_panels {
            if panel_id_to_live_id(panel_id) == id {
                return Some(panel_id.clone());
            }
        }
        None
    }

    /// Close a panel
    fn close_panel(&mut self, cx: &mut Cx, panel_id: &str) {
        self.layout_state.close_panel(panel_id);
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Toggle maximize state for a panel
    fn toggle_maximize(&mut self, cx: &mut Cx, panel_id: &str) {
        if self.layout_state.maximized_panel.as_deref() == Some(panel_id) {
            self.layout_state.maximized_panel = None;
        } else {
            self.layout_state.maximized_panel = Some(panel_id.to_string());
        }
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Find the drop position based on cursor location
    fn find_drop_position(&self, cx: &Cx, abs: DVec2) -> Option<DropPosition> {
        // Get visible panels per row
        let rows_with_panels: Vec<Vec<String>> = (0..3)
            .map(|r| self.layout_state.visible_in_row(r))
            .filter(|row| !row.is_empty())
            .collect();

        // Build mapping from visual row to actual row
        let mut row_to_actual = Vec::new();
        for r in 0..3 {
            if !self.layout_state.visible_in_row(r).is_empty() {
                row_to_actual.push(r);
            }
        }

        // Get the container rect
        let container = self.view.view(id!(window_container));
        let container_rect = container.area().rect(cx);

        calculate_drop_position(abs, container_rect, &rows_with_panels, &row_to_actual)
    }

    /// Handle a drop operation - move panel to new row/position
    fn handle_drop(&mut self, cx: &mut Cx, abs: DVec2, dragged_panel_id: &str) {
        let Some(drop_pos) = self.find_drop_position(cx, abs) else {
            return;
        };

        self.layout_state.move_panel(dragged_panel_id, drop_pos.row, drop_pos.col);
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Get panel index from panel ID (extracts number from "panel_N" format)
    fn panel_index_from_id(panel_id: &str) -> usize {
        // Try to extract index from "panel_N" format
        if let Some(suffix) = panel_id.strip_prefix("panel_") {
            if let Ok(idx) = suffix.parse::<usize>() {
                return idx;
            }
        }
        // Fallback: hash the string to get a consistent index
        let mut hash: usize = 0;
        for byte in panel_id.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
        }
        hash % 9
    }

    /// Apply row-based layout using visibility and Fill sizing
    fn apply_row_layout(&mut self, cx: &mut Cx) {
        // Slot IDs organized by row (9 slots per row)
        let row_slot_ids = [
            [
                id!(window_container.row1.s1_1),
                id!(window_container.row1.s1_2),
                id!(window_container.row1.s1_3),
                id!(window_container.row1.s1_4),
                id!(window_container.row1.s1_5),
                id!(window_container.row1.s1_6),
                id!(window_container.row1.s1_7),
                id!(window_container.row1.s1_8),
                id!(window_container.row1.s1_9),
            ],
            [
                id!(window_container.row2.s2_1),
                id!(window_container.row2.s2_2),
                id!(window_container.row2.s2_3),
                id!(window_container.row2.s2_4),
                id!(window_container.row2.s2_5),
                id!(window_container.row2.s2_6),
                id!(window_container.row2.s2_7),
                id!(window_container.row2.s2_8),
                id!(window_container.row2.s2_9),
            ],
            [
                id!(window_container.row3.s3_1),
                id!(window_container.row3.s3_2),
                id!(window_container.row3.s3_3),
                id!(window_container.row3.s3_4),
                id!(window_container.row3.s3_5),
                id!(window_container.row3.s3_6),
                id!(window_container.row3.s3_7),
                id!(window_container.row3.s3_8),
                id!(window_container.row3.s3_9),
            ],
        ];

        let row_view_ids = [
            id!(window_container.row1),
            id!(window_container.row2),
            id!(window_container.row3),
        ];

        // Get visible panels per row
        let visible_per_row: [Vec<String>; 3] = [
            self.layout_state.visible_in_row(0),
            self.layout_state.visible_in_row(1),
            self.layout_state.visible_in_row(2),
        ];

        let total_visible: usize = visible_per_row.iter().map(|r| r.len()).sum();

        const SLOTS_PER_ROW: usize = 9;

        // Handle maximized panel
        if let Some(ref max_id) = self.layout_state.maximized_panel.clone() {
            // Hide all slots and rows first
            for row_idx in 0..3 {
                for slot_idx in 0..SLOTS_PER_ROW {
                    self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                        visible: false, width: 0, height: 0
                    });
                }
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: false, height: 0
                });
            }

            // Find which row contains the maximized panel
            if let Some((row_idx, _)) = self.layout_state.find_panel_row(max_id) {
                // Show only that row
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: true, height: Fill
                });

                // Show only the maximized panel's slot (use first slot in that row)
                self.view.view(row_slot_ids[row_idx][0]).apply_over(cx, live! {
                    visible: true, width: Fill, height: Fill
                });
                self.view.panel(row_slot_ids[row_idx][0]).set_panel_id_str(max_id);
                self.view.panel(row_slot_ids[row_idx][0]).set_panel_index(cx, Self::panel_index_from_id(max_id));
                self.view.panel(row_slot_ids[row_idx][0]).set_maximized(true);
            }
            return;
        }

        // Auto-maximize if only 1 panel left
        if total_visible == 1 {
            // Hide all first
            for row_idx in 0..3 {
                for slot_idx in 0..SLOTS_PER_ROW {
                    self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                        visible: false, width: 0, height: 0
                    });
                }
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: false, height: 0
                });
            }

            // Find the only visible panel
            for row_idx in 0..3 {
                if !visible_per_row[row_idx].is_empty() {
                    let panel_id = &visible_per_row[row_idx][0];
                    self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                        visible: true, height: Fill
                    });
                    self.view.view(row_slot_ids[row_idx][0]).apply_over(cx, live! {
                        visible: true, width: Fill, height: Fill
                    });
                    self.view.panel(row_slot_ids[row_idx][0]).set_panel_id_str(panel_id);
                    self.view.panel(row_slot_ids[row_idx][0]).set_panel_index(cx, Self::panel_index_from_id(panel_id));
                    break;
                }
            }
            return;
        }

        // Normal layout: each row shows its assigned panels
        // First hide all slots
        for row_idx in 0..3 {
            for slot_idx in 0..SLOTS_PER_ROW {
                self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                    visible: false, width: 0, height: 0
                });
                self.view.panel(row_slot_ids[row_idx][slot_idx]).set_maximized(false);
            }
        }

        // Configure each row
        for row_idx in 0..3 {
            let panels_in_row = &visible_per_row[row_idx];

            if panels_in_row.is_empty() {
                // Hide empty rows
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: false, height: 0
                });
            } else {
                // Show row
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: true, height: Fill
                });

                // Show slots for panels in this row
                for (slot_idx, panel_id) in panels_in_row.iter().take(SLOTS_PER_ROW).enumerate() {
                    self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                        visible: true, width: Fill, height: Fill
                    });
                    self.view.panel(row_slot_ids[row_idx][slot_idx]).set_panel_id_str(panel_id);
                    self.view.panel(row_slot_ids[row_idx][slot_idx]).set_panel_index(cx, Self::panel_index_from_id(panel_id));
                }
            }
        }
    }
}

impl PanelGridRef {
    /// Get the current layout state
    pub fn layout_state(&self) -> Option<LayoutState> {
        self.borrow().map(|inner| inner.layout_state.clone())
    }

    /// Set layout state (for restoring from persistence)
    ///
    /// Note: If called before first draw, stores the state to be applied during initialization.
    pub fn set_layout_state(&self, cx: &mut Cx, state: LayoutState) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_layout_state(cx, state);
        } else {
            // Store in thread-local for retrieval during first draw
            PENDING_LAYOUT.with(|p| *p.borrow_mut() = Some(state));
        }
    }

    /// Reset layout to default state
    pub fn reset_layout(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.layout_state = LayoutState::default();
            inner.needs_layout_update = true;
            inner.view.redraw(cx);
        } else {
            // Store reset flag for retrieval during next draw
            PENDING_RESET.with(|p| *p.borrow_mut() = true);
            cx.redraw_all();
        }
    }

    /// Apply dark mode value to this grid and all panels
    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply to grid background
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply to all panels in all slots
            let slot_ids = [
                // Row 1
                id!(window_container.row1.s1_1), id!(window_container.row1.s1_2),
                id!(window_container.row1.s1_3), id!(window_container.row1.s1_4),
                id!(window_container.row1.s1_5), id!(window_container.row1.s1_6),
                id!(window_container.row1.s1_7), id!(window_container.row1.s1_8),
                id!(window_container.row1.s1_9),
                // Row 2
                id!(window_container.row2.s2_1), id!(window_container.row2.s2_2),
                id!(window_container.row2.s2_3), id!(window_container.row2.s2_4),
                id!(window_container.row2.s2_5), id!(window_container.row2.s2_6),
                id!(window_container.row2.s2_7), id!(window_container.row2.s2_8),
                id!(window_container.row2.s2_9),
                // Row 3
                id!(window_container.row3.s3_1), id!(window_container.row3.s3_2),
                id!(window_container.row3.s3_3), id!(window_container.row3.s3_4),
                id!(window_container.row3.s3_5), id!(window_container.row3.s3_6),
                id!(window_container.row3.s3_7), id!(window_container.row3.s3_8),
                id!(window_container.row3.s3_9),
            ];

            for slot_id in &slot_ids {
                inner.view.panel(*slot_id).apply_dark_mode(cx, dark_mode);
            }
        }
    }
}
