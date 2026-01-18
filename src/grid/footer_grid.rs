//! Footer grid widget - horizontal panel strip with controller sidebar
//!
//! Provides a row of panels for the footer area with a resizable left controller sidebar.
//! Supports drag-and-drop between panels with vertical splitting.
//!
//! ## Features
//! - Resizable controller sidebar (independent splitter)
//! - Drag panels to merge (vertical split)
//! - Fullscreen mode (panel takes entire footer dock)
//! - Close panels
//!
//! ## Addressing Scheme
//! - `{0,0}`: Controller sidebar
//! - `{1,0}` to `{1,6}`: Panel slots (can be single or vertically split)

use std::cell::RefCell;
use makepad_widgets::*;
use crate::panel::PanelAction;
use crate::panel::panel::PanelWidgetRefExt;
use crate::shell::sidebar::ShellSidebarWidgetExt;
use crate::grid::{FooterLayoutState, FooterSlotState};

// Thread-local storage for pending footer layout state (used when set_layout_state is called before first draw)
thread_local! {
    static PENDING_FOOTER_LAYOUT: RefCell<Option<FooterLayoutState>> = RefCell::new(None);
    static PENDING_FOOTER_RESET: RefCell<bool> = RefCell::new(false);
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::panel::panel::Panel;
    use crate::shell::sidebar::ShellSidebar;

    // A slot that can hold multiple vertically stacked panels (up to 5)
    FooterSlot = <View> {
        width: Fill
        height: Fill
        flow: Down
        spacing: 0

        show_bg: false

        // Panel slots (p0 at top, up to p4 at bottom)
        p0 = <Panel> {
            width: Fill, height: Fill
            closable: true
            maximizable: false
            fullscreenable: true
        }
        p1 = <Panel> {
            visible: false
            width: Fill, height: 0
            closable: true
            maximizable: false
            fullscreenable: true
        }
        p2 = <Panel> {
            visible: false
            width: Fill, height: 0
            closable: true
            maximizable: false
            fullscreenable: true
        }
        p3 = <Panel> {
            visible: false
            width: Fill, height: 0
            closable: true
            maximizable: false
            fullscreenable: true
        }
        p4 = <Panel> {
            visible: false
            width: Fill, height: 0
            closable: true
            maximizable: false
            fullscreenable: true
        }
    }

    // Thin splitter for footer with light colors
    FooterThinSplitter = <Splitter> {
        size: 1.0
        draw_bg: {
            color: vec4(0.886, 0.910, 0.941, 1.0)     // slate-200 (light)
            color_hover: vec4(0.384, 0.514, 0.965, 1.0)  // blue-500 (highlight)
            color_drag: vec4(0.231, 0.400, 0.900, 1.0)   // blue-600

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Background changes on hover
                let bg_normal = vec4(0.945, 0.961, 0.976, 1.0);  // slate-100
                let bg_hover = vec4(0.925, 0.937, 0.976, 1.0);   // slight blue tint
                sdf.clear(mix(bg_normal, bg_hover, self.hover));

                if self.is_vertical > 0.5 {
                    sdf.box(
                        self.splitter_pad,
                        self.rect_size.y * 0.5 - self.size * 0.5,
                        self.rect_size.x - 2.0 * self.splitter_pad,
                        self.size,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * 0.5 - self.size * 0.5,
                        self.splitter_pad,
                        self.size,
                        self.rect_size.y - 2.0 * self.splitter_pad,
                        self.border_radius
                    );
                }

                return sdf.fill_keep(
                    mix(
                        self.color,
                        mix(
                            self.color_hover,
                            self.color_drag,
                            self.drag
                        ),
                        self.hover
                    )
                );
            }
        }
    }

    pub FooterGrid = {{FooterGrid}} {
        width: Fill
        height: Fill

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let light = vec4(0.945, 0.961, 0.976, 1.0);
                let dark = vec4(0.122, 0.161, 0.231, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        // Drop preview overlay
        drop_preview: {
            draw_depth: 10.0
            color: #3b82f680
        }

        // Use Dock with independent horizontal splitter
        dock = <Dock> {
            width: Fill
            height: Fill
            padding: 0

            // Use thin splitter
            splitter: <FooterThinSplitter> {}

            // No corner radius
            round_corner: {
                border_radius: 0.0
            }

            root = Splitter {
                axis: Horizontal
                align: FromA(200.0)
                a: controller_tab
                b: panel_strip_tab
            }

            controller_tab = Tab {
                name: ""
                kind: controller_content
            }

            panel_strip_tab = Tab {
                name: ""
                kind: panel_strip_content
            }

            controller_content = <ShellSidebar> {
                title: "Timeline"
            }

            panel_strip_content = <View> {
                width: Fill
                height: Fill
                flow: Right
                padding: 0
                spacing: 0

                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let light = vec4(0.886, 0.910, 0.941, 1.0);
                        let dark = vec4(0.059, 0.090, 0.165, 1.0);
                        return mix(light, dark, self.dark_mode);
                    }
                }

                f1_0 = <FooterSlot> {}
                f1_1 = <FooterSlot> {}
                f1_2 = <FooterSlot> {}
                f1_3 = <FooterSlot> {}
                f1_4 = <FooterSlot> {}
                f1_5 = <FooterSlot> {}
                f1_6 = <FooterSlot> {}
            }
        }
    }
}

