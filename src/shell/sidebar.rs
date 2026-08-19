//! Shell sidebar widget with app menu
//!
//! Provides a sidebar with:
//! - App selection menu items with hover effects
//! - Show More/Less expandable section
//! - Selection state tracking
//! - Dark mode support

use makepad_widgets::*;
use crate::theme::get_global_dark_mode;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*
    use mod.widgets.shell.*

    // Sidebar header component
    mod.widgets.ShellSidebarHeader = View{
        width: Fill
        height: 40
        padding: Inset{left: 16.}
        align: Align{y: 0.5}

        show_bg: true
        draw_bg +: {
            dark_mode: instance(0.0)
            bg_light: uniform(mod.widgets.shell.BG_SIDEBAR_L)
            bg_dark: uniform(mod.widgets.shell.BG_SIDEBAR_D)
            pixel: fn() {
                return mix(self.bg_light, self.bg_dark, self.dark_mode)
            }
        }

        header_label := Label{
            draw_text +: {
                dark_mode: instance(0.0)
                text_style: mod.widgets.shell.FONT_SEMIBOLD{font_size: 12.0}
                get_color: fn() {
                    // Light: gray-700, Dark: slate-200
                    let light = vec4(0.247, 0.282, 0.333, 1.0)
                    let dark = vec4(0.886, 0.910, 0.941, 1.0)
                    return mix(light, dark, self.dark_mode)
                }
            }
            text: "Apps"
        }
    }

    // Separator line
    mod.widgets.MenuSeparator = View{
        width: Fill
        height: 1
        margin: Inset{top: 8. bottom: 8. left: 8. right: 8.}
        show_bg: true
        draw_bg +: {
            dark_mode: instance(0.0)
            bg_light: uniform(mod.widgets.shell.BORDER_L)
            bg_dark: uniform(mod.widgets.shell.BORDER_D)
            pixel: fn() {
                return mix(self.bg_light, self.bg_dark, self.dark_mode)
            }
        }
    }

    mod.widgets.ShellSidebarBase = #(ShellSidebar::register_widget(vm))
    mod.widgets.ShellSidebar = set_type_default() do mod.widgets.ShellSidebarBase{
        width: Fill
        height: Fill
        flow: Down
        cursor: MouseCursor.Default

        show_bg: true
        draw_bg +: {
            dark_mode: instance(0.0)
            pixel: fn() {
                // Light: slate-50, Dark: slate-900
                let light = vec4(0.973, 0.980, 0.988, 1.0)
                let dark = vec4(0.059, 0.090, 0.165, 1.0)
                return mix(light, dark, self.dark_mode)
            }
        }

        header := mod.widgets.ShellSidebarHeader{}

        // Main menu section
        menu_section := View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 8. right: 8. top: 4. bottom: 4.}
            spacing: 2

            // Primary apps with icons
            app_btn_0 := SidebarMenuButton{
                text: "Dashboard"
                draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_layout.svg")
            }
            app_btn_1 := SidebarMenuButton{
                text: "Editor"
                draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_file.svg")
            }
            app_btn_2 := SidebarMenuButton{
                text: "Terminal"
                draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_text.svg")
            }
            app_btn_3 := SidebarMenuButton{
                text: "Explorer"
                draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_folder.svg")
            }

            // Show More button (using Button for reliable click detection)
            show_more_btn := Button{
                width: Fill, height: Fit
                padding: Inset{top: 8. bottom: 8. left: 12. right: 12.}
                align: Align{x: 0.0 y: 0.5}
                draw_bg +: {
                    dark_mode: instance(0.0)
                    pixel: fn() {
                        let light = vec4(0.973, 0.980, 0.988, 1.0) // slate-50
                        let dark = vec4(0.059, 0.090, 0.165, 1.0)  // slate-900
                        return mix(light, dark, self.dark_mode)
                    }
                }
                draw_text +: {
                    dark_mode: instance(0.0)
                    text_style: mod.widgets.shell.FONT_REGULAR{font_size: 10.0}
                    get_color: fn() {
                        let light = vec4(0.392, 0.455, 0.545, 1.0) // slate-500
                        let dark = vec4(0.580, 0.639, 0.722, 1.0)  // slate-400
                        return mix(light, dark, self.dark_mode)
                    }
                }
                text: "Show More >"
            }

            // Collapsible section for additional apps
            more_apps_section := View{
                width: Fill, height: Fit
                flow: Down
                spacing: 2
                visible: false

                app_btn_4 := SidebarMenuButton{
                    text: "Database"
                    draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_widget.svg")
                }
                app_btn_5 := SidebarMenuButton{
                    text: "Network"
                    draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_vector.svg")
                }
                app_btn_6 := SidebarMenuButton{
                    text: "Metrics"
                    draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_draw.svg")
                }
                app_btn_7 := SidebarMenuButton{
                    text: "Logs"
                    draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_text.svg")
                }
            }
        }

        // Separator
        separator := mod.widgets.MenuSeparator{}

        // Bottom section (settings, etc.)
        bottom_section := View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 8. right: 8. bottom: 8.}

            settings_btn := SidebarMenuButton{
                text: "Settings"
                draw_icon.svg: crate_resource("makepad_widgets:resources/icons/icon_select.svg")
            }
        }

        // Spacer to push bottom section down
        View{
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
#[derive(Script, ScriptHook, Widget)]
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
        if self.view.button(cx, ids!(menu_section.show_more_btn)).clicked(&actions) {
            self.toggle_more_apps(cx);
        }

        // Handle app button clicks
        if self.view.button(cx, ids!(menu_section.app_btn_0)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(0));
        }
        if self.view.button(cx, ids!(menu_section.app_btn_1)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(1));
        }
        if self.view.button(cx, ids!(menu_section.app_btn_2)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(2));
        }
        if self.view.button(cx, ids!(menu_section.app_btn_3)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(3));
        }
        if self.view.button(cx, ids!(menu_section.more_apps_section.app_btn_4)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(4));
        }
        if self.view.button(cx, ids!(menu_section.more_apps_section.app_btn_5)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(5));
        }
        if self.view.button(cx, ids!(menu_section.more_apps_section.app_btn_6)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(6));
        }
        if self.view.button(cx, ids!(menu_section.more_apps_section.app_btn_7)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::App(7));
        }
        if self.view.button(cx, ids!(bottom_section.settings_btn)).clicked(&actions) {
            self.handle_selection(cx, SidebarSelection::Settings);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Apply global theme on every draw
        let dm = get_global_dark_mode();
        self.apply_dark_mode_internal(cx, dm);

        if !self.title.is_empty() {
            self.view.label(cx, ids!(header.header_label)).set_text(cx, &self.title);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShellSidebar {
    fn toggle_more_apps(&mut self, cx: &mut Cx) {
        self.more_apps_visible = !self.more_apps_visible;

        // Toggle visibility
        self.view.view(cx, ids!(menu_section.more_apps_section))
            .set_visible(cx, self.more_apps_visible);

        // Update button text
        if self.more_apps_visible {
            self.view.button(cx, ids!(menu_section.show_more_btn))
                .set_text(cx, "Show Less ^");
        } else {
            self.view.button(cx, ids!(menu_section.show_more_btn))
                .set_text(cx, "Show More >");
        }

        self.view.redraw(cx);
    }

    fn handle_selection(&mut self, cx: &mut Cx, selection: SidebarSelection) {
        // Clear all selections first
        self.clear_all_selections(cx);

        // Apply selection
        self.apply_selection(cx, &selection);
        self.selection = Some(selection.clone());

        // Emit action to parent
        cx.widget_action(
            self.widget_uid(),
            SidebarAction::SelectionChanged(Some(selection)),
        );

        self.view.redraw(cx);
    }

    fn clear_all_selections(&mut self, cx: &mut Cx) {
        // Clear all app buttons
        let button_ids: [&[LiveId]; 9] = [
            ids!(menu_section.app_btn_0),
            ids!(menu_section.app_btn_1),
            ids!(menu_section.app_btn_2),
            ids!(menu_section.app_btn_3),
            ids!(menu_section.more_apps_section.app_btn_4),
            ids!(menu_section.more_apps_section.app_btn_5),
            ids!(menu_section.more_apps_section.app_btn_6),
            ids!(menu_section.more_apps_section.app_btn_7),
            ids!(bottom_section.settings_btn),
        ];
        for btn_id in button_ids {
            let mut btn = self.view.button(cx, btn_id);
            script_apply_eval!(cx, btn, { draw_bg +: { selected: 0.0 } });
        }
    }

    fn apply_selection(&mut self, cx: &mut Cx, selection: &SidebarSelection) {
        let btn_id: &[LiveId] = match selection {
            SidebarSelection::App(0) => ids!(menu_section.app_btn_0),
            SidebarSelection::App(1) => ids!(menu_section.app_btn_1),
            SidebarSelection::App(2) => ids!(menu_section.app_btn_2),
            SidebarSelection::App(3) => ids!(menu_section.app_btn_3),
            SidebarSelection::App(4) => ids!(menu_section.more_apps_section.app_btn_4),
            SidebarSelection::App(5) => ids!(menu_section.more_apps_section.app_btn_5),
            SidebarSelection::App(6) => ids!(menu_section.more_apps_section.app_btn_6),
            SidebarSelection::App(7) => ids!(menu_section.more_apps_section.app_btn_7),
            SidebarSelection::Settings => ids!(bottom_section.settings_btn),
            _ => return,
        };
        let mut btn = self.view.button(cx, btn_id);
        script_apply_eval!(cx, btn, { draw_bg +: { selected: 1.0 } });
    }

    pub fn apply_dark_mode_internal(&mut self, cx: &mut Cx, dark_mode: f64) {
        // Background
        script_apply_eval!(cx, self.view, {
            draw_bg +: { dark_mode: #(dark_mode) }
        });

        // Header
        let mut header = self.view.view(cx, ids!(header));
        script_apply_eval!(cx, header, {
            draw_bg +: { dark_mode: #(dark_mode) }
        });
        let mut header_label = self.view.label(cx, ids!(header.header_label));
        script_apply_eval!(cx, header_label, {
            draw_text +: { dark_mode: #(dark_mode) }
        });

        // Separator
        let mut separator = self.view.view(cx, ids!(separator));
        script_apply_eval!(cx, separator, {
            draw_bg +: { dark_mode: #(dark_mode) }
        });

        // Menu buttons in menu_section
        let menu_button_ids: [&[LiveId]; 4] = [
            ids!(menu_section.app_btn_0), ids!(menu_section.app_btn_1),
            ids!(menu_section.app_btn_2), ids!(menu_section.app_btn_3),
        ];
        for btn_id in menu_button_ids {
            let mut btn = self.view.button(cx, btn_id);
            script_apply_eval!(cx, btn, {
                draw_bg +: { dark_mode: #(dark_mode) }
                draw_text +: { dark_mode: #(dark_mode) }
                draw_icon +: { dark_mode: #(dark_mode) }
            });
        }

        // Show More button
        let mut show_more_btn = self.view.button(cx, ids!(menu_section.show_more_btn));
        script_apply_eval!(cx, show_more_btn, {
            draw_bg +: { dark_mode: #(dark_mode) }
            draw_text +: { dark_mode: #(dark_mode) }
        });

        // More apps section buttons
        let more_button_ids: [&[LiveId]; 4] = [
            ids!(menu_section.more_apps_section.app_btn_4),
            ids!(menu_section.more_apps_section.app_btn_5),
            ids!(menu_section.more_apps_section.app_btn_6),
            ids!(menu_section.more_apps_section.app_btn_7),
        ];
        for btn_id in more_button_ids {
            let mut btn = self.view.button(cx, btn_id);
            script_apply_eval!(cx, btn, {
                draw_bg +: { dark_mode: #(dark_mode) }
                draw_text +: { dark_mode: #(dark_mode) }
                draw_icon +: { dark_mode: #(dark_mode) }
            });
        }

        // Settings button
        let mut settings_btn = self.view.button(cx, ids!(bottom_section.settings_btn));
        script_apply_eval!(cx, settings_btn, {
            draw_bg +: { dark_mode: #(dark_mode) }
            draw_text +: { dark_mode: #(dark_mode) }
            draw_icon +: { dark_mode: #(dark_mode) }
        });
    }
}

impl ShellSidebarRef {
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.title = title.to_string();
            inner.view.label(cx, ids!(header.header_label)).set_text(cx, title);
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

#[derive(Clone, Debug, Default)]
pub enum SidebarAction {
    SelectionChanged(Option<SidebarSelection>),
    #[default]
    None,
}

// ShellSidebarWidgetExt trait is auto-generated by #[derive(Widget)]
