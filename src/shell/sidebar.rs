//! Shell sidebar widget with app menu
//!
//! Provides a sidebar with:
//! - App selection menu items with hover effects
//! - Show More/Less expandable section
//! - Selection state tracking
//! - Dark mode support

use makepad_widgets::*;
use crate::shell::sidebar_menu::{SidebarMenuAction, SidebarMenuWidgetExt};

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

            // Primary apps (always visible)
            app_item_0 = <SidebarMenuItem> {
                item_id: app_0
                label = { text: "Dashboard" }
            }
            app_item_1 = <SidebarMenuItem> {
                item_id: app_1
                label = { text: "Editor" }
            }
            app_item_2 = <SidebarMenuItem> {
                item_id: app_2
                label = { text: "Terminal" }
            }
            app_item_3 = <SidebarMenuItem> {
                item_id: app_3
                label = { text: "Explorer" }
            }

            // Show More button
            show_more_btn = <ShowMoreButton> {}

            // Expandable section for additional apps
            expanded_section = <ExpandableSection> {
                content = {
                    app_item_4 = <SidebarMenuItem> {
                        item_id: app_4
                        label = { text: "Database" }
                    }
                    app_item_5 = <SidebarMenuItem> {
                        item_id: app_5
                        label = { text: "Network" }
                    }
                    app_item_6 = <SidebarMenuItem> {
                        item_id: app_6
                        label = { text: "Metrics" }
                    }
                    app_item_7 = <SidebarMenuItem> {
                        item_id: app_7
                        label = { text: "Logs" }
                    }
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

            settings_item = <SidebarMenuItem> {
                item_id: settings
                label = { text: "Settings" }
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
    expanded: bool,
}

impl Widget for ShellSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Capture actions from menu items
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle menu actions
        for action in actions.iter() {
            match action.as_widget_action().cast::<SidebarMenuAction>() {
                SidebarMenuAction::ItemClicked(id) => {
                    self.handle_item_click(cx, id, scope);
                }
                SidebarMenuAction::ToggleExpand(expanded) => {
                    self.handle_expand_toggle(cx, expanded);
                }
                SidebarMenuAction::None => {}
            }
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
    fn handle_item_click(&mut self, cx: &mut Cx, id: LiveId, scope: &mut Scope) {
        // Clear all selections first
        self.clear_all_selections(cx);

        // Determine selection based on clicked ID
        let selection = match id {
            id if id == live_id!(app_0) => Some(SidebarSelection::App(0)),
            id if id == live_id!(app_1) => Some(SidebarSelection::App(1)),
            id if id == live_id!(app_2) => Some(SidebarSelection::App(2)),
            id if id == live_id!(app_3) => Some(SidebarSelection::App(3)),
            id if id == live_id!(app_4) => Some(SidebarSelection::App(4)),
            id if id == live_id!(app_5) => Some(SidebarSelection::App(5)),
            id if id == live_id!(app_6) => Some(SidebarSelection::App(6)),
            id if id == live_id!(app_7) => Some(SidebarSelection::App(7)),
            id if id == live_id!(settings) => Some(SidebarSelection::Settings),
            _ => None,
        };

        if let Some(ref sel) = selection {
            // Apply selection styling
            self.apply_selection(cx, sel);
            self.selection = selection.clone();

            // Emit action to parent
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SidebarAction::SelectionChanged(selection),
            );
        }
    }

    fn handle_expand_toggle(&mut self, cx: &mut Cx, expanded: bool) {
        self.expanded = expanded;

        // Calculate content height (4 items * 36px height + spacing)
        let content_height = 4.0 * 36.0 + 4.0 * 4.0;

        self.view
            .expandable_section(id!(menu_section.expanded_section))
            .set_expanded(cx, expanded, content_height);
    }

    fn clear_all_selections(&mut self, cx: &mut Cx) {
        // Clear all app items
        self.view.sidebar_menu_item(id!(menu_section.app_item_0)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.app_item_1)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.app_item_2)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.app_item_3)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_4)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_5)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_6)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_7)).set_selected(cx, false);
        self.view.sidebar_menu_item(id!(bottom_section.settings_item)).set_selected(cx, false);
    }

    fn apply_selection(&mut self, cx: &mut Cx, selection: &SidebarSelection) {
        match selection {
            SidebarSelection::App(0) => {
                self.view.sidebar_menu_item(id!(menu_section.app_item_0)).set_selected(cx, true);
            }
            SidebarSelection::App(1) => {
                self.view.sidebar_menu_item(id!(menu_section.app_item_1)).set_selected(cx, true);
            }
            SidebarSelection::App(2) => {
                self.view.sidebar_menu_item(id!(menu_section.app_item_2)).set_selected(cx, true);
            }
            SidebarSelection::App(3) => {
                self.view.sidebar_menu_item(id!(menu_section.app_item_3)).set_selected(cx, true);
            }
            SidebarSelection::App(4) => {
                self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_4)).set_selected(cx, true);
            }
            SidebarSelection::App(5) => {
                self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_5)).set_selected(cx, true);
            }
            SidebarSelection::App(6) => {
                self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_6)).set_selected(cx, true);
            }
            SidebarSelection::App(7) => {
                self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_7)).set_selected(cx, true);
            }
            SidebarSelection::Settings => {
                self.view.sidebar_menu_item(id!(bottom_section.settings_item)).set_selected(cx, true);
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

        // Menu items
        self.view.sidebar_menu_item(id!(menu_section.app_item_0)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.app_item_1)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.app_item_2)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.app_item_3)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_4)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_5)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_6)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(menu_section.expanded_section.content.app_item_7)).apply_dark_mode(cx, dark_mode);
        self.view.sidebar_menu_item(id!(bottom_section.settings_item)).apply_dark_mode(cx, dark_mode);

        // Show more button
        self.view.show_more_button(id!(menu_section.show_more_btn)).apply_dark_mode(cx, dark_mode);
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

