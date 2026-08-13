//! Shell footer widget

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*
    use mod.widgets.shell.*

    mod.widgets.ShellFooterBase = #(ShellFooter::register_widget(vm))
    mod.widgets.ShellFooter = set_type_default() do mod.widgets.ShellFooterBase{
        width: Fill
        height: Fill

        show_bg: true
        draw_bg +: {
            dark_mode: instance(0.0)
            bg_light: uniform(mod.widgets.shell.BG_FOOTER_L)
            bg_dark: uniform(mod.widgets.shell.BG_FOOTER_D)
            pixel: fn() {
                return mix(self.bg_light, self.bg_dark, self.dark_mode)
            }
        }

        padding: 12
        flow: Right
        align: Align{y: 0.5}
        spacing: 16

        status_label := Label{
            draw_text +: {
                dark_mode: instance(0.0)
                text_style: mod.widgets.shell.FONT_REGULAR{font_size: 11.0}
                get_color: fn() {
                    // Light: gray-600, Dark: slate-400
                    let light = vec4(0.294, 0.333, 0.388, 1.0)
                    let dark = vec4(0.580, 0.639, 0.722, 1.0)
                    return mix(light, dark, self.dark_mode)
                }
            }
            text: "Footer - Timeline / Status Bar"
        }

        View{width: Fill}

        hint_label := Label{
            draw_text +: {
                dark_mode: instance(0.0)
                text_style: mod.widgets.shell.FONT_REGULAR{font_size: 10.0}
                get_color: fn() {
                    // Light: gray-400, Dark: slate-500
                    let light = vec4(0.612, 0.639, 0.686, 1.0)
                    let dark = vec4(0.392, 0.455, 0.545, 1.0)
                    return mix(light, dark, self.dark_mode)
                }
            }
            text: "Drag top edge to resize"
        }
    }
}

/// Shell footer widget
#[derive(Script, ScriptHook, Widget)]
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
            self.view.label(cx, ids!(status_label)).set_text(cx, &self.status);
        }
        if !self.hint.is_empty() {
            self.view.label(cx, ids!(hint_label)).set_text(cx, &self.hint);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShellFooterRef {
    pub fn set_status(&self, cx: &mut Cx, status: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.status = status.to_string();
            inner.view.label(cx, ids!(status_label)).set_text(cx, status);
        }
    }

    pub fn set_hint(&self, cx: &mut Cx, hint: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hint = hint.to_string();
            inner.view.label(cx, ids!(hint_label)).set_text(cx, hint);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            script_apply_eval!(cx, inner.view, {
                draw_bg +: {dark_mode: #(dark_mode)}
            });
            for path in [ids!(status_label), ids!(hint_label)] {
                let mut label = inner.view.label(cx, path);
                script_apply_eval!(cx, label, {
                    draw_text +: {dark_mode: #(dark_mode)}
                });
            }
        }
    }
}
