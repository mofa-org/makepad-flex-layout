//! Shell layout widget - main container for the app shell

use makepad_widgets::*;
use crate::theme::{ShellTheme, THEME_TRANSITION_DURATION};
use crate::shell::config::ShellConfig;
use crate::shell::header::ShellHeaderAction;
use crate::shell::sidebar::ShellSidebarWidgetExt;
use crate::grid::panel_grid::PanelGridWidgetExt;
use crate::grid::footer_grid::FooterGridWidgetExt;
use crate::grid::{LayoutState, FooterLayoutState};
use crate::panel::PanelAction;
use crate::persistence::ShellPreferences;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::live_design::*;

    // Import shell components - must use crate path for cross-module visibility
    use crate::shell::header::ShellHeader;
    use crate::shell::sidebar::ShellSidebar;
    use crate::grid::panel_grid::PanelGrid;
    use crate::grid::footer_grid::FooterGrid;

    // Thin splitter template with light colors
    ThinSplitter = <Splitter> {
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

    // Overlay menu button with purple accent
    // Using Button's built-in hover/pressed handling via shader variables
    pub OverlayMenuButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 12, bottom: 12, left: 16, right: 16}
        margin: 0
        align: {x: 0.0, y: 0.5}
        icon_walk: {width: 20, height: 20, margin: {right: 12}}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Light mode: white -> purple tint on hover -> darker on pressed
                // Dark mode: dark purple -> lighter purple on hover -> even lighter on pressed
                let light_normal = vec4(1.0, 1.0, 1.0, 1.0);
                let light_hover = vec4(0.933, 0.918, 0.980, 1.0);  // purple-100
                let light_pressed = vec4(0.882, 0.859, 0.957, 1.0);  // purple-200
                let dark_normal = vec4(0.090, 0.075, 0.145, 1.0);  // dark purple
                let dark_hover = vec4(0.150, 0.130, 0.210, 1.0);   // lighter dark purple
                let dark_pressed = vec4(0.180, 0.160, 0.250, 1.0); // even lighter

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let pressed_color = mix(light_pressed, dark_pressed, self.dark_mode);
                let color = mix(normal, hover_color, self.hover);
                let color = mix(color, pressed_color, self.pressed);

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                // Purple-tinted text
                let light = vec4(0.345, 0.290, 0.502, 1.0);  // purple-700
                let dark = vec4(0.847, 0.824, 0.941, 1.0);   // purple-200
                return mix(light, dark, self.dark_mode);
            }
        }

        draw_icon: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                // Purple icon color
                let light = vec4(0.545, 0.467, 0.757, 1.0);  // purple-500
                let dark = vec4(0.694, 0.631, 0.871, 1.0);   // purple-400
                return mix(light, dark, self.dark_mode);
            }
        }

        text: "Menu Item"
    }

    // Overlay sidebar content with purple theme
    pub OverlaySidebarContent = <View> {
        width: Fill
        height: Fill
        flow: Down

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Light: white, Dark: very dark purple
                let light = vec4(1.0, 1.0, 1.0, 1.0);
                let dark = vec4(0.067, 0.055, 0.110, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        // Header
        <View> {
            width: Fill
            height: 48
            padding: { left: 16 }
            align: { y: 0.5 }

            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    // Purple header
                    let light = vec4(0.945, 0.929, 0.988, 1.0);  // purple-50
                    let dark = vec4(0.110, 0.090, 0.180, 1.0);   // dark purple
                    return mix(light, dark, self.dark_mode);
                }
            }

            <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD> { font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        let light = vec4(0.345, 0.290, 0.502, 1.0);  // purple-700
                        let dark = vec4(0.847, 0.824, 0.941, 1.0);   // purple-200
                        return mix(light, dark, self.dark_mode);
                    }
                }
                text: "Quick Actions"
            }
        }

        // Menu items
        <View> {
            width: Fill
            height: Fit
            flow: Down
            padding: { top: 8, bottom: 8 }

            <OverlayMenuButton> {
                text: "New Project"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_layout.svg") }
            }
            <OverlayMenuButton> {
                text: "Open Recent"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_folder.svg") }
            }
            <OverlayMenuButton> {
                text: "Import File"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_file.svg") }
            }
            <OverlayMenuButton> {
                text: "Export Data"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_vector.svg") }
            }

            // Separator
            <View> {
                width: Fill
                height: 1
                margin: { top: 8, bottom: 8, left: 16, right: 16 }
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let light = vec4(0.847, 0.824, 0.941, 1.0);  // purple-200
                        let dark = vec4(0.200, 0.180, 0.280, 1.0);   // dark purple border
                        return mix(light, dark, self.dark_mode);
                    }
                }
            }

            <OverlayMenuButton> {
                text: "Preferences"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_select.svg") }
            }
            <OverlayMenuButton> {
                text: "Help & Support"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_text.svg") }
            }
        }
    }

    pub ShellLayout = {{ShellLayout}} {
        width: Fill
        height: Fill
        flow: Overlay  // Changed to Overlay to support overlay sidebar

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Light: white, Dark: slate-900
                let light = vec4(1.0, 1.0, 1.0, 1.0);
                let dark = vec4(0.059, 0.090, 0.165, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        // Main content container (header + dock)
        main_container = <View> {
            width: Fill
            height: Fill
            flow: Down

            // Fixed header (stays in place)
            header = <ShellHeader> {}

            // Dock wrapper - this gets pushed when sidebar expands
            dock_wrapper = <View> {
                width: Fill
                height: Fill

                // Animator for push effect
                animator: {
                    sidebar = {
                        default: collapsed
                        collapsed = {
                            from: {all: Forward {duration: 0.2}}
                            ease: OutCubic
                            apply: { margin: { left: 0.0 } }
                        }
                        expanded = {
                            from: {all: Forward {duration: 0.2}}
                            ease: OutCubic
                            apply: { margin: { left: 270.0 } }
                        }
                    }
                }

                // Main area using Dock with both horizontal and vertical splitters
                dock = <Dock> {
                    width: Fill
                    height: Fill
                    padding: 0

                    // Use thin splitter for this dock
                    splitter: <ThinSplitter> {}

                    // Reduce corner radius (default is 20)
                    round_corner: {
                        border_radius: 0.0
                    }

                    // Root is vertical splitter for footer
                    root = Splitter {
                        axis: Vertical
                        align: FromB(100.0)
                        a: main_area
                        b: footer_panel
                    }

                    // Main area has horizontal splitters
                    main_area = Splitter {
                        axis: Horizontal
                        align: FromA(280.0)
                        a: left_panel
                        b: right_area
                    }

                    right_area = Splitter {
                        axis: Horizontal
                        align: FromB(300.0)
                        a: center_panel
                        b: right_panel
                    }

                    left_panel = Tab {
                        name: ""
                        kind: left_sidebar_content
                    }

                    center_panel = Tab {
                        name: ""
                        kind: center_content
                    }

                    right_panel = Tab {
                        name: ""
                        kind: right_sidebar_content
                    }

                    footer_panel = Tab {
                        name: ""
                        kind: footer_content
                    }

                    left_sidebar_content = <ShellSidebar> {
                        title: "Blueprint"
                    }

                    center_content = <PanelGrid> {}

                    right_sidebar_content = <ShellSidebar> {
                        title: "Properties"
                    }

                    footer_content = <FooterGrid> {
                        initial_panels: 7
                    }
                }
            } // End dock_wrapper
        } // End main_container

        // Overlay sidebar - appears/disappears instantly on hover
        overlay_sidebar = <View> {
            width: 270
            height: Fill
            margin: { top: 48.0, left: 0.0 }  // Below header, at left edge
            visible: false

            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    // Main background - purple tinted
                    let light_bg = vec4(0.992, 0.988, 1.0, 1.0);  // very light purple
                    let dark_bg = vec4(0.067, 0.055, 0.110, 1.0); // very dark purple
                    return mix(light_bg, dark_bg, self.dark_mode);
                }
            }

            overlay_sidebar_content = <OverlaySidebarContent> {}
        }
    }
}

