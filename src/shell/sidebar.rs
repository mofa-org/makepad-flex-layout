//! Shell sidebar widget with app menu
//!
//! Provides a sidebar with:
//! - App selection menu items with hover effects
//! - Show More/Less expandable section
//! - Selection state tracking
//! - Dark mode support

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::live_design::*;
    use crate::shell::sidebar_menu::*;

    // Sidebar header component
    pub ShellSidebarHeader = <View> {
        width: Fill
        height: 40
        padding: { left: 16 }
        align: { y: 0.5 }

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Light: slate-100, Dark: slate-800
                let light = vec4(0.945, 0.961, 0.976, 1.0);
                let dark = vec4(0.122, 0.161, 0.231, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        header_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD> { font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    // Light: gray-700, Dark: slate-200
                    let light = vec4(0.247, 0.282, 0.333, 1.0);
                    let dark = vec4(0.886, 0.910, 0.941, 1.0);
                    return mix(light, dark, self.dark_mode);
                }
            }
            text: "Apps"
        }
    }

    // Separator line
    pub MenuSeparator = <View> {
        width: Fill
        height: 1
        margin: { top: 8, bottom: 8, left: 8, right: 8 }
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let light = vec4(0.886, 0.910, 0.941, 1.0);  // slate-200
                let dark = vec4(0.192, 0.231, 0.302, 1.0);   // slate-700
                return mix(light, dark, self.dark_mode);
            }
        }
    }

    pub ShellSidebar = {{ShellSidebar}} {
        width: Fill
        height: Fill
        flow: Down
        cursor: Default

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Light: slate-50, Dark: slate-900
                let light = vec4(0.973, 0.980, 0.988, 1.0);
                let dark = vec4(0.059, 0.090, 0.165, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        header = <ShellSidebarHeader> {}

        // Main menu section
        menu_section = <View> {
            width: Fill
            height: Fit
            flow: Down
            padding: { left: 8, right: 8, top: 4, bottom: 4 }
            spacing: 2

            // Primary apps with icons
            app_btn_0 = <SidebarMenuButton> {
                text: "Dashboard"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_layout.svg") }
            }
            app_btn_1 = <SidebarMenuButton> {
                text: "Editor"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_file.svg") }
            }
            app_btn_2 = <SidebarMenuButton> {
                text: "Terminal"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_text.svg") }
            }
            app_btn_3 = <SidebarMenuButton> {
                text: "Explorer"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_folder.svg") }
            }

            // Show More button (using Button for reliable click detection)
            show_more_btn = <Button> {
                width: Fill, height: Fit
                padding: {top: 8, bottom: 8, left: 12, right: 12}
                align: {x: 0.0, y: 0.5}
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return vec4(0.973, 0.980, 0.988, 1.0); // slate-50
                    }
                }
                draw_text: {
                    text_style: <FONT_REGULAR> { font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return vec4(0.392, 0.455, 0.545, 1.0); // slate-500
                    }
                }
                text: "Show More >"
            }

            // Collapsible section for additional apps
            more_apps_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 2
                visible: false

                app_btn_4 = <SidebarMenuButton> {
                    text: "Database"
                    draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_widget.svg") }
                }
                app_btn_5 = <SidebarMenuButton> {
                    text: "Network"
                    draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_vector.svg") }
                }
                app_btn_6 = <SidebarMenuButton> {
                    text: "Metrics"
                    draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_draw.svg") }
                }
                app_btn_7 = <SidebarMenuButton> {
                    text: "Logs"
                    draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_text.svg") }
                }
            }
        }

        // Separator
        separator = <MenuSeparator> {}

        // Bottom section (settings, etc.)
        bottom_section = <View> {
            width: Fill
            height: Fit
            flow: Down
            padding: { left: 8, right: 8, bottom: 8 }

            settings_btn = <SidebarMenuButton> {
                text: "Settings"
                draw_icon: { svg_file: dep("crate://makepad-widgets/resources/icons/icon_select.svg") }
            }
        }

        // Spacer to push bottom section down
        <View> {
            width: Fill
            height: Fill
        }
    }
}

/// Selection state for the sidebar
#[derive(Clone, Debug, PartialEq)]
pub enum SidebarSelection {
    App(usize),
    Settings,
}

/// Shell sidebar widget with app menu
#[derive(Live, LiveHook, Widget)]
pub struct ShellSidebar {
    #[deref]
    view: View,

    #[live]
    title: String,

    #[rust]
    selection: Option<SidebarSelection>,

    #[rust]
    more_apps_visible: bool,
}

