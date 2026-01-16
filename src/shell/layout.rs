//! Shell layout widget - main container for the app shell

use makepad_widgets::*;
use crate::theme::{ShellTheme, THEME_TRANSITION_DURATION};
use crate::shell::config::ShellConfig;
use crate::shell::header::ShellHeaderAction;
use crate::shell::sidebar::ShellSidebarWidgetExt;
use crate::shell::footer::ShellFooterWidgetExt;
use crate::grid::panel_grid::PanelGridWidgetExt;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Import shell components - must use crate path for cross-module visibility
    use crate::shell::header::ShellHeader;
    use crate::shell::footer::ShellFooter;
    use crate::shell::sidebar::ShellSidebar;
    use crate::grid::panel_grid::PanelGrid;

    pub ShellLayout = {{ShellLayout}} {
        width: Fill
        height: Fill
        flow: Down

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

        // Fixed header
        header = <ShellHeader> {}

        // Main area using Dock with both horizontal and vertical splitters
        dock = <Dock> {
            width: Fill
            height: Fill

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

            footer_content = <ShellFooter> {}
        }
    }
}

/// Main shell layout widget
///
/// Provides the complete app shell with header, footer, sidebars, and panel grid.
/// Supports dark/light theme switching with smooth animations.
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
}

impl Widget for ShellLayout {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle header actions
        for action in actions.iter() {
            if let ShellHeaderAction::ToggleDarkMode = action.as_widget_action().cast() {
                self.toggle_dark_mode(cx);
            }
        }

        // Handle animation updates
        if self.dark_mode_animating {
            if let Event::NextFrame(_) = event {
                self.update_dark_mode_animation(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize on first draw
        if !self.initialized {
            self.initialized = true;
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

    /// Apply current theme to all widgets
    fn apply_theme(&mut self, cx: &mut Cx) {
        let dm = self.theme.dark_mode_anim;

        // Apply to shell background
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });

        // Apply to header
        self.view.view(id!(header)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });
        self.view.label(id!(header.title_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dm) }
        });
        self.view.label(id!(header.subtitle_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dm) }
        });
        self.view.button(id!(header.theme_toggle)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dm) }
        });

        // Access Dock content using shell_sidebar/shell_footer/panel_grid widget refs
        // These use recursive search to find widgets inside the Dock
        self.view.shell_sidebar(id!(left_sidebar_content)).apply_dark_mode(cx, dm);
        self.view.shell_sidebar(id!(right_sidebar_content)).apply_dark_mode(cx, dm);
        self.view.shell_footer(id!(footer_content)).apply_dark_mode(cx, dm);
        self.view.panel_grid(id!(center_content)).apply_dark_mode(cx, dm);

        // Note: Dock splitters use a neutral semi-transparent color
        // that works in both light and dark modes (can't dynamically theme them)
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