/// Slot state - holds one or more vertically stacked panels
#[derive(Clone, Debug, Default)]
pub struct SlotState {
    /// Is this slot visible?
    pub visible: bool,
    /// Panel IDs stacked vertically (top to bottom)
    pub panel_ids: Vec<String>,
}

/// Helper to convert string panel ID to LiveId
fn panel_id_to_live_id(panel_id: &str) -> LiveId {
    LiveId::from_str_lc(panel_id)
}

/// Get panel index from panel ID (extracts number from "footer_panel_N" format)
fn panel_index_from_id(panel_id: &str) -> usize {
    // Try to extract index from "footer_panel_N" format
    if let Some(suffix) = panel_id.strip_prefix("footer_panel_") {
        if let Ok(idx) = suffix.parse::<usize>() {
            return idx;
        }
    }
    // Fallback: hash the string to get a consistent index
    let mut hash: usize = 0;
    for byte in panel_id.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
    }
    hash % 7
}

/// Footer grid widget with resizable controller sidebar and horizontal panel strip.
#[derive(Live, LiveHook, Widget)]
pub struct FooterGrid {
    #[deref]
    view: View,

    #[live]
    drop_preview: DrawColor,

    /// Number of initially visible panels (default: 3)
    #[live(3i64)]
    initial_panels: i64,

    /// Slot states (visibility, split state, panel IDs)
    #[rust]
    slots: Vec<SlotState>,

    /// Currently fullscreen panel ID (None if no fullscreen)
    #[rust]
    fullscreen_panel: Option<String>,

    /// Currently dragging panel ID
    #[rust]
    dragging_panel: Option<String>,

    /// Current drop target (slot index, is_bottom_half)
    #[rust]
    drop_target: Option<(usize, bool)>,

    #[rust]
    initialized: bool,

    #[rust]
    needs_layout_update: bool,
}

