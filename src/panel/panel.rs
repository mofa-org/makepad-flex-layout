//! Panel widget - a draggable window panel with title bar
//!
//! This is the core panel component for the app shell grid system.

use makepad_widgets::*;
use crate::panel::PanelAction;
use crate::theme::colors::panel_colors;
use crate::theme::get_global_dark_mode;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::live_design::*;

    pub Panel = {{Panel}} {
        width: 200
        height: 150

        closable: true
        maximizable: true
        fullscreenable: false

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            uniform border_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Square corners - no border radius
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);

                // Panel background - responds to dark_mode
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0);           // white
                let dark_bg = vec4(0.122, 0.161, 0.231, 1.0);      // slate-800
                let bg_color = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg_color);

                // Border
                let border_color = mix(
                    vec4(0.886, 0.910, 0.941, 1.0),  // slate-200
                    vec4(0.200, 0.255, 0.333, 1.0),  // slate-700
                    self.dark_mode
                );
                sdf.stroke(border_color, self.border_width);
                return sdf.result;
            }
        }

        flow: Down
        padding: 0

        // Title bar
        title_bar = <View> {
            width: Fill
            height: 32
            padding: { left: 8, right: 8 }
            flow: Right
            align: { y: 0.5 }

            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    // Light: slate-100, Dark: slate-700
                    let light = vec4(0.945, 0.961, 0.976, 1.0);
                    let dark = vec4(0.200, 0.255, 0.333, 1.0);
                    return mix(light, dark, self.dark_mode);
                }
            }

            // Drag handle icon (6 dots in 2 columns)
            drag_handle = <View> {
                width: 16
                height: 20
                margin: { right: 8 }
                cursor: Hand

                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let dot_r = 1.5;

                        // Dot color based on theme
                        let light_dot = vec4(0.580, 0.639, 0.722, 1.0);  // slate-400
                        let dark_dot = vec4(0.392, 0.455, 0.545, 1.0);   // slate-500
                        let dot_color = mix(light_dot, dark_dot, self.dark_mode);

                        let col1_x = 5.0;
                        let col2_x = 11.0;
                        let row1_y = 5.0;
                        let row2_y = 10.0;
                        let row3_y = 15.0;

                        sdf.circle(col1_x, row1_y, dot_r);
                        sdf.fill(dot_color);
                        sdf.circle(col2_x, row1_y, dot_r);
                        sdf.fill(dot_color);
                        sdf.circle(col1_x, row2_y, dot_r);
                        sdf.fill(dot_color);
                        sdf.circle(col2_x, row2_y, dot_r);
                        sdf.fill(dot_color);
                        sdf.circle(col1_x, row3_y, dot_r);
                        sdf.fill(dot_color);
                        sdf.circle(col2_x, row3_y, dot_r);
                        sdf.fill(dot_color);

                        return sdf.result;
                    }
                }
            }

            title = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM> { font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        // Light: gray-700, Dark: slate-200
                        let light = vec4(0.247, 0.282, 0.333, 1.0);
                        let dark = vec4(0.886, 0.910, 0.941, 1.0);
                        return mix(light, dark, self.dark_mode);
                    }
                }
                text: "Panel"
            }

            <View> { width: Fill }

            // Fullscreen button (arrows pointing outward)
            fullscreen_btn = <Button> {
                width: 20
                height: 20
                padding: 0
                margin: { right: 4 }
                visible: false
                text: ""
                draw_bg: {
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let inset = 5.0;
                        let arrow_len = 4.0;

                        let light_color = vec4(0.420, 0.447, 0.502, 1.0);
                        let dark_color = vec4(0.580, 0.639, 0.722, 1.0);
                        let hover_color = vec4(0.231, 0.510, 0.965, 1.0);
                        let base = mix(light_color, dark_color, self.dark_mode);
                        let color = mix(base, hover_color, self.hover);

                        // Four corners with arrows pointing outward
                        // Top-left arrow
                        sdf.move_to(inset, inset + arrow_len);
                        sdf.line_to(inset, inset);
                        sdf.line_to(inset + arrow_len, inset);
                        sdf.stroke(color, 1.2);

                        // Top-right arrow
                        sdf.move_to(self.rect_size.x - inset - arrow_len, inset);
                        sdf.line_to(self.rect_size.x - inset, inset);
                        sdf.line_to(self.rect_size.x - inset, inset + arrow_len);
                        sdf.stroke(color, 1.2);

                        // Bottom-left arrow
                        sdf.move_to(inset, self.rect_size.y - inset - arrow_len);
                        sdf.line_to(inset, self.rect_size.y - inset);
                        sdf.line_to(inset + arrow_len, self.rect_size.y - inset);
                        sdf.stroke(color, 1.2);

                        // Bottom-right arrow
                        sdf.move_to(self.rect_size.x - inset - arrow_len, self.rect_size.y - inset);
                        sdf.line_to(self.rect_size.x - inset, self.rect_size.y - inset);
                        sdf.line_to(self.rect_size.x - inset, self.rect_size.y - inset - arrow_len);
                        sdf.stroke(color, 1.2);

                        return sdf.result;
                    }
                }
            }

            // Restore from fullscreen button (arrows pointing inward)
            restore_fullscreen_btn = <Button> {
                width: 20
                height: 20
                padding: 0
                margin: { right: 4 }
                visible: false
                text: ""
                draw_bg: {
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let inset = 5.0;
                        let arrow_len = 4.0;
                        let cx = self.rect_size.x / 2.0;
                        let cy = self.rect_size.y / 2.0;

                        let light_color = vec4(0.420, 0.447, 0.502, 1.0);
                        let dark_color = vec4(0.580, 0.639, 0.722, 1.0);
                        let hover_color = vec4(0.231, 0.510, 0.965, 1.0);
                        let base = mix(light_color, dark_color, self.dark_mode);
                        let color = mix(base, hover_color, self.hover);

                        // Four corners with arrows pointing inward (toward center)
                        // Top-left pointing to center
                        sdf.move_to(inset, inset);
                        sdf.line_to(cx - 2.0, cy - 2.0);
                        sdf.stroke(color, 1.2);

                        // Top-right pointing to center
                        sdf.move_to(self.rect_size.x - inset, inset);
                        sdf.line_to(cx + 2.0, cy - 2.0);
                        sdf.stroke(color, 1.2);

                        // Bottom-left pointing to center
                        sdf.move_to(inset, self.rect_size.y - inset);
                        sdf.line_to(cx - 2.0, cy + 2.0);
                        sdf.stroke(color, 1.2);

                        // Bottom-right pointing to center
                        sdf.move_to(self.rect_size.x - inset, self.rect_size.y - inset);
                        sdf.line_to(cx + 2.0, cy + 2.0);
                        sdf.stroke(color, 1.2);

                        return sdf.result;
                    }
                }
            }

            max_btn = <Button> {
                width: 20
                height: 20
                padding: 0
                margin: { right: 4 }
                text: ""
                draw_bg: {
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let inset = 5.0;

                        // Icon color
                        let light_color = vec4(0.420, 0.447, 0.502, 1.0);  // gray-500
                        let dark_color = vec4(0.580, 0.639, 0.722, 1.0);   // slate-400
                        let hover_color = vec4(0.231, 0.510, 0.965, 1.0);  // blue-500
                        let base = mix(light_color, dark_color, self.dark_mode);
                        let color = mix(base, hover_color, self.hover);

                        sdf.rect(inset, inset, self.rect_size.x - inset * 2.0, self.rect_size.y - inset * 2.0);
                        sdf.stroke(color, 1.5);
                        return sdf.result;
                    }
                }
            }

            restore_btn = <Button> {
                width: 20
                height: 20
                padding: 0
                margin: { right: 4 }
                visible: false
                text: ""
                draw_bg: {
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let inset = 5.0;
                        let offset = 2.0;

                        let light_color = vec4(0.420, 0.447, 0.502, 1.0);
                        let dark_color = vec4(0.580, 0.639, 0.722, 1.0);
                        let hover_color = vec4(0.231, 0.510, 0.965, 1.0);
                        let base = mix(light_color, dark_color, self.dark_mode);
                        let color = mix(base, hover_color, self.hover);

                        // Back square
                        sdf.rect(inset + offset, inset, self.rect_size.x - inset * 2.0 - offset, self.rect_size.y - inset * 2.0 - offset);
                        sdf.stroke(color, 1.2);
                        // Front square
                        sdf.rect(inset, inset + offset, self.rect_size.x - inset * 2.0 - offset, self.rect_size.y - inset * 2.0 - offset);
                        sdf.stroke(color, 1.2);
                        return sdf.result;
                    }
                }
            }

            close_btn = <Button> {
                width: 20
                height: 20
                padding: 0
                margin: 0
                text: ""
                draw_bg: {
                    instance dark_mode: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let inset = 6.0;

                        let light_color = vec4(0.420, 0.447, 0.502, 1.0);
                        let dark_color = vec4(0.580, 0.639, 0.722, 1.0);
                        let hover_color = vec4(0.937, 0.267, 0.267, 1.0);  // red-500
                        let base = mix(light_color, dark_color, self.dark_mode);
                        let color = mix(base, hover_color, self.hover);

                        sdf.move_to(inset, inset);
                        sdf.line_to(self.rect_size.x - inset, self.rect_size.y - inset);
                        sdf.stroke(color, 1.5);
                        sdf.move_to(self.rect_size.x - inset, inset);
                        sdf.line_to(inset, self.rect_size.y - inset);
                        sdf.stroke(color, 1.5);
                        return sdf.result;
                    }
                }
            }
        }

        // Content area - empty slot for user content injection
        content = <View> {
            width: Fill
            height: Fill
            // Empty - content injected at runtime or via live_design
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Panel {
    #[deref]
    view: View,

    /// LiveId for internal identification (hash of panel_id_str)
    #[rust]
    panel_id: LiveId,

    /// Semantic string ID for this panel (e.g., "editor", "console")
    /// This is stored separately because LiveId is a hash and can't be reversed
    #[rust]
    panel_id_str: String,

    #[live]
    title: String,

    #[live]
    closable: bool,

    #[live]
    maximizable: bool,

    #[live]
    fullscreenable: bool,

    #[rust]
    panel_index: usize,

    #[rust]
    is_maximized: bool,

    #[rust]
    is_fullscreen: bool,

    #[rust]
    is_dragging: bool,

    #[rust]
    drag_start: DVec2,

    #[rust]
    needs_visual_update: bool,

    /// Reference to user-provided content widget (for programmatic injection)
    #[rust]
    content_widget: Option<WidgetRef>,
}

impl Widget for Panel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Push panel ID to scope path so content can identify which panel it's in
        let actions = scope.with_id(self.panel_id, |scope| {
            cx.capture_actions(|cx| {
                self.view.handle_event(cx, event, scope);
            })
        });

        if self.view.button(id!(title_bar.close_btn)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                PanelAction::Close(self.panel_id),
            );
        }

        if self.view.button(id!(title_bar.max_btn)).clicked(&actions)
            || self.view.button(id!(title_bar.restore_btn)).clicked(&actions)
        {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                PanelAction::Maximize(self.panel_id),
            );
        }

        if self.view.button(id!(title_bar.fullscreen_btn)).clicked(&actions)
            || self.view.button(id!(title_bar.restore_fullscreen_btn)).clicked(&actions)
        {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                PanelAction::Fullscreen(self.panel_id),
            );
        }

        let drag_handle = self.view.view(id!(title_bar.drag_handle));
        let title_bar = self.view.view(id!(title_bar));

        // Handle drag from drag_handle
        let mut handled = false;
        match event.hits(cx, drag_handle.area()) {
            Hit::FingerDown(fe) => {
                self.is_dragging = false;
                self.drag_start = fe.abs;
                handled = true;
            }
            Hit::FingerMove(fe) => {
                let dist = (fe.abs - self.drag_start).length();
                if !self.is_dragging && dist > 10.0 {
                    self.is_dragging = true;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        PanelAction::StartDrag(self.panel_id),
                    );
                }
                handled = true;
            }
            Hit::FingerUp(fe) => {
                if self.is_dragging {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        PanelAction::EndDrag(self.panel_id, fe.abs),
                    );
                }
                self.is_dragging = false;
                handled = true;
            }
            _ => {}
        }

        // Also allow dragging from title bar (excluding buttons area)
        if !handled {
            match event.hits(cx, title_bar.area()) {
                Hit::FingerDown(fe) => {
                    self.is_dragging = false;
                    self.drag_start = fe.abs;
                }
                Hit::FingerMove(fe) => {
                    if !self.is_dragging && (fe.abs - self.drag_start).length() > 10.0 {
                        self.is_dragging = true;
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            PanelAction::StartDrag(self.panel_id),
                        );
                    }
                }
                Hit::FingerUp(fe) => {
                    if self.is_dragging {
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            PanelAction::EndDrag(self.panel_id, fe.abs),
                        );
                    }
                    self.is_dragging = false;
                }
                _ => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Apply global theme on every draw
        let dm = get_global_dark_mode();
        self.apply_dark_mode_internal(cx, dm);

        self.apply_visual_update(cx);

        // Maximize buttons (for main grid)
        self.view.button(id!(title_bar.max_btn)).set_visible(cx, !self.is_maximized && self.maximizable);
        self.view.button(id!(title_bar.restore_btn)).set_visible(cx, self.is_maximized && self.maximizable);

        // Fullscreen buttons (for footer grid)
        self.view.button(id!(title_bar.fullscreen_btn)).set_visible(cx, !self.is_fullscreen && self.fullscreenable);
        self.view.button(id!(title_bar.restore_fullscreen_btn)).set_visible(cx, self.is_fullscreen && self.fullscreenable);

        // Close button
        self.view.button(id!(title_bar.close_btn)).set_visible(cx, self.closable);

        // Draw with panel ID in scope path so content can identify which panel it's in
        // Content widgets can access panel ID via: scope.path.from_end(0)
        scope.with_id(self.panel_id, |scope| {
            self.view.draw_walk(cx, scope, walk)
        })
    }
}

