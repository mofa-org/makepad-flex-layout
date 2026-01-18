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

        // Hamburger menu button for overlay sidebar
        hamburger_btn = <Button> {
            width: 28
            height: 28
            margin: { right: 8 }
            text: ""

            draw_bg: {
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;

                    // Stroke color: dark in light mode, light in dark mode
                    let light_stroke = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark_stroke = vec4(0.945, 0.961, 0.976, 1.0);
                    let hover_stroke = vec4(0.231, 0.510, 0.965, 1.0);
                    let base = mix(light_stroke, dark_stroke, self.dark_mode);
                    let stroke = mix(base, hover_stroke, self.hover);
                    let line_width = 1.8;

                    // Draw three horizontal lines (hamburger icon)
                    let line_len = 14.0;
                    let gap = 5.0;

                    // Top line
                    sdf.move_to(cx - line_len * 0.5, cy - gap);
                    sdf.line_to(cx + line_len * 0.5, cy - gap);
                    sdf.stroke(stroke, line_width);

                    // Middle line
                    sdf.move_to(cx - line_len * 0.5, cy);
                    sdf.line_to(cx + line_len * 0.5, cy);
                    sdf.stroke(stroke, line_width);

                    // Bottom line
                    sdf.move_to(cx - line_len * 0.5, cy + gap);
                    sdf.line_to(cx + line_len * 0.5, cy + gap);
                    sdf.stroke(stroke, line_width);

                    return sdf.result;
                }
            }
        }

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
            text: "Makepad Flex App Layout Shell"
        }

        <View> { width: Fill }

        // Reset layout button (undo arrow with window)
        reset_btn = <Button> {
            width: 28
            height: 28
            margin: { left: 8 }
            text: ""

            draw_bg: {
                instance dark_mode: 0.0
                instance anim_progress: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;

                    // Stroke color with animation flash
                    let light_stroke = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark_stroke = vec4(0.945, 0.961, 0.976, 1.0);
                    let hover_stroke = vec4(0.231, 0.510, 0.965, 1.0);
                    let success_stroke = vec4(0.22, 0.80, 0.46, 1.0);  // green
                    let base = mix(light_stroke, dark_stroke, self.dark_mode);
                    let hovered = mix(base, hover_stroke, self.hover);
                    let stroke = mix(hovered, success_stroke, self.anim_progress);
                    let line_width = 1.4;

                    // Scale to fit 28x28 (original is 32x32)
                    let s = 0.7;
                    let ox = cx - 16.0 * s;
                    let oy = cy - 16.0 * s;

                    // Arrow pointing left (from path: M9 3 L3 9 L9 15)
                    // Arrow shaft from box to left
                    sdf.move_to(ox + 27.0 * s, oy + 8.0 * s);
                    sdf.line_to(ox + 7.0 * s, oy + 8.0 * s);
                    sdf.stroke(stroke, line_width);

                    // Arrow head
                    sdf.move_to(ox + 10.0 * s, oy + 4.0 * s);
                    sdf.line_to(ox + 4.0 * s, oy + 9.0 * s);
                    sdf.line_to(ox + 10.0 * s, oy + 14.0 * s);
                    sdf.stroke(stroke, line_width);

                    // Box/window (right side, going down then left then up)
                    // Right edge down
                    sdf.move_to(ox + 27.0 * s, oy + 10.0 * s);
                    sdf.line_to(ox + 27.0 * s, oy + 26.0 * s);
                    sdf.stroke(stroke, line_width);

                    // Bottom edge
                    sdf.move_to(ox + 27.0 * s, oy + 26.0 * s);
                    sdf.line_to(ox + 7.0 * s, oy + 26.0 * s);
                    sdf.stroke(stroke, line_width);

                    // Left edge up (partial)
                    sdf.move_to(ox + 7.0 * s, oy + 26.0 * s);
                    sdf.line_to(ox + 7.0 * s, oy + 19.0 * s);
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
                instance anim_progress: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let cx = self.rect_size.x * 0.5;
                    let cy = self.rect_size.y * 0.5;

                    // Stroke color with animation flash
                    let light_stroke = vec4(0.122, 0.161, 0.216, 1.0);
                    let dark_stroke = vec4(0.945, 0.961, 0.976, 1.0);
                    let hover_stroke = vec4(0.231, 0.510, 0.965, 1.0);
                    let success_stroke = vec4(0.22, 0.80, 0.46, 1.0);  // green
                    let base = mix(light_stroke, dark_stroke, self.dark_mode);
                    let hovered = mix(base, hover_stroke, self.hover);
                    let stroke = mix(hovered, success_stroke, self.anim_progress);
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
    HamburgerHoverIn,
    HamburgerHoverOut,
    HamburgerClicked,
    ResetLayout,
    SaveLayout,
    None,
}

/// Animation state for button feedback
#[derive(Clone, Debug, Default)]
pub struct ButtonAnimState {
    pub animating: bool,
    pub start_time: f64,
    pub progress: f64,
}

/// Shell header widget
#[derive(Live, LiveHook, Widget)]
pub struct ShellHeader {
    #[deref]
    view: View,

    #[live]
    title: String,

    #[rust]
    save_anim: ButtonAnimState,

    #[rust]
    reset_anim: ButtonAnimState,

    #[rust]
    hamburger_hovering: bool,
}

impl Widget for ShellHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle hamburger button hover and click
        let hamburger = self.view.button(id!(hamburger_btn));

        if hamburger.clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::HamburgerClicked,
            );
        }

        // Check for hover using Hit events
        let hamburger_area = hamburger.area();
        let hamburger_rect = hamburger_area.rect(cx);
        match event.hits(cx, hamburger_area) {
            Hit::FingerHoverIn(_) => {
                log!("header.rs - FingerHoverIn triggered, rect={:?}, was_hovering={}", hamburger_rect, self.hamburger_hovering);
                if !self.hamburger_hovering {
                    self.hamburger_hovering = true;
                    log!("header.rs - Emitting HamburgerHoverIn action");
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        ShellHeaderAction::HamburgerHoverIn,
                    );
                }
            }
            Hit::FingerHoverOut(_) => {
                log!("header.rs - FingerHoverOut triggered, rect={:?}, was_hovering={}", hamburger_rect, self.hamburger_hovering);
                self.hamburger_hovering = false;
            }
            _ => {}
        }

        if self.view.button(id!(theme_toggle)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::ToggleDarkMode,
            );
        }

        if self.view.button(id!(reset_btn)).clicked(&actions) {
            // Start reset animation
            self.reset_anim.animating = true;
            self.reset_anim.start_time = Cx::time_now();
            self.reset_anim.progress = 1.0;
            cx.new_next_frame();

            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::ResetLayout,
            );
        }

        if self.view.button(id!(save_btn)).clicked(&actions) {
            // Start save animation
            self.save_anim.animating = true;
            self.save_anim.start_time = Cx::time_now();
            self.save_anim.progress = 1.0;
            cx.new_next_frame();

            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ShellHeaderAction::SaveLayout,
            );
        }

        // Update animations
        if let Event::NextFrame(_) = event {
            let mut needs_redraw = false;
            let duration = 1.2;  // Animation duration in seconds

            if self.save_anim.animating {
                let elapsed = Cx::time_now() - self.save_anim.start_time;
                if elapsed < duration {
                    // Ease out cubic
                    let t = elapsed / duration;
                    self.save_anim.progress = 1.0 - (t * t * t);
                    self.view.button(id!(save_btn)).apply_over(cx, live! {
                        draw_bg: { anim_progress: (self.save_anim.progress) }
                    });
                    cx.new_next_frame();
                    needs_redraw = true;
                } else {
                    self.save_anim.animating = false;
                    self.save_anim.progress = 0.0;
                    self.view.button(id!(save_btn)).apply_over(cx, live! {
                        draw_bg: { anim_progress: 0.0 }
                    });
                    needs_redraw = true;
                }
            }

            if self.reset_anim.animating {
                let elapsed = Cx::time_now() - self.reset_anim.start_time;
                if elapsed < duration {
                    // Ease out cubic
                    let t = elapsed / duration;
                    self.reset_anim.progress = 1.0 - (t * t * t);
                    self.view.button(id!(reset_btn)).apply_over(cx, live! {
                        draw_bg: { anim_progress: (self.reset_anim.progress) }
                    });
                    cx.new_next_frame();
                    needs_redraw = true;
                } else {
                    self.reset_anim.animating = false;
                    self.reset_anim.progress = 0.0;
                    self.view.button(id!(reset_btn)).apply_over(cx, live! {
                        draw_bg: { anim_progress: 0.0 }
                    });
                    needs_redraw = true;
                }
            }

            if needs_redraw {
                self.view.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.title.is_empty() {
            self.view.label(id!(title_label)).set_text(cx, &self.title);
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

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.label(id!(title_label)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(hamburger_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
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