impl Widget for FooterGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let mut layout_changed = false;

        // Handle Panel actions
        for action in actions.iter() {
            match action.as_widget_action().cast::<PanelAction>() {
                PanelAction::Close(id) => {
                    if let Some(panel_id) = self.find_panel_by_live_id(id) {
                        self.close_panel(cx, &panel_id);
                        layout_changed = true;
                    }
                }
                PanelAction::Fullscreen(id) => {
                    if let Some(panel_id) = self.find_panel_by_live_id(id) {
                        self.toggle_fullscreen(cx, &panel_id);
                        layout_changed = true;
                    }
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
                            self.update_drop_target(cx, abs);
                            self.handle_drop(cx, &panel_id, abs);
                            layout_changed = true;
                        }
                    }
                    self.dragging_panel = None;
                    self.drop_target = None;
                    self.view.redraw(cx);
                }
                PanelAction::Maximize(_) => {}
                PanelAction::LayoutChanged(_) | PanelAction::FooterLayoutChanged(_) | PanelAction::ResetLayout => {
                    // Ignore - we emit these or handle via thread-local
                }
                PanelAction::None => {}
            }
        }

        // Handle drag-and-drop
        if self.dragging_panel.is_some() {
            match event.hits_with_capture_overload(cx, self.view.area(), true) {
                Hit::FingerMove(fe) => {
                    self.update_drop_target(cx, fe.abs);
                    self.view.redraw(cx);
                }
                Hit::FingerUp(_) => {
                    // Clear state on any FingerUp as fallback
                    // (actual drop is handled via EndDrag action from Panel)
                    self.dragging_panel = None;
                    self.drop_target = None;
                    self.view.redraw(cx);
                }
                _ => {}
            }
        }

        // Emit layout changed action if needed
        if layout_changed {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                PanelAction::FooterLayoutChanged(self.get_layout_state()),
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Check for pending reset
        let should_reset = PENDING_FOOTER_RESET.with(|p| {
            let val = *p.borrow();
            if val { *p.borrow_mut() = false; }
            val
        });
        if should_reset {
            self.initialize_slots();
            self.fullscreen_panel = None;
            self.needs_layout_update = true;
        }

        if !self.initialized {
            self.initialized = true;

            // Check for pending layout from set_layout_state called before first draw
            let pending = PENDING_FOOTER_LAYOUT.with(|p| p.borrow_mut().take());
            if let Some(state) = pending {
                self.slots = state.slots.into_iter().map(|s| SlotState {
                    visible: s.visible,
                    panel_ids: s.panel_ids,
                }).collect();
                self.fullscreen_panel = state.fullscreen_panel;
            } else {
                self.initialize_slots();
            }
            self.needs_layout_update = true;
        }

        if self.needs_layout_update {
            self.needs_layout_update = false;
            self.apply_layout(cx);
        }

        let result = self.view.draw_walk(cx, scope, walk);

        // Draw drop preview if dragging
        if let Some((slot_idx, is_bottom)) = self.drop_target {
            if let Some(rect) = self.get_slot_drop_rect(cx, slot_idx, is_bottom) {
                self.drop_preview.draw_abs(cx, rect);
            }
        }

        result
    }
}

impl FooterGrid {
    const SLOT_COUNT: usize = 7;

    fn slot_ids() -> [&'static [LiveId]; 7] {
        [id!(f1_0), id!(f1_1), id!(f1_2), id!(f1_3), id!(f1_4), id!(f1_5), id!(f1_6)]
    }

    fn initialize_slots(&mut self) {
        let count = self.initial_panels.max(0) as usize;
        self.slots = (0..Self::SLOT_COUNT)
            .map(|i| SlotState {
                visible: i < count,
                panel_ids: vec![format!("footer_panel_{}", i)],
            })
            .collect();
    }

    fn panel_slot_ids() -> [&'static [LiveId]; 5] {
        [id!(p0), id!(p1), id!(p2), id!(p3), id!(p4)]
    }

    /// Find panel string ID by LiveId (reverse lookup through all slot panels)
    fn find_panel_by_live_id(&self, id: LiveId) -> Option<String> {
        for slot in &self.slots {
            for panel_id in &slot.panel_ids {
                if panel_id_to_live_id(panel_id) == id {
                    return Some(panel_id.clone());
                }
            }
        }
        None
    }

    fn apply_layout(&mut self, cx: &mut Cx) {
        let slot_ids = Self::slot_ids();

        // Handle fullscreen mode
        if let Some(ref fs_id) = self.fullscreen_panel.clone() {
            // Hide all slots
            for (i, slot_id) in slot_ids.iter().enumerate() {
                self.view.view(*slot_id).apply_over(cx, live! {
                    visible: false, width: 0, height: 0
                });

                // Find and show only the fullscreen panel
                if let Some(slot) = self.slots.get(i) {
                    if slot.panel_ids.contains(&fs_id) {
                        self.view.view(*slot_id).apply_over(cx, live! {
                            visible: true, width: Fill, height: Fill
                        });
                        // Configure as single panel in fullscreen
                        self.configure_slot(cx, *slot_id, &[fs_id.clone()], true);
                    }
                }
            }
            return;
        }

        // Normal layout
        for (i, slot_id) in slot_ids.iter().enumerate() {
            if let Some(slot) = self.slots.get(i) {
                if !slot.visible || slot.panel_ids.is_empty() {
                    self.view.view(*slot_id).apply_over(cx, live! {
                        visible: false, width: 0, height: 0
                    });
                    continue;
                }

                self.view.view(*slot_id).apply_over(cx, live! {
                    visible: true, width: Fill, height: Fill
                });

                // Clone panel_ids to avoid borrow conflict
                let panel_ids = slot.panel_ids.clone();
                self.configure_slot(cx, *slot_id, &panel_ids, false);
            }
        }
    }

