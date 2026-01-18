//! Sidebar menu item widget with hover and click effects
//!
//! Provides animated menu items for the sidebar with:
//! - Hover overlay effect (150ms smooth transition)
//! - Press effect (100ms)
//! - Selection highlight
//! - Dark mode support

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Color constants (Tailwind-inspired)
    // Light mode: slate-50 base, slate-200 hover, blue-100 selected
    // Dark mode: slate-800 base, slate-700 hover, blue-900 selected

    /// Menu item with hover animation
    pub SidebarMenuItem = {{SidebarMenuItem}} {
        width: Fill
        height: 36
        padding: { left: 12, right: 12 }
        align: { y: 0.5 }
        cursor: Hand

        animator: {
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { pressed: 1.0 } }
                }
            }
            selected = {
                default: off,
                off = {
                    from: { all: Snap }
                    apply: { draw_bg: { selected: 0.0 } }
                }
                on = {
                    from: { all: Snap }
                    apply: { draw_bg: { selected: 1.0 } }
                }
            }
        }

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                // Base colors
                let light_base = vec4(0.973, 0.980, 0.988, 1.0);    // slate-50
                let dark_base = vec4(0.122, 0.161, 0.231, 1.0);     // slate-800

                // Hover colors
                let light_hover = vec4(0.886, 0.910, 0.941, 1.0);   // slate-200
                let dark_hover = vec4(0.192, 0.231, 0.302, 1.0);    // slate-700

                // Selected colors (blue tint)
                let light_selected = vec4(0.859, 0.914, 0.988, 1.0); // blue-100
                let dark_selected = vec4(0.118, 0.227, 0.392, 1.0);  // blue-900

                // Pressed colors (slightly darker than hover)
                let light_pressed = vec4(0.792, 0.835, 0.890, 1.0);  // slate-300
                let dark_pressed = vec4(0.259, 0.298, 0.369, 1.0);   // slate-600

                // Calculate base color based on dark mode
                let base = mix(light_base, dark_base, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let selected_color = mix(light_selected, dark_selected, self.dark_mode);
                let pressed_color = mix(light_pressed, dark_pressed, self.dark_mode);

                // Layer the effects: base → selected → hover → pressed
                let color = mix(base, selected_color, self.selected);
                let color = mix(color, hover_color, self.hover * (1.0 - self.selected));
                let color = mix(color, pressed_color, self.pressed);

                // Rounded corners
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(color);

                return sdf.result;
            }
        }

        // Icon placeholder (optional)
        icon = <View> {
            width: 20, height: 20
            align: { x: 0.5, y: 0.5 }
            visible: false
        }

        // Label
        label = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                instance selected: 0.0
                text_style: <FONT_REGULAR> { font_size: 11.0 }

                fn get_color(self) -> vec4 {
                    // Normal text colors
                    let light_text = vec4(0.247, 0.282, 0.333, 1.0);    // gray-700
                    let dark_text = vec4(0.886, 0.910, 0.941, 1.0);     // slate-200

                    // Selected text colors (slightly blue tinted)
                    let light_selected = vec4(0.110, 0.329, 0.651, 1.0); // blue-700
                    let dark_selected = vec4(0.573, 0.773, 0.988, 1.0);  // blue-300

                    let normal = mix(light_text, dark_text, self.dark_mode);
                    let selected = mix(light_selected, dark_selected, self.dark_mode);

                    return mix(normal, selected, self.selected);
                }
            }
            text: "Menu Item"
        }
    }

    /// Show More/Less button with expansion arrow
    pub ShowMoreButton = {{ShowMoreButton}} {
        width: Fill
        height: 32
        padding: { left: 12, right: 12 }
        align: { y: 0.5 }
        cursor: Hand

        animator: {
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
            expanded = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: { draw_arrow: { rotation: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: { draw_arrow: { rotation: 1.0 } }
                }
            }
        }

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let light_base = vec4(0.973, 0.980, 0.988, 0.0);    // transparent
                let dark_base = vec4(0.122, 0.161, 0.231, 0.0);

                let light_hover = vec4(0.918, 0.937, 0.957, 1.0);   // slate-100
                let dark_hover = vec4(0.153, 0.192, 0.263, 1.0);    // slate-750

                let base = mix(light_base, dark_base, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);

                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(mix(base, hover_color, self.hover));

                return sdf.result;
            }
        }

        // Arrow indicator
        arrow = <View> {
            width: 16, height: 16
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance rotation: 0.0  // 0 = down, 1 = up
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                    // Arrow color
                    let light_color = vec4(0.392, 0.455, 0.545, 1.0);  // slate-500
                    let dark_color = vec4(0.573, 0.627, 0.702, 1.0);   // slate-400
                    let color = mix(light_color, dark_color, self.dark_mode);

                    // Draw chevron (rotates between down and up)
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;
                    let size = 4.0;

                    // Rotation: 0 = pointing down, 1 = pointing up
                    let dir = mix(1.0, -1.0, self.rotation);

                    sdf.move_to(cx - size, cy - size * 0.5 * dir);
                    sdf.line_to(cx, cy + size * 0.5 * dir);
                    sdf.line_to(cx + size, cy - size * 0.5 * dir);
                    sdf.stroke(color, 1.5);

                    return sdf.result;
                }
            }
        }

        label = <Label> {
            width: Fill
            margin: { left: 4 }
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR> { font_size: 10.0 }

                fn get_color(self) -> vec4 {
                    let light = vec4(0.392, 0.455, 0.545, 1.0);  // slate-500
                    let dark = vec4(0.573, 0.627, 0.702, 1.0);   // slate-400
                    return mix(light, dark, self.dark_mode);
                }
            }
            text: "Show More"
        }
    }

    /// Expandable container for additional menu items
    pub ExpandableSection = {{ExpandableSection}} {
        width: Fill
        height: 0  // Starts collapsed
        clip_x: true
        clip_y: true

        flow: Down

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
        }
    }
}

