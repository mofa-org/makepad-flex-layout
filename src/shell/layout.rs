//! Shell layout widget - main container for the app shell

use makepad_widgets::*;
use crate::theme::{ShellTheme, THEME_TRANSITION_DURATION, set_global_dark_mode};
use crate::shell::config::ShellConfig;
use crate::shell::header::{ShellHeaderAction, ShellHeaderWidgetExt};
use crate::shell::sidebar::ShellSidebarWidgetExt;
use crate::grid::panel_grid::PanelGridWidgetExt;
use crate::grid::footer_grid::FooterGridWidgetExt;
use crate::grid::{LayoutState, FooterLayoutState};
use crate::panel::PanelAction;
use crate::persistence::ShellPreferences;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*
    use mod.widgets.shell.*

    // Thin splitter template with hover highlight
    let ThinSplitter = Splitter{
        size: 1.0
        draw_bg +: {
            color: vec4(0.886, 0.910, 0.941, 1.0)     // slate-200 (light)
            color_hover: vec4(0.384, 0.514, 0.965, 1.0)  // blue-500 (highlight)
            color_drag: vec4(0.231, 0.400, 0.900, 1.0)   // blue-600

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                // Background: transparent normally, blue tint on hover
                let bg_normal = vec4(0.945, 0.961, 0.976, 0.0)  // transparent
                let bg_hover = vec4(0.859, 0.898, 0.996, 1.0)   // blue-100 highlight
                sdf.clear(mix(bg_normal, bg_hover, self.hover))

                // Line thickness increases on hover for better visibility
                let line_size = mix(self.bar_size, 3.0, self.hover)

                if self.is_vertical > 0.5 {
                    sdf.box(
                        self.splitter_pad,
                        self.rect_size.y * 0.5 - line_size * 0.5,
                        self.rect_size.x - 2.0 * self.splitter_pad,
                        line_size,
                        self.border_radius
                    )
                }
                else {
                    sdf.box(
                        self.rect_size.x * 0.5 - line_size * 0.5,
                        self.splitter_pad,
                        line_size,
                        self.rect_size.y - 2.0 * self.splitter_pad,
                        self.border_radius
                    )
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
                )
            }
        }
    }

    // Overlay menu button with purple accent
    // Using Button's built-in hover/pressed handling via shader variables
    mod.widgets.OverlayMenuButton = Button{
        width: Fill height: Fit
        padding: Inset{top: 12. bottom: 12. left: 16. right: 16.}
        margin: 0
        align: Align{x: 0.0 y: 0.5}
        icon_walk +: {width: 20 height: 20 margin: Inset{right: 12.}}

        animator +: {
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 0.0, pressed: 0.0}}
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 1.0, pressed: 0.0}}
                }
                down: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {draw_bg: {hover: 1.0, pressed: 1.0}}
                }
            }
        }

        draw_bg +: {
            pressed: instance(0.0)
            dark_mode: instance(0.0)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                // Light mode: white -> purple tint on hover -> darker on pressed
                // Dark mode: dark purple -> lighter purple on hover -> even lighter on pressed
                let light_normal = vec4(1.0, 1.0, 1.0, 1.0)
                let light_hover = vec4(0.933, 0.918, 0.980, 1.0)  // purple-100
                let light_pressed = vec4(0.882, 0.859, 0.957, 1.0)  // purple-200
                let dark_normal = vec4(0.090, 0.075, 0.145, 1.0)  // dark purple
                let dark_hover = vec4(0.150, 0.130, 0.210, 1.0)   // lighter dark purple
                let dark_pressed = vec4(0.180, 0.160, 0.250, 1.0) // even lighter

                let normal = mix(light_normal, dark_normal, self.dark_mode)
                let hover_color = mix(light_hover, dark_hover, self.dark_mode)
                let pressed_color = mix(light_pressed, dark_pressed, self.dark_mode)
                let color_h = mix(normal, hover_color, self.hover)
                let color_p = mix(color_h, pressed_color, self.pressed)

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0)
                sdf.fill(color_p)
                return sdf.result
            }
        }

        draw_text +: {
            dark_mode: instance(0.0)
            text_style: mod.widgets.shell.FONT_REGULAR{font_size: 12.0}
            get_color: fn() {
                // Purple-tinted text
                let light = vec4(0.345, 0.290, 0.502, 1.0)  // purple-700
                let dark = vec4(0.847, 0.824, 0.941, 1.0)   // purple-200
                return mix(light, dark, self.dark_mode)
            }
        }

        draw_icon +: {
            dark_mode: instance(0.0)
            get_color: fn() {
                // Purple icon color
                let light = vec4(0.545, 0.467, 0.757, 1.0)  // purple-500
                let dark = vec4(0.694, 0.631, 0.871, 1.0)   // purple-400
                return mix(light, dark, self.dark_mode)
            }
        }

        text: "Menu Item"
    }

    // Overlay sidebar content with purple theme
    mod.widgets.OverlaySidebarContent = View{
        width: Fill
        height: Fill
        flow: Down

        show_bg: true
        draw_bg +: {
            dark_mode: instance(0.0)
            pixel: fn() {
                // Light: white, Dark: very dark purple
                let light = vec4(1.0, 1.0, 1.0, 1.0)
                let dark = vec4(0.067, 0.055, 0.110, 1.0)
                return mix(light, dark, self.dark_mode)
            }
        }

        // Header
        overlay_header := View{
            width: Fill
            height: 48
            padding: Inset{left: 16.}
            align: Align{y: 0.5}

            show_bg: true
            draw_bg +: {
                dark_mode: instance(0.0)
                pixel: fn() {
                    // Purple header
                    let light = vec4(0.945, 0.929, 0.988, 1.0)  // purple-50
                    let dark = vec4(0.110, 0.090, 0.180, 1.0)   // dark purple
                    return mix(light, dark, self.dark_mode)
                }
            }

            overlay_title := Label{
                draw_text +: {
                    dark_mode: instance(0.0)
                    text_style: mod.widgets.shell.FONT_SEMIBOLD{font_size: 13.0}
                    get_color: fn() {
                        let light = vec4(0.345, 0.290, 0.502, 1.0)  // purple-700
                        let dark = vec4(0.847, 0.824, 0.941, 1.0)   // purple-200
                        return mix(light, dark, self.dark_mode)
                    }
                }
                text: "Quick Actions"
            }
        }

        // Menu items
        menu_items := View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{top: 8. bottom: 8.}

            btn_new_project := mod.widgets.OverlayMenuButton{
                text: "New Project"
                draw_icon +: { svg: crate_resource("makepad-widgets:resources/icons/icon_layout.svg") }
            }
            btn_open_recent := mod.widgets.OverlayMenuButton{
                text: "Open Recent"
                draw_icon +: { svg: crate_resource("makepad-widgets:resources/icons/icon_folder.svg") }
            }
            btn_import_file := mod.widgets.OverlayMenuButton{
                text: "Import File"
                draw_icon +: { svg: crate_resource("makepad-widgets:resources/icons/icon_file.svg") }
            }
            btn_export_data := mod.widgets.OverlayMenuButton{
                text: "Export Data"
                draw_icon +: { svg: crate_resource("makepad-widgets:resources/icons/icon_vector.svg") }
            }

            // Separator
            overlay_separator := View{
                width: Fill
                height: 1
                margin: Inset{top: 8. bottom: 8. left: 16. right: 16.}
                show_bg: true
                draw_bg +: {
                    dark_mode: instance(0.0)
                    pixel: fn() {
                        let light = vec4(0.847, 0.824, 0.941, 1.0)  // purple-200
                        let dark = vec4(0.200, 0.180, 0.280, 1.0)   // dark purple border
                        return mix(light, dark, self.dark_mode)
                    }
                }
            }

            btn_preferences := mod.widgets.OverlayMenuButton{
                text: "Preferences"
                draw_icon +: { svg: crate_resource("makepad-widgets:resources/icons/icon_select.svg") }
            }
            btn_help := mod.widgets.OverlayMenuButton{
                text: "Help & Support"
                draw_icon +: { svg: crate_resource("makepad-widgets:resources/icons/icon_text.svg") }
            }
        }
    }

    mod.widgets.ShellLayoutBase = #(ShellLayout::register_widget(vm))
    mod.widgets.ShellLayout = set_type_default() do mod.widgets.ShellLayoutBase{
        width: Fill
        height: Fill
        flow: Overlay  // Changed to Overlay to support overlay sidebar

        show_bg: true
        draw_bg +: {
            dark_mode: instance(0.0)
            bg_light: uniform(mod.widgets.shell.BG_CONTENT_L)
            bg_dark: uniform(mod.widgets.shell.BG_CONTENT_D)
            pixel: fn() {
                return mix(self.bg_light, self.bg_dark, self.dark_mode)
            }
        }

        // Main content container (header + dock)
        main_container := View{
            width: Fill
            height: Fill
            flow: Down

            // Fixed header (stays in place)
            header := ShellHeader{}

            // Dock wrapper - this gets pushed when sidebar expands
            dock_wrapper := View{
                width: Fill
                height: Fill
                margin: Inset{left: 0.}  // Explicit initial margin (collapsed state)

                // Animator for push effect
                animator: Animator{
                    sidebar: {
                        default: @collapsed
                        collapsed: AnimatorState{
                            from: {all: Forward {duration: 0.2}}
                            ease: OutCubic
                            apply: {margin: {left: 0.0}}
                        }
                        expanded: AnimatorState{
                            from: {all: Forward {duration: 0.2}}
                            ease: OutCubic
                            apply: {margin: {left: 270.0}}
                        }
                    }
                }

                // Main area using Dock with both horizontal and vertical splitters
                dock := Dock{
                    width: Fill
                    height: Fill
                    padding: 0

                    // Use thin splitter for this dock
                    splitter: ThinSplitter{}

                    // Reduce corner radius (default is 20)
                    round_corner +: {
                        border_radius: 0.0
                    }

                    // Root is vertical splitter for footer
                    root := DockSplitter{
                        axis: SplitterAxis.Vertical
                        align: SplitterAlign.FromB(100.0)
                        a: @main_area
                        b: @footer_panel
                    }

                    // Main area has horizontal splitters
                    main_area := DockSplitter{
                        axis: SplitterAxis.Horizontal
                        align: SplitterAlign.FromA(280.0)
                        a: @left_panel
                        b: @right_area
                    }

                    right_area := DockSplitter{
                        axis: SplitterAxis.Horizontal
                        align: SplitterAlign.FromB(300.0)
                        a: @center_panel
                        b: @right_panel
                    }

                    left_panel := DockTab{
                        name: ""
                        kind: @left_sidebar_content
                    }

                    center_panel := DockTab{
                        name: ""
                        kind: @center_content
                    }

                    right_panel := DockTab{
                        name: ""
                        kind: @right_sidebar_content
                    }

                    footer_panel := DockTab{
                        name: ""
                        kind: @footer_content
                    }

                    left_sidebar_content := ShellSidebar{
                        title: "Blueprint"
                    }

                    center_content := PanelGrid{}

                    right_sidebar_content := ShellSidebar{
                        title: "Properties"
                    }

                    footer_content := FooterGrid{
                        initial_panels: 7
                    }
                }
            } // End dock_wrapper
        } // End main_container

        // Pinned sidebar - animates width and pushes content (click behavior)
        pinned_sidebar := View{
            width: 0  // Starts collapsed, animates to 270
            height: Fill
            margin: Inset{top: 48. left: 0.}  // Below header, at left edge
            visible: false
            clip_x: true  // Clip content during width animation
            clip_y: true

            show_bg: true
            draw_bg +: {
                dark_mode: instance(0.0)

                pixel: fn() {
                    // Main background - purple tinted (same as overlay)
                    let light_bg = vec4(0.992, 0.988, 1.0, 1.0)  // very light purple
                    let dark_bg = vec4(0.067, 0.055, 0.110, 1.0) // very dark purple
                    return mix(light_bg, dark_bg, self.dark_mode)
                }
            }

            pinned_sidebar_content := mod.widgets.OverlaySidebarContent{}
        }

        // Overlay sidebar - appears/disappears instantly on hover (not used currently)
        overlay_sidebar := View{
            width: 270
            height: Fill
            margin: Inset{top: 48. left: 0.}  // Below header, at left edge
            visible: false

            show_bg: true
            draw_bg +: {
                dark_mode: instance(0.0)

                pixel: fn() {
                    // Main background - purple tinted
                    let light_bg = vec4(0.992, 0.988, 1.0, 1.0)  // very light purple
                    let dark_bg = vec4(0.067, 0.055, 0.110, 1.0) // very dark purple
                    return mix(light_bg, dark_bg, self.dark_mode)
                }
            }

            overlay_sidebar_content := mod.widgets.OverlaySidebarContent{}
        }
    }
}