/// Main shell layout widget
///
/// Provides the complete app shell with header, footer, sidebars, and panel grid.
/// Supports dark/light theme switching with smooth animations.
/// App ID for persistence
const APP_ID: &str = "makepad-flex-layout";

#[derive(Live, LiveHook, Widget)]
pub struct ShellLayout {
    #[deref]
    view: View,

    #[rust]
    config: ShellConfig,

    #[rust]
    theme: ShellTheme,

    #[rust]
    dark_mode_animating: bool,

    #[rust]
    dark_mode_anim_start: f64,

    #[rust]
    initialized: bool,

    #[rust]
    preferences: ShellPreferences,

    /// Current layout state (updated via LayoutChanged actions from PanelGrid)
    #[rust]
    current_layout: Option<LayoutState>,

    /// Current footer layout state (updated via FooterLayoutChanged actions)
    #[rust]
    current_footer_layout: Option<FooterLayoutState>,

    /// Whether sidebar is expanded/pinned (click state - pushes content)
    #[rust]
    sidebar_pinned: bool,

    /// Whether overlay is visible (hover state - doesn't push)
    #[rust]
    overlay_visible: bool,

    /// Track if mouse is over overlay area
    #[rust]
    mouse_over_overlay: bool,
}

impl Widget for ShellLayout {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Track mouse position to detect hover over hamburger and overlay
        // This is more reliable than Hit events which can be affected by Button's internal handling
        if let Event::MouseMove(e) = event {
            // Get hamburger button area
            let hamburger = self.view.button(id!(main_container.header.hamburger_btn));
            let hamburger_rect = hamburger.area().rect(cx);

            // Check if mouse is over hamburger with padding
            let padding = 15.0;
            let in_hamburger = e.abs.x >= hamburger_rect.pos.x - padding
                && e.abs.x <= hamburger_rect.pos.x + hamburger_rect.size.x + padding
                && e.abs.y >= hamburger_rect.pos.y - padding
                && e.abs.y <= hamburger_rect.pos.y + hamburger_rect.size.y + padding;

            // Show overlay when hovering hamburger (if not already visible)
            if in_hamburger && !self.overlay_visible && !self.sidebar_pinned {
                log!("layout.rs - MouseMove detected over hamburger, showing overlay");
                self.show_overlay(cx);
            }

            // Track overlay area for closing
            if self.overlay_visible && !self.sidebar_pinned {
                // Overlay area: x from left edge to 285, y from 0 to bottom
                let overlay_width = 270.0;
                let overlay_height = 900.0;

                let in_overlay = e.abs.x <= overlay_width + padding
                    && e.abs.y >= 0.0
                    && e.abs.y <= overlay_height + padding;

                let was_over = self.mouse_over_overlay;
                self.mouse_over_overlay = in_hamburger || in_overlay;

                // Log only when state changes
                if was_over != self.mouse_over_overlay {
                    log!("layout.rs - MouseMove: mouse=({:.1}, {:.1}), in_hamburger={}, in_overlay={}", e.abs.x, e.abs.y, in_hamburger, in_overlay);
                }

                // If mouse left both areas, hide the overlay
                if !self.mouse_over_overlay {
                    log!("layout.rs - Hiding overlay: mouse left hover areas");
                    self.hide_overlay(cx);
                }
            }
        }