// ============================================================================
// SIDEBAR MENU ITEM WIDGET
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct SidebarMenuItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,

    #[live]
    item_id: LiveId,

    #[rust]
    selected: bool,
}

impl Widget for SidebarMenuItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerDown(_) => {
                self.animator_play(cx, id!(pressed.on));
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, id!(pressed.off));
                if fe.is_over {
                    // Emit selection action
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        SidebarMenuAction::ItemClicked(self.item_id),
                    );
                }
            }
            _ => {}
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SidebarMenuItem {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: bool) {
        self.selected = selected;
        if selected {
            self.animator_play(cx, id!(selected.on));
        } else {
            self.animator_play(cx, id!(selected.off));
        }
    }

    pub fn apply_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.label(id!(label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
    }
}

impl SidebarMenuItemRef {
    pub fn set_selected(&self, cx: &mut Cx, selected: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected(cx, selected);
        }
    }

    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(inner) = self.borrow_mut() {
            inner.view.label(id!(label)).set_text(cx, text);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.apply_dark_mode(cx, dark_mode);
        }
    }
}

// ============================================================================
// SHOW MORE BUTTON WIDGET
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct ShowMoreButton {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,

    #[rust]
    expanded: bool,
}

impl Widget for ShowMoreButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    self.toggle_expanded(cx);
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        SidebarMenuAction::ToggleExpand(self.expanded),
                    );
                }
            }
            _ => {}
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShowMoreButton {
    pub fn toggle_expanded(&mut self, cx: &mut Cx) {
        self.expanded = !self.expanded;
        if self.expanded {
            self.animator_play(cx, id!(expanded.on));
            self.view.label(id!(label)).set_text(cx, "Show Less");
        } else {
            self.animator_play(cx, id!(expanded.off));
            self.view.label(id!(label)).set_text(cx, "Show More");
        }
    }

    pub fn set_expanded(&mut self, cx: &mut Cx, expanded: bool) {
        self.expanded = expanded;
        if expanded {
            self.animator_play(cx, id!(expanded.on));
            self.view.label(id!(label)).set_text(cx, "Show Less");
        } else {
            self.animator_play(cx, id!(expanded.off));
            self.view.label(id!(label)).set_text(cx, "Show More");
        }
    }

    pub fn apply_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.view(id!(arrow)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.label(id!(label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });
    }
}

