//! Shell footer widget

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::live_design::*;

    pub ShellFooter = {{ShellFooter}} {
        width: Fill
        height: Fill

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

        padding: 12
        flow: Right
        align: { y: 0.5 }
        spacing: 16

        status_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR> { font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    // Light: gray-600, Dark: slate-400
                    let light = vec4(0.294, 0.333, 0.388, 1.0);
                    let dark = vec4(0.580, 0.639, 0.722, 1.0);
                    return mix(light, dark, self.dark_mode);
                }
            }
            text: "Footer - Timeline / Status Bar"
        }

        <View> { width: Fill }

        hint_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR> { font_size: 10.0 }
                fn get_color(self) -> vec4 {
                    // Light: gray-400, Dark: slate-500
                    let light = vec4(0.612, 0.639, 0.686, 1.0);
                    let dark = vec4(0.392, 0.455, 0.545, 1.0);
                    return mix(light, dark, self.dark_mode);
                }
            }
            text: "Drag top edge to resize"
        }
    }
}

/// Shell footer widget
#[derive(Live, LiveHook, Widget)]
pub struct ShellFooter {
    #[deref]
    view: View,

    #[live]
    status: String,

    #[live]
    hint: String,
}

impl Widget for ShellFooter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.status.is_empty() {
            self.view.label(id!(status_label)).set_text(cx, &self.status);
        }
        if !self.hint.is_empty() {
            self.view.label(id!(hint_label)).set_text(cx, &self.hint);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShellFooterRef {
    pub fn set_status(&self, cx: &mut Cx, status: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.status = status.to_string();
            inner.view.label(id!(status_label)).set_text(cx, status);
        }
    }

    pub fn set_hint(&self, cx: &mut Cx, hint: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hint = hint.to_string();
            inner.view.label(id!(hint_label)).set_text(cx, hint);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(status_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(hint_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
        }
    }
}