        // Handle header actions
        for action in actions.iter() {
            match action.as_widget_action().cast::<ShellHeaderAction>() {
                ShellHeaderAction::ToggleDarkMode => {
                    self.toggle_dark_mode(cx);
                }
                ShellHeaderAction::HamburgerHoverIn => {
                    // Hover is now handled via MouseMove tracking in layout
                    // This action is kept for compatibility but not used
                }
                ShellHeaderAction::HamburgerHoverOut => {
                    // Hover is now handled via MouseMove tracking in layout
                }
                ShellHeaderAction::HamburgerClicked => {
                    // Toggle sidebar expanded/pinned state (click to expand/collapse)
                    self.toggle_sidebar_expanded(cx);
                }
                ShellHeaderAction::ResetLayout => {
                    self.reset_layout(cx);
                }
                ShellHeaderAction::SaveLayout => {
                    self.save_layout(cx);
                }
                ShellHeaderAction::None => {}
            }

            // Capture layout changes from PanelGrid and FooterGrid
            match action.as_widget_action().cast::<PanelAction>() {
                PanelAction::LayoutChanged(state) => {
                    self.current_layout = Some(state);
                }
                PanelAction::FooterLayoutChanged(state) => {
                    self.current_footer_layout = Some(state);
                }
                _ => {}
            }
        }