impl Widget for ShellSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Capture actions from child widgets (buttons)
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle Show More/Less click
        if self.view.button(id!(menu_section.show_more_btn)).clicked(&actions) {
            self.toggle_more_apps(cx);
        }

        // Handle app button clicks
        if self.view.button(id!(menu_section.app_btn_0)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(0), scope);
        }
        if self.view.button(id!(menu_section.app_btn_1)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(1), scope);
        }
        if self.view.button(id!(menu_section.app_btn_2)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(2), scope);
        }
        if self.view.button(id!(menu_section.app_btn_3)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(3), scope);
        }
        if self.view.button(id!(menu_section.more_apps_section.app_btn_4)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(4), scope);
        }
        if self.view.button(id!(menu_section.more_apps_section.app_btn_5)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(5), scope);
        }
        if self.view.button(id!(menu_section.more_apps_section.app_btn_6)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(6), scope);
        }
        if self.view.button(id!(menu_section.more_apps_section.app_btn_7)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(7), scope);
        }
        if self.view.button(id!(bottom_section.settings_btn)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::Settings, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.title.is_empty() {
            self.view.label(id!(header.header_label)).set_text(cx, &self.title);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShellSidebar {
    fn toggle_more_apps(&mut self, cx: &mut Cx) {
        self.more_apps_visible = !self.more_apps_visible;

        // Toggle visibility
        self.view.view(id!(menu_section.more_apps_section))
            .set_visible(cx, self.more_apps_visible);

        // Update button text
        if self.more_apps_visible {
            self.view.button(id!(menu_section.show_more_btn))
                .set_text(cx, "Show Less ^");
        } else {
            self.view.button(id!(menu_section.show_more_btn))
                .set_text(cx, "Show More >");
        }

        self.view.redraw(cx);
    }

    fn handle_selection(&mut self, cx: &mut Cx, selection: SidebarSelection, scope: &mut Scope) {
        // Clear all selections first
        self.clear_all_selections(cx);

        // Apply selection
        self.apply_selection(cx, &selection);
        self.selection = Some(selection.clone());

        // Emit action to parent
        cx.widget_action(
            self.widget_uid(),
            &scope.path,
            SidebarAction::SelectionChanged(Some(selection)),
        );

        self.view.redraw(cx);
    }

    fn clear_all_selections(&mut self, cx: &mut Cx) {
        // Clear all app buttons
        self.view.button(id!(menu_section.app_btn_0)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.app_btn_1)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.app_btn_2)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.app_btn_3)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.more_apps_section.app_btn_4)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.more_apps_section.app_btn_5)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.more_apps_section.app_btn_6)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(menu_section.more_apps_section.app_btn_7)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
        self.view.button(id!(bottom_section.settings_btn)).apply_over(cx, live!{ draw_bg: { selected: 0.0 } });
    }

    fn apply_selection(&mut self, cx: &mut Cx, selection: &SidebarSelection) {
        match selection {
            SidebarSelection::App(0) => {
                self.view.button(id!(menu_section.app_btn_0)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(1) => {
                self.view.button(id!(menu_section.app_btn_1)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(2) => {
                self.view.button(id!(menu_section.app_btn_2)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(3) => {
                self.view.button(id!(menu_section.app_btn_3)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(4) => {
                self.view.button(id!(menu_section.more_apps_section.app_btn_4)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(5) => {
                self.view.button(id!(menu_section.more_apps_section.app_btn_5)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(6) => {
                self.view.button(id!(menu_section.more_apps_section.app_btn_6)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::App(7) => {
                self.view.button(id!(menu_section.more_apps_section.app_btn_7)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::Settings => {
                self.view.button(id!(bottom_section.settings_btn)).apply_over(cx, live!{ draw_bg: { selected: 1.0 } });
            }
            _ => {}
        }
    }

    pub fn apply_dark_mode_internal(&mut self, cx: &mut Cx, dark_mode: f64) {
        // Background
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Header
        self.view.view(id!(header)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.label(id!(header.header_label)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Separator
        self.view.view(id!(separator)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
    }
}

impl ShellSidebarRef {
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.title = title.to_string();
            inner.view.label(id!(header.header_label)).set_text(cx, title);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.apply_dark_mode_internal(cx, dark_mode);
        }
    }

    pub fn get_selection(&self) -> Option<SidebarSelection> {
        self.borrow().and_then(|inner| inner.selection.clone())
    }

    pub fn set_selection(&self, cx: &mut Cx, selection: Option<SidebarSelection>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_all_selections(cx);
            if let Some(ref sel) = selection {
                inner.apply_selection(cx, sel);
            }
            inner.selection = selection;
        }
    }
}

// ============================================================================
// SIDEBAR ACTION
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum SidebarAction {
    SelectionChanged(Option<SidebarSelection>),
    None,
}

// ShellSidebarWidgetExt trait is auto-generated by #[derive(Widget)]
