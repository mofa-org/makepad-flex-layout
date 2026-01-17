//! Shell sidebar widget

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::live_design::*;

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
            text: "Sidebar"
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

        content = <View> {
            width: Fill
            height: Fill
            padding: 12

            placeholder = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR> { font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        // Light: gray-400, Dark: slate-500
                        let light = vec4(0.612, 0.639, 0.686, 1.0);
                        let dark = vec4(0.392, 0.455, 0.545, 1.0);
                        return mix(light, dark, self.dark_mode);
                    }
                }
                text: "Sidebar content"
            }
        }
    }
}

/// Shell sidebar widget
#[derive(Live, LiveHook, Widget)]
pub struct ShellSidebar {
    #[deref]
    view: View,

    #[live]
    title: String,
}

impl Widget for ShellSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.title.is_empty() {
            self.view.label(id!(header.header_label)).set_text(cx, &self.title);
        }

        self.view.draw_walk(cx, scope, walk)
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
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.view(id!(header)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(header.header_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(content.placeholder)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
        }
    }
}