/// Main shell layout widget
///
/// Provides the complete app shell with header, footer, sidebars, and panel grid.
/// Supports dark/light theme switching with smooth animations.
/// Also includes overlay sidebar (hover) and pinned sidebar (click) features.

// Layout constants
const APP_ID: &str = "makepad-flex-layout";
const SIDEBAR_WIDTH: f64 = 270.0;
const HEADER_HEIGHT: f64 = 48.0;
const SIDEBAR_ANIM_DURATION: f64 = 0.25;  // 250ms
const CLICK_DEBOUNCE_TIME: f64 = 0.3;     // 300ms

#[derive(Script, ScriptHook, Widget)]
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

    /// Debounce for click events (prevent double-toggle)
    #[rust]
    last_click_time: f64,

    /// Animation state for pinned sidebar (frame-by-frame animation like mofa-studio)
    #[rust]
    sidebar_pin_animating: bool,

    #[rust]
    sidebar_pin_anim_start: f64,

    #[rust]
    sidebar_pin_expanding: bool,

    /// Whether overlay sidebar is showing (hover state - doesn't push content)
    #[rust]
    overlay_showing: bool,
}

impl Widget for ShellLayout {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Let events flow to children
        self.view.handle_event(cx, event, scope);

        // Hover logic: show overlay sidebar when hovering hamburger or overlay itself
        // Only show overlay if sidebar is not pinned (pinned takes precedence)
        if let Event::MouseMove(e) = event {
            if !self.sidebar_pinned {
                // Get hamburger button area
                let hamburger = self.view.button(cx, ids!(main_container.header.hamburger_btn));
                let hamburger_rect = hamburger.area().rect(cx);

                // Get overlay sidebar area (fixed size even when not visible)
                let overlay = self.view.view(cx, ids!(overlay_sidebar));

                // Create a combined hover zone:
                // - The hamburger button
                // - A bridge zone from hamburger down to overlay
                // - The overlay sidebar area (270x full height, starting at y=48)
                let over_hamburger = hamburger_rect.contains(e.abs);

                // Bridge zone: area below hamburger connecting to overlay
                // Extends from hamburger's left edge to overlay width, from hamburger bottom to overlay top + some buffer
                let bridge_zone = Rect {
                    pos: dvec2(0.0, hamburger_rect.pos.y),
                    size: dvec2(SIDEBAR_WIDTH, HEADER_HEIGHT + 12.0),  // Cover header area + a bit below
                };
                let over_bridge = self.overlay_showing && bridge_zone.contains(e.abs);

                // Overlay zone: the actual sidebar area (y starts at header height, extends down)
                let overlay_zone = Rect {
                    pos: dvec2(0.0, HEADER_HEIGHT),
                    size: dvec2(SIDEBAR_WIDTH, 600.0),  // Fixed height for hover detection
                };
                let over_overlay = self.overlay_showing && overlay_zone.contains(e.abs);

                if over_hamburger || over_bridge || over_overlay {
                    if !self.overlay_showing {
                        self.overlay_showing = true;
                        overlay.set_visible(cx, true);
                        self.apply_overlay_theme(cx);
                        self.view.redraw(cx);
                    }
                } else if self.overlay_showing {
                    self.overlay_showing = false;
                    overlay.set_visible(cx, false);
                    self.view.redraw(cx);
                }
            }
        }