        // Handle animation updates
        if let Event::NextFrame(_) = event {
            if self.dark_mode_animating {
                self.update_dark_mode_animation(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize on first draw
        if !self.initialized {
            self.initialized = true;
            self.load_preferences(cx);
            self.apply_theme(cx);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShellLayout {
    /// Toggle dark mode with animation
    pub fn toggle_dark_mode(&mut self, cx: &mut Cx) {
        self.theme.dark_mode = !self.theme.dark_mode;
        self.dark_mode_animating = true;
        self.dark_mode_anim_start = Cx::time_now();
        cx.new_next_frame();
        self.view.redraw(cx);
    }

    /// Set dark mode state (immediately, no animation)
    pub fn set_dark_mode(&mut self, cx: &mut Cx, dark: bool) {
        self.theme.set_dark_mode(dark);
        self.apply_theme(cx);
    }

    /// Check if dark mode is enabled
    pub fn is_dark_mode(&self) -> bool {
        self.theme.dark_mode
    }

    /// Update dark mode animation
    fn update_dark_mode_animation(&mut self, cx: &mut Cx) {
        let elapsed = Cx::time_now() - self.dark_mode_anim_start;

        if self.theme.update_animation(elapsed, THEME_TRANSITION_DURATION) {
            // Animation still in progress
            self.apply_theme(cx);
            cx.new_next_frame();
        } else {
            // Animation complete
            self.dark_mode_animating = false;
            self.apply_theme(cx);
        }

        self.view.redraw(cx);
    }

    /// Show overlay sidebar (hover - no push)
    fn show_overlay(&mut self, cx: &mut Cx) {
        log!("layout.rs - show_overlay called, overlay_visible={}", self.overlay_visible);
        if self.overlay_visible {
            return;
        }
        self.overlay_visible = true;

        let overlay = self.view.view(id!(overlay_sidebar));
        overlay.set_visible(cx, true);
        log!("layout.rs - show_overlay: set visible");

        self.view.redraw(cx);
    }

    /// Hide overlay sidebar (hover out - no push)
    fn hide_overlay(&mut self, cx: &mut Cx) {
        log!("layout.rs - hide_overlay called, overlay_visible={}, sidebar_pinned={}", self.overlay_visible, self.sidebar_pinned);
        if !self.overlay_visible || self.sidebar_pinned {
            return;
        }
        self.overlay_visible = false;

        let overlay = self.view.view(id!(overlay_sidebar));
        // Hide instantly - just set visibility
        overlay.set_visible(cx, false);
        log!("layout.rs - hide_overlay: set hidden");

        self.view.redraw(cx);
    }

    /// Toggle sidebar expanded state - expands sidebar and pushes dock content
    fn toggle_sidebar_expanded(&mut self, cx: &mut Cx) {
        self.sidebar_pinned = !self.sidebar_pinned;

        let overlay = self.view.view(id!(overlay_sidebar));
        let dock_wrapper = self.view.view(id!(main_container.dock_wrapper));

        if self.sidebar_pinned {
            // Expand: show overlay and push dock content
            self.overlay_visible = true;
            overlay.set_visible(cx, true);
            dock_wrapper.animator_play(cx, id!(sidebar.expanded));
        } else {
            // Collapse: hide overlay and restore dock content
            self.overlay_visible = false;
            overlay.set_visible(cx, false);
            dock_wrapper.animator_play(cx, id!(sidebar.collapsed));
        }

        self.view.redraw(cx);
    }

    /// Apply current theme to all widgets
    fn apply_theme(&mut self, cx: &mut Cx) {
        let dm = self.theme.dark_mode_anim;

        // Apply to shell background
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });

        // Apply to header (now inside main_container)
        self.view.view(id!(main_container.header)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });
        self.view.label(id!(main_container.header.title_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dm) }
        });
        self.view.button(id!(main_container.header.hamburger_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });
        self.view.button(id!(main_container.header.theme_toggle)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });
        self.view.button(id!(main_container.header.reset_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });
        self.view.button(id!(main_container.header.save_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });

