//! Shell header widget

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::live_design::*;

    pub ShellHeader = {{ShellHeader}} {
        width: Fill
        height: 48

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

        padding: { left: 16, right: 16 }
        flow: Right
        align: { y: 0.5 }
        spacing: 16

        title_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD> { font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    // Light: gray-800, Dark: slate-100
                    let light = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark = vec4(0.945, 0.961, 0.976, 1.0);
                    return mix(light, dark, self.dark_mode);
                }
            }
            text: "Flex App Layout Shell"
        }

        subtitle_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR> { font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    // Light: gray-500, Dark: slate-400
                    let light = vec4(0.420, 0.447, 0.502, 1.0);
                    let dark = vec4(0.580, 0.639, 0.722, 1.0);
                    return mix(light, dark, self.dark_mode);
                }
            }
            text: "Makepad Resizable Layout"
        }

        <View> { width: Fill }

        // Reset layout button
        reset_btn = <Button> {
            width: 28
            height: 28
            margin: { left: 8 }
            text: ""

            draw_bg: {
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;

                    // Stroke color
                    let light_stroke = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark_stroke = vec4(0.945, 0.961, 0.976, 1.0);
                    let hover_stroke = vec4(0.231, 0.510, 0.965, 1.0);
                    let base = mix(light_stroke, dark_stroke, self.dark_mode);
                    let stroke = mix(base, hover_stroke, self.hover);
                    let line_width = 1.5;

                    // Draw circular arrow (reset icon) - arc using line segments
                    let r = 7.0;

                    // Draw arc manually (no loops in shaders)
                    // Arc from ~-30 deg to ~240 deg (about 3/4 circle)
                    sdf.move_to(cx + r * 0.866, cy - r * 0.5);  // -30 deg
                    sdf.line_to(cx + r * 0.5, cy - r * 0.866);  // -60 deg
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx + r * 0.5, cy - r * 0.866);
                    sdf.line_to(cx, cy - r);  // -90 deg (top)
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx, cy - r);
                    sdf.line_to(cx - r * 0.5, cy - r * 0.866);
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx - r * 0.5, cy - r * 0.866);
                    sdf.line_to(cx - r * 0.866, cy - r * 0.5);
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx - r * 0.866, cy - r * 0.5);
                    sdf.line_to(cx - r, cy);  // 180 deg (left)
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx - r, cy);
                    sdf.line_to(cx - r * 0.866, cy + r * 0.5);
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx - r * 0.866, cy + r * 0.5);
                    sdf.line_to(cx - r * 0.5, cy + r * 0.866);
                    sdf.stroke(stroke, line_width);

                    sdf.move_to(cx - r * 0.5, cy + r * 0.866);
                    sdf.line_to(cx, cy + r);  // 270 deg (bottom)
                    sdf.stroke(stroke, line_width);

                    // Arrow head at top right
                    let ax = cx + r * 0.866;
                    let ay = cy - r * 0.5;
                    sdf.move_to(ax - 2.0, ay - 4.0);
                    sdf.line_to(ax, ay);
                    sdf.line_to(ax + 4.0, ay - 2.0);
                    sdf.stroke(stroke, line_width);

                    return sdf.result;
                }
            }
        }

        // Save layout button
        save_btn = <Button> {
            width: 28
            height: 28
            margin: { left: 4 }
            text: ""

            draw_bg: {
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;

                    // Stroke color
                    let light_stroke = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark_stroke = vec4(0.945, 0.961, 0.976, 1.0);
                    let hover_stroke = vec4(0.231, 0.510, 0.965, 1.0);
                    let base = mix(light_stroke, dark_stroke, self.dark_mode);
                    let stroke = mix(base, hover_stroke, self.hover);
                    let line_width = 1.5;

                    // Draw download/save icon
                    // Arrow pointing down
                    sdf.move_to(cx, cy - 5.0);
                    sdf.line_to(cx, cy + 3.0);
                    sdf.stroke(stroke, line_width);

                    // Arrow head
                    sdf.move_to(cx - 4.0, cy);
                    sdf.line_to(cx, cy + 4.0);
                    sdf.line_to(cx + 4.0, cy);
                    sdf.stroke(stroke, line_width);

                    // Tray/container at bottom
                    let tray_y = cy + 6.0;
                    let tray_w = 8.0;
                    let tray_h = 3.0;
                    sdf.move_to(cx - tray_w, tray_y - tray_h);
                    sdf.line_to(cx - tray_w, tray_y);
                    sdf.line_to(cx + tray_w, tray_y);
                    sdf.line_to(cx + tray_w, tray_y - tray_h);
                    sdf.stroke(stroke, line_width);

                    return sdf.result;
                }
            }
        }

        // Dark mode toggle button
        theme_toggle = <Button> {
            width: 32
            height: 28
            margin: { left: 8 }
            text: ""

            draw_bg: {
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;
                    let scale = 0.5;

                    // Stroke color: dark in light mode, light in dark mode
                    let light_stroke = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark_stroke = vec4(0.945, 0.961, 0.976, 1.0);
                    let stroke = mix(light_stroke, dark_stroke, self.dark_mode);
                    let line_width = 1.5;

                    // Sun (shown in light mode)
                    let sun_alpha = 1.0 - self.dark_mode;
                    if sun_alpha > 0.01 {
                        let sun_r = 6.0 * scale;
                        sdf.circle(cx, cy, sun_r);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        let ray_inner = 8.0 * scale;
                        let ray_outer = 11.0 * scale;

                        sdf.move_to(cx, cy - ray_inner);
                        sdf.line_to(cx, cy - ray_outer);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        sdf.move_to(cx, cy + ray_inner);
                        sdf.line_to(cx, cy + ray_outer);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        sdf.move_to(cx - ray_inner, cy);
                        sdf.line_to(cx - ray_outer, cy);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        sdf.move_to(cx + ray_inner, cy);
                        sdf.line_to(cx + ray_outer, cy);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        let diag = 0.707;
                        let di = ray_inner * diag;
                        let do_ = ray_outer * diag;

                        sdf.move_to(cx + di, cy - di);
                        sdf.line_to(cx + do_, cy - do_);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        sdf.move_to(cx - di, cy - di);
                        sdf.line_to(cx - do_, cy - do_);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        sdf.move_to(cx + di, cy + di);
                        sdf.line_to(cx + do_, cy + do_);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);

                        sdf.move_to(cx - di, cy + di);
                        sdf.line_to(cx - do_, cy + do_);
                        sdf.stroke(vec4(stroke.xyz, sun_alpha), line_width);
                    }

                    // Moon (shown in dark mode)
                    let moon_alpha = self.dark_mode;
                    if moon_alpha > 0.01 {
                        let moon_r = 9.0 * scale;
                        sdf.circle(cx, cy, moon_r);
                        sdf.stroke(vec4(stroke.xyz, moon_alpha), line_width);

                        let cut_r = 7.0 * scale;
                        let cut_offset_x = 6.0 * scale;
                        let cut_offset_y = -3.0 * scale;
                        sdf.circle(cx + cut_offset_x, cy + cut_offset_y, cut_r + line_width);
                        // Use header bg for cutout
                        let light_bg = vec4(1.0, 1.0, 1.0, 1.0);
                        let dark_bg = vec4(0.059, 0.090, 0.165, 1.0);
                        let bg = mix(light_bg, dark_bg, self.dark_mode);
                        sdf.fill(vec4(bg.xyz, moon_alpha));
                    }

                    return sdf.result;
                }
            }
        }
    }
}