impl ShowMoreButtonRef {
    pub fn set_expanded(&self, cx: &mut Cx, expanded: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_expanded(cx, expanded);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.apply_dark_mode(cx, dark_mode);
        }
    }
}

// ============================================================================
// EXPANDABLE SECTION WIDGET
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct ExpandableSection {
    #[deref]
    view: View,

    #[rust]
    expanded: bool,

    #[rust]
    animation_start: f64,

    #[rust]
    animating: bool,

    #[rust]
    target_height: f64,
}

impl Widget for ExpandableSection {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Handle expansion animation
        if self.animating {
            if let Event::NextFrame(_) = event {
                self.update_animation(cx);
            }
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

const EXPAND_DURATION: f64 = 0.2; // 200ms

impl ExpandableSection {
    pub fn set_expanded(&mut self, cx: &mut Cx, expanded: bool, content_height: f64) {
        if self.expanded == expanded {
            return;
        }

        self.expanded = expanded;
        self.target_height = if expanded { content_height } else { 0.0 };
        self.animation_start = Cx::time_now();
        self.animating = true;
        cx.new_next_frame();
    }

    fn update_animation(&mut self, cx: &mut Cx) {
        let elapsed = Cx::time_now() - self.animation_start;
        let t = (elapsed / EXPAND_DURATION).min(1.0);

        // Ease out cubic
        let eased = 1.0 - (1.0 - t).powi(3);

        let current_height = if self.expanded {
            eased * self.target_height
        } else {
            (1.0 - eased) * self.target_height
        };

        self.view.apply_over(cx, live! {
            height: (current_height)
        });

        if t >= 1.0 {
            self.animating = false;
            // Set final state
            if self.expanded {
                self.view.apply_over(cx, live! {
                    height: Fit
                });
            } else {
                self.view.apply_over(cx, live! {
                    height: 0
                });
            }
        } else {
            cx.new_next_frame();
        }

        self.view.redraw(cx);
    }
}

impl ExpandableSectionRef {
    pub fn set_expanded(&self, cx: &mut Cx, expanded: bool, content_height: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_expanded(cx, expanded, content_height);
        }
    }
}

// ============================================================================
// SIDEBAR MENU ACTIONS
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum SidebarMenuAction {
    ItemClicked(LiveId),
    ToggleExpand(bool),
    None,
}

// ============================================================================
// WIDGET EXTENSION TRAITS
// ============================================================================

/// Extension trait for accessing sidebar menu widgets from View
pub trait SidebarMenuWidgetExt {
    fn sidebar_menu_item(&self, path: &[LiveId]) -> SidebarMenuItemRef;
    fn show_more_button(&self, path: &[LiveId]) -> ShowMoreButtonRef;
    fn expandable_section(&self, path: &[LiveId]) -> ExpandableSectionRef;
}

impl SidebarMenuWidgetExt for WidgetRef {
    fn sidebar_menu_item(&self, path: &[LiveId]) -> SidebarMenuItemRef {
        SidebarMenuItemRef(self.widget(path))
    }

    fn show_more_button(&self, path: &[LiveId]) -> ShowMoreButtonRef {
        ShowMoreButtonRef(self.widget(path))
    }

    fn expandable_section(&self, path: &[LiveId]) -> ExpandableSectionRef {
        ExpandableSectionRef(self.widget(path))
    }
}

impl SidebarMenuWidgetExt for View {
    fn sidebar_menu_item(&self, path: &[LiveId]) -> SidebarMenuItemRef {
        SidebarMenuItemRef(self.widget(path))
    }

    fn show_more_button(&self, path: &[LiveId]) -> ShowMoreButtonRef {
        ShowMoreButtonRef(self.widget(path))
    }

    fn expandable_section(&self, path: &[LiveId]) -> ExpandableSectionRef {
        ExpandableSectionRef(self.widget(path))
    }
}