        // Access Dock content using widget refs with recursive search
        self.view.shell_sidebar(id!(left_sidebar_content)).apply_dark_mode(cx, dm);
        self.view.shell_sidebar(id!(right_sidebar_content)).apply_dark_mode(cx, dm);
        self.view.panel_grid(id!(center_content)).apply_dark_mode(cx, dm);
        self.view.footer_grid(id!(footer_content)).apply_dark_mode(cx, dm);

        // Apply to overlay sidebar (purple themed)
        self.view.view(id!(overlay_sidebar)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });
        // Note: overlay_sidebar_content is a View, not ShellSidebar, so we don't call apply_dark_mode on it
        // The dark_mode instances in the overlay menu buttons handle theming automatically

        // Note: Dock splitters use a neutral semi-transparent color
        // that works in both light and dark modes (can't dynamically theme them)
    }

    /// Reset layout to default state
    pub fn reset_layout(&mut self, cx: &mut Cx) {
        // Reset our tracked layouts
        self.current_layout = Some(LayoutState::default());
        self.current_footer_layout = Some(FooterLayoutState::default());
        // Reset PanelGrid and FooterGrid (uses thread-local pending reset if borrow fails)
        self.view.panel_grid(id!(center_content)).reset_layout(cx);
        self.view.footer_grid(id!(footer_content)).reset_layout(cx);
        self.view.redraw(cx);
    }

    /// Load preferences from disk and apply
    fn load_preferences(&mut self, cx: &mut Cx) {
        self.preferences = ShellPreferences::load(APP_ID);

        // Apply dark mode preference
        if self.preferences.dark_mode {
            self.theme.set_dark_mode(true);
        }

        // Apply saved layout to PanelGrid and track it
        if let Some(ref layout) = self.preferences.layout {
            self.current_layout = Some(layout.clone());
            self.view.panel_grid(id!(center_content)).set_layout_state(cx, layout.clone());
        }

        // Apply saved footer layout and track it
        if let Some(ref footer_layout) = self.preferences.footer_layout {
            self.current_footer_layout = Some(footer_layout.clone());
            self.view.footer_grid(id!(footer_content)).set_layout_state(cx, footer_layout.clone());
        }
    }

    /// Save current layout to disk
    pub fn save_layout(&mut self, cx: &mut Cx) {
        // Use the layout state captured from LayoutChanged actions
        if let Some(layout) = &self.current_layout {
            self.preferences.layout = Some(layout.clone());
        } else if self.preferences.layout.is_none() {
            // No layout changes made yet, save default
            self.preferences.layout = Some(LayoutState::default());
        }

        // Save footer layout state
        if let Some(footer_layout) = &self.current_footer_layout {
            self.preferences.footer_layout = Some(footer_layout.clone());
        } else if self.preferences.footer_layout.is_none() {
            self.preferences.footer_layout = Some(FooterLayoutState::default());
        }

        // Save dark mode preference
        self.preferences.dark_mode = self.theme.dark_mode;

        // Persist to disk
        if let Err(e) = self.preferences.save(APP_ID) {
            log!("Failed to save layout: {}", e);
        }

        self.view.redraw(cx);
    }

    /// Get the shell configuration
    pub fn config(&self) -> &ShellConfig {
        &self.config
    }

    /// Get the current theme
    pub fn theme(&self) -> &ShellTheme {
        &self.theme
    }
}

impl ShellLayoutRef {
    /// Toggle dark mode with animation
    pub fn toggle_dark_mode(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle_dark_mode(cx);
        }
    }

    /// Set dark mode state
    pub fn set_dark_mode(&self, cx: &mut Cx, dark: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_dark_mode(cx, dark);
        }
    }

    /// Check if dark mode is enabled
    pub fn is_dark_mode(&self) -> bool {
        self.borrow().map(|inner| inner.is_dark_mode()).unwrap_or(false)
    }

    /// Apply dark mode value directly
    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.theme.dark_mode_anim = dark_mode;
            inner.apply_theme(cx);
        }
    }
}