    fn configure_slot(&mut self, cx: &mut Cx, slot_id: &[LiveId], panel_ids: &[String], is_fullscreen: bool) {
        let panel_slot_ids = Self::panel_slot_ids();
        let count = panel_ids.len().min(5);

        for (i, p_slot_id) in panel_slot_ids.iter().enumerate() {
            if i < count {
                // Show this panel
                self.view.view(slot_id).view(*p_slot_id).apply_over(cx, live! {
                    visible: true, width: Fill, height: Fill
                });
                let panel_ref = self.view.view(slot_id).panel(*p_slot_id);
                panel_ref.set_panel_id_str(&panel_ids[i]);
                panel_ref.set_panel_index(cx, panel_index_from_id(&panel_ids[i]));
                panel_ref.set_fullscreen(is_fullscreen && count == 1);
            } else {
                // Hide unused panel slots
                self.view.view(slot_id).view(*p_slot_id).apply_over(cx, live! {
                    visible: false, width: Fill, height: 0
                });
            }
        }
    }

    fn close_panel(&mut self, cx: &mut Cx, panel_id: &str) {
        // Exit fullscreen if closing fullscreen panel
        if self.fullscreen_panel.as_deref() == Some(panel_id) {
            self.fullscreen_panel = None;
        }

        for slot in &mut self.slots {
            if let Some(pos) = slot.panel_ids.iter().position(|id| id == panel_id) {
                slot.panel_ids.remove(pos);
                if slot.panel_ids.is_empty() {
                    slot.visible = false;
                }
                break;
            }
        }

        // Auto-compact after closing
        self.compact_slots();

        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    fn toggle_fullscreen(&mut self, cx: &mut Cx, panel_id: &str) {
        if self.fullscreen_panel.as_deref() == Some(panel_id) {
            self.fullscreen_panel = None;
        } else {
            self.fullscreen_panel = Some(panel_id.to_string());
        }
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    fn update_drop_target(&mut self, cx: &Cx, abs: DVec2) {
        self.drop_target = None;
        let slot_ids = Self::slot_ids();

        for (i, slot_id) in slot_ids.iter().enumerate() {
            if let Some(slot) = self.slots.get(i) {
                if !slot.visible || slot.panel_ids.is_empty() {
                    continue;
                }

                // Skip if dragging a panel that's already in this slot
                if let Some(ref dragging) = self.dragging_panel {
                    if slot.panel_ids.contains(dragging) {
                        continue;
                    }
                }

                let slot_view = self.view.view(*slot_id);
                let rect = slot_view.area().rect(cx);

                if rect.contains(abs) {
                    // Determine if dropping on top or bottom half
                    let mid_y = rect.pos.y + rect.size.y / 2.0;
                    let is_bottom = abs.y > mid_y;
                    self.drop_target = Some((i, is_bottom));
                    return;
                }
            }
        }
    }

    fn get_slot_drop_rect(&self, cx: &Cx, slot_idx: usize, is_bottom: bool) -> Option<Rect> {
        let slot_ids = Self::slot_ids();
        if slot_idx >= slot_ids.len() {
            return None;
        }

        let slot_view = self.view.view(slot_ids[slot_idx]);
        let rect = slot_view.area().rect(cx);

        if rect.size.x <= 0.0 || rect.size.y <= 0.0 {
            return None;
        }

        // Show preview on top or bottom half
        let half_height = rect.size.y / 2.0;
        Some(Rect {
            pos: DVec2 {
                x: rect.pos.x,
                y: if is_bottom { rect.pos.y + half_height } else { rect.pos.y },
            },
            size: DVec2 {
                x: rect.size.x,
                y: half_height,
            },
        })
    }

    fn handle_drop(&mut self, cx: &mut Cx, dragged_id: &str, _abs: DVec2) {
        let Some((target_idx, is_bottom)) = self.drop_target else {
            return;
        };

        // Find the source slot for the dragged panel
        let mut source_slot_idx: Option<usize> = None;

        for (i, slot) in self.slots.iter().enumerate() {
            if slot.panel_ids.iter().any(|id| id == dragged_id) {
                source_slot_idx = Some(i);
                break;
            }
        }

        // If dragging from within footer, remove from source slot
        if let Some(src_idx) = source_slot_idx {
            // Skip if dropping on same slot
            if src_idx == target_idx {
                return;
            }

            // Remove from source slot
            let src_slot = &mut self.slots[src_idx];
            if let Some(pos) = src_slot.panel_ids.iter().position(|id| id == dragged_id) {
                src_slot.panel_ids.remove(pos);
            }
            if src_slot.panel_ids.is_empty() {
                src_slot.visible = false;
            }
        }

        // Add to target slot (max 5 panels per slot)
        let target_slot = &mut self.slots[target_idx];
        if target_slot.panel_ids.len() < 5 {
            if is_bottom {
                target_slot.panel_ids.push(dragged_id.to_string());
            } else {
                target_slot.panel_ids.insert(0, dragged_id.to_string());
            }
        }

        // Auto-compact: shift visible slots to fill gaps
        self.compact_slots();

        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Compact slots by shifting visible ones to fill gaps
    fn compact_slots(&mut self) {
        // Collect visible slots with panels
        let visible_slots: Vec<SlotState> = self.slots.iter()
            .filter(|s| s.visible && !s.panel_ids.is_empty())
            .cloned()
            .collect();

        // Reassign slots
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if i < visible_slots.len() {
                *slot = visible_slots[i].clone();
            } else {
                slot.visible = false;
                slot.panel_ids.clear();
            }
        }
    }

    /// Set which panels are visible (by slot index)
    pub fn set_visible_panels(&mut self, cx: &mut Cx, count: usize) {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            slot.visible = i < count;
        }
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Get current layout state for persistence
    pub fn get_layout_state(&self) -> FooterLayoutState {
        FooterLayoutState {
            slots: self.slots.iter().map(|s| FooterSlotState {
                visible: s.visible,
                panel_ids: s.panel_ids.clone(),
            }).collect(),
            fullscreen_panel: self.fullscreen_panel.clone(),
        }
    }

    /// Set layout state from persistence
    pub fn set_layout_state(&mut self, cx: &mut Cx, state: FooterLayoutState) {
        self.slots = state.slots.into_iter().map(|s| SlotState {
            visible: s.visible,
            panel_ids: s.panel_ids,
        }).collect();
        self.fullscreen_panel = state.fullscreen_panel;
        self.initialized = true;
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }
}

impl FooterGridRef {
    pub fn set_visible_panels(&self, cx: &mut Cx, count: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_visible_panels(cx, count);
        }
    }

    /// Set layout state from persistence
    ///
    /// Note: If called before first draw, stores the state to be applied during initialization.
    pub fn set_layout_state(&self, cx: &mut Cx, state: FooterLayoutState) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_layout_state(cx, state);
        } else {
            // Store in thread-local for retrieval during first draw
            PENDING_FOOTER_LAYOUT.with(|p| *p.borrow_mut() = Some(state));
        }
    }

    /// Reset layout to default state
    pub fn reset_layout(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_slots();
            inner.fullscreen_panel = None;
            inner.dragging_panel = None;
            inner.drop_target = None;
            inner.needs_layout_update = true;
            inner.view.redraw(cx);
        } else {
            // Store reset flag for retrieval during next draw
            PENDING_FOOTER_RESET.with(|p| *p.borrow_mut() = true);
            cx.redraw_all();
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            inner.view.view(id!(panel_strip_content)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            inner.view.shell_sidebar(id!(controller_content)).apply_dark_mode(cx, dark_mode);

            // Apply to all panel slots (p0-p4 in each slot)
            let slot_ids = FooterGrid::slot_ids();
            let panel_slot_ids = FooterGrid::panel_slot_ids();
            for slot_id in &slot_ids {
                for p_slot_id in &panel_slot_ids {
                    inner.view.view(*slot_id).panel(*p_slot_id).apply_dark_mode(cx, dark_mode);
                }
            }
        }
    }
}