/// Actions emitted by the shell header
#[derive(Clone, Debug, DefaultNone)]
pub enum ShellHeaderAction {
    ToggleDarkMode,
    ResetLayout,
    SaveLayout,
    None,
}

/// Shell header widget
#[derive(Live, LiveHook, Widget)]
pub struct ShellHeader {
    #[deref]
    view: View,

    #[live]
    title: String,

    #[live]
    subtitle: String,
}

impl Widget for ShellHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(id!(theme_toggle)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::ToggleDarkMode,
            );
        }

        if self.view.button(id!(reset_btn)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::ResetLayout,
            );
        }

        if self.view.button(id!(save_btn)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::SaveLayout,
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.title.is_empty() {
            self.view.label(id!(title_label)).set_text(cx, &self.title);
        }
        if !self.subtitle.is_empty() {
            self.view.label(id!(subtitle_label)).set_text(cx, &self.subtitle);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShellHeaderRef {
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.title = title.to_string();
            inner.view.label(id!(title_label)).set_text(cx, title);
        }
    }

    pub fn set_subtitle(&self, cx: &mut Cx, subtitle: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.subtitle = subtitle.to_string();
            inner.view.label(id!(subtitle_label)).set_text(cx, subtitle);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(title_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(subtitle_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(theme_toggle)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(reset_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(save_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
        }
    }
}