        // Handle animation updates
        if let Event::NextFrame(_) = event {
            if self.dark_mode_animating {
                self.update_dark_mode_animation(cx);
            }
            if self.sidebar_pin_animating {
                self.update_sidebar_animation(cx);
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

    /// Toggle sidebar expanded state - expands sidebar and pushes dock content
    /// Uses frame-by-frame animation (like MoFA Studio) for synced sidebar + content push
    fn toggle_sidebar_expanded(&mut self, cx: &mut Cx) {
        self.sidebar_pinned = !self.sidebar_pinned;
        self.sidebar_pin_expanding = self.sidebar_pinned;
        log!("layout.rs - toggle_sidebar_expanded: sidebar_pinned={}, expanding={}", self.sidebar_pinned, self.sidebar_pin_expanding);

        // Hide overlay sidebar when pinning (pinned takes over)
        if self.overlay_showing {
            self.overlay_showing = false;
            self.view.view(cx, ids!(overlay_sidebar)).set_visible(cx, false);
        }

        // Start frame-by-frame animation
        self.sidebar_pin_animating = true;
        self.sidebar_pin_anim_start = Cx::time_now();

        // Make pinned sidebar visible when expanding
        let pinned = self.view.view(cx, ids!(pinned_sidebar));
        if self.sidebar_pin_expanding {
            pinned.set_visible(cx, true);
        }

        // Request animation frames
        cx.new_next_frame();
        self.view.redraw(cx);
    }

    /// Update sidebar pin animation (frame-by-frame like MoFA Studio)
    fn update_sidebar_animation(&mut self, cx: &mut Cx) {
        let elapsed = Cx::time_now() - self.sidebar_pin_anim_start;
        let progress = (elapsed / SIDEBAR_ANIM_DURATION).min(1.0);

        // Ease out cubic for smooth deceleration
        let eased = 1.0 - (1.0 - progress).powi(3);

        // Calculate current width based on direction
        let current_width = if self.sidebar_pin_expanding {
            SIDEBAR_WIDTH * eased
        } else {
            SIDEBAR_WIDTH * (1.0 - eased)
        };

        // Apply width to pinned sidebar
        let mut pinned = self.view.view(cx, ids!(pinned_sidebar));
        script_apply_eval!(cx, pinned, { width: #(current_width) });

        // Apply matching margin to dock_wrapper (synced push effect)
        let mut dock_wrapper = self.view.view(cx, ids!(main_container.dock_wrapper));
        script_apply_eval!(cx, dock_wrapper, { margin +: { left: #(current_width) } });

        log!("layout.rs - sidebar animation: progress={:.2}, width={:.1}", progress, current_width);

        if progress >= 1.0 {
            // Animation complete
            self.sidebar_pin_animating = false;
            log!("layout.rs - sidebar animation complete, expanding={}", self.sidebar_pin_expanding);

            // Hide sidebar when fully collapsed
            if !self.sidebar_pin_expanding {
                pinned.set_visible(cx, false);
            }
        } else {
            // Continue animation
            cx.new_next_frame();
        }

        self.view.redraw(cx);
    }

    /// Apply current theme to all widgets
    fn apply_theme(&mut self, cx: &mut Cx) {
        let dm = self.theme.dark_mode_anim;

        // Update global theme state for widgets that read it during draw
        set_global_dark_mode(dm);

        // Apply to shell background
        script_apply_eval!(cx, self.view, {
            draw_bg +: { dark_mode: #(dm) }
        });

        // Apply to header (now inside main_container). Runtime applies must
        // target the header's inner View (composite widgets have no draw_bg
        // field of their own) — its Ref method does exactly that.
        self.view
            .shell_header(cx, ids!(main_container.header))
            .apply_dark_mode(cx, dm);

        // Access Dock content widgets using apply_over_and_redraw for deep propagation
        // The Dock creates content widgets dynamically with "kind" templates

        // Apply dark_mode to ALL nested widgets via recursive apply
        self.theme_bg(cx, ids!(main_container.dock_wrapper), dm);

        // Apply to left sidebar content
        self.theme_bg(cx, ids!(left_sidebar_content), dm);
        self.theme_bg(cx, ids!(left_sidebar_content.header), dm);
        self.theme_text(cx, ids!(left_sidebar_content.header.header_label), dm);

        // Apply to right sidebar content
        self.theme_bg(cx, ids!(right_sidebar_content), dm);
        self.theme_bg(cx, ids!(right_sidebar_content.header), dm);
        self.theme_text(cx, ids!(right_sidebar_content.header.header_label), dm);

        // Apply to center content (PanelGrid)
        self.theme_bg(cx, ids!(center_content), dm);

        // Apply to footer content
        self.theme_bg(cx, ids!(footer_content), dm);

        // Apply to overlay sidebar (purple themed)
        self.theme_bg(cx, ids!(overlay_sidebar), dm);
        self.apply_overlay_content_theme(cx, dm, ids!(overlay_sidebar.overlay_sidebar_content));

        // Apply to pinned sidebar (purple themed - used for click toggle)
        self.theme_bg(cx, ids!(pinned_sidebar), dm);
        self.apply_overlay_content_theme(cx, dm, ids!(pinned_sidebar.pinned_sidebar_content));

        // Note: Dock splitters use a neutral semi-transparent color
        // that works in both light and dark modes (can't dynamically theme them)
    }

    /// Apply a dark_mode value to a widget's draw_bg, skipping widgets that
    /// don't exist yet (dock tab content is created lazily).
    fn theme_bg(&mut self, cx: &mut Cx, path: &[LiveId], dm: f64) {
        let mut w = self.view.widget(cx, path);
        if !w.is_empty() {
            script_apply_eval!(cx, w, {
                draw_bg +: { dark_mode: #(dm) }
            });
        }
    }

    /// Same as `theme_bg` but for draw_text.
    fn theme_text(&mut self, cx: &mut Cx, path: &[LiveId], dm: f64) {
        let mut w = self.view.widget(cx, path);
        if !w.is_empty() {
            script_apply_eval!(cx, w, {
                draw_text +: { dark_mode: #(dm) }
            });
        }
    }

    /// Same as `theme_bg` but for themed buttons (bg + text + icon).
    fn theme_button(&mut self, cx: &mut Cx, path: &[LiveId], dm: f64) {
        let mut w = self.view.widget(cx, path);
        if !w.is_empty() {
            script_apply_eval!(cx, w, {
                draw_bg +: { dark_mode: #(dm) }
                draw_text +: { dark_mode: #(dm) }
                draw_icon +: { dark_mode: #(dm) }
            });
        }
    }

    /// Apply theme to overlay sidebar only (called when showing overlay)
    fn apply_overlay_theme(&mut self, cx: &mut Cx) {
        let dm = self.theme.dark_mode_anim;

        // Apply to overlay container
        self.theme_bg(cx, ids!(overlay_sidebar), dm);

        // Apply to overlay content
        self.apply_overlay_content_theme(cx, dm, ids!(overlay_sidebar.overlay_sidebar_content));

        // Apply to pinned sidebar too
        self.theme_bg(cx, ids!(pinned_sidebar), dm);
        self.apply_overlay_content_theme(cx, dm, ids!(pinned_sidebar.pinned_sidebar_content));
    }

    /// Apply theme to overlay content (overlay_sidebar)
    fn apply_overlay_content_theme(&mut self, cx: &mut Cx, dm: f64, _content_id: &[LiveId]) {
        // We apply to both overlay and pinned sidebar contents
        // The content_id parameter is no longer used - we explicitly access both

        // Apply to overlay_sidebar content
        self.theme_bg(cx, ids!(overlay_sidebar.overlay_sidebar_content), dm);
        self.theme_bg(cx, ids!(overlay_sidebar.overlay_sidebar_content.overlay_header), dm);
        self.theme_text(cx, ids!(overlay_sidebar.overlay_sidebar_content.overlay_header.overlay_title), dm);
        self.theme_bg(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.overlay_separator), dm);

        // Menu buttons in overlay_sidebar
        self.theme_button(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.btn_new_project), dm);
        self.theme_button(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.btn_open_recent), dm);
        self.theme_button(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.btn_import_file), dm);
        self.theme_button(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.btn_export_data), dm);
        self.theme_button(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.btn_preferences), dm);
        self.theme_button(cx, ids!(overlay_sidebar.overlay_sidebar_content.menu_items.btn_help), dm);

        // Apply to pinned_sidebar content (same structure)
        self.theme_bg(cx, ids!(pinned_sidebar.pinned_sidebar_content), dm);
        self.theme_bg(cx, ids!(pinned_sidebar.pinned_sidebar_content.overlay_header), dm);
        self.theme_text(cx, ids!(pinned_sidebar.pinned_sidebar_content.overlay_header.overlay_title), dm);
        self.theme_bg(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.overlay_separator), dm);

        // Menu buttons in pinned_sidebar
        self.theme_button(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.btn_new_project), dm);
        self.theme_button(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.btn_open_recent), dm);
        self.theme_button(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.btn_import_file), dm);
        self.theme_button(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.btn_export_data), dm);
        self.theme_button(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.btn_preferences), dm);
        self.theme_button(cx, ids!(pinned_sidebar.pinned_sidebar_content.menu_items.btn_help), dm);
    }

    /// Reset layout to default state
    pub fn reset_layout(&mut self, cx: &mut Cx) {
        // Reset our tracked layouts
        self.current_layout = Some(LayoutState::default());
        self.current_footer_layout = Some(FooterLayoutState::default());
        // Reset PanelGrid and FooterGrid (uses thread-local pending reset if borrow fails)
        self.view.panel_grid(cx, ids!(center_content)).reset_layout(cx);
        self.view.footer_grid(cx, ids!(footer_content)).reset_layout(cx);
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
            self.view.panel_grid(cx, ids!(center_content)).set_layout_state(cx, layout.clone());
        }

        // Apply saved footer layout and track it
        if let Some(ref footer_layout) = self.preferences.footer_layout {
            self.current_footer_layout = Some(footer_layout.clone());
            self.view.footer_grid(cx, ids!(footer_content)).set_layout_state(cx, footer_layout.clone());
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