impl Panel {
    pub fn set_panel_index(&mut self, cx: &mut Cx, index: usize) {
        if self.panel_index == index {
            return;
        }
        self.panel_index = index;
        self.needs_visual_update = true;
        self.view.redraw(cx);
    }

    pub fn set_panel_id(&mut self, id: LiveId) {
        self.panel_id = id;
    }

    /// Set both the LiveId and string ID for this panel
    pub fn set_panel_id_str(&mut self, id_str: &str) {
        self.panel_id_str = id_str.to_string();
        self.panel_id = LiveId::from_str_lc(id_str);
    }

    /// Get the semantic string ID for this panel
    pub fn panel_id_str(&self) -> &str {
        &self.panel_id_str
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.is_maximized = maximized;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
    }

    /// Set custom content widget for this panel
    pub fn set_content(&mut self, widget: WidgetRef) {
        self.content_widget = Some(widget);
    }

    /// Get the content area view for adding children
    pub fn content_view(&self) -> ViewRef {
        self.view.view(id!(content))
    }

    /// Apply dark mode to this panel (internal, called during draw)
    fn apply_dark_mode_internal(&mut self, cx: &mut Cx, dark_mode: f64) {
        // Apply to main panel background
        self.view.apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to title bar
        self.view.view(id!(title_bar)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to drag handle
        self.view.view(id!(title_bar.drag_handle)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });

        // Apply to title label
        self.view.label(id!(title_bar.title)).apply_over(cx, live! {
            draw_text: { dark_mode: (dark_mode) }
        });

        // Apply to all title bar buttons
        self.view.button(id!(title_bar.close_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.button(id!(title_bar.max_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.button(id!(title_bar.restore_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.button(id!(title_bar.fullscreen_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
        self.view.button(id!(title_bar.restore_fullscreen_btn)).apply_over(cx, live! {
            draw_bg: { dark_mode: (dark_mode) }
        });
    }

    fn apply_visual_update(&mut self, cx: &mut Cx2d) {
        if !self.needs_visual_update {
            return;
        }
        self.needs_visual_update = false;

        let index = self.panel_index;
        let colors = panel_colors();
        let color = colors[index % colors.len()];

        self.view.apply_over(cx, live! {
            draw_bg: { panel_color: (color) }
        });

        let title = if self.title.is_empty() {
            format!("Panel {}", index + 1)
        } else {
            self.title.clone()
        };
        self.view.label(id!(title_bar.title)).set_text(cx, &title);
    }
}

impl PanelRef {
    pub fn set_panel_index(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_panel_index(cx, index);
        }
    }

    pub fn set_panel_id(&self, id: LiveId) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_panel_id(id);
        }
    }

    /// Set both the LiveId and string ID for this panel
    pub fn set_panel_id_str(&self, id_str: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_panel_id_str(id_str);
        }
    }

    /// Get the semantic string ID for this panel
    pub fn panel_id_str(&self) -> Option<String> {
        self.borrow().map(|inner| inner.panel_id_str().to_string())
    }

    pub fn set_maximized(&self, maximized: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_maximized(maximized);
        }
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_fullscreen(fullscreen);
        }
    }

    pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply to main panel background
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply to title bar
            inner.view.view(id!(title_bar)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply to drag handle
            inner.view.view(id!(title_bar.drag_handle)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply to title label
            inner.view.label(id!(title_bar.title)).apply_over(cx, live! {
                draw_text: { dark_mode: (dark_mode) }
            });

            // Apply to all title bar buttons
            inner.view.button(id!(title_bar.close_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(title_bar.max_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(title_bar.restore_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(title_bar.fullscreen_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
            inner.view.button(id!(title_bar.restore_fullscreen_btn)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });
        }
    }

    /// Set custom content widget for this panel
    pub fn set_content(&self, widget: WidgetRef) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_content(widget);
        }
    }

    /// Get the content area view for adding children
    pub fn content_view(&self) -> Option<ViewRef> {
        self.borrow().map(|inner| inner.content_view())
    }
}
