//! Flex Layout Demo Application
//!
//! Demonstrates a fully resizable studio layout with Makepad.
//!
//! ## Features
//! - Drag-and-drop window reordering between rows
//! - Dynamic row sizing based on window count
//! - Visual drop preview during drag operations
//! - Maximize/restore individual windows
//! - Close windows with automatic layout reconfiguration
//!
//! ## Architecture
//! - `SubWindow`: Individual draggable window panel with title bar
//! - `ContentArea`: Grid container managing window layout and drop handling
//! - `row_assignments`: Per-row window ID lists (source of truth for layout)
//!
//! See DRAG_DROP_IMPLEMENTATION.md for detailed documentation.

use makepad_widgets::*;
use makepad_widgets::file_tree::*;

// ============================================================================
// LIVE DESIGN
// ============================================================================

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ========================================
    // COLOR PALETTE
    // ========================================

    COLOR_BG_APP = #f0f0f0
    COLOR_BG_HEADER = #4080c0
    COLOR_BG_SIDEBAR = #80a0d0
    COLOR_BG_FOOTER = #60a060
    COLOR_BG_CONTENT = #e8e8f0
    COLOR_ACCENT = #2060a0
    COLOR_TEXT = #202020
    COLOR_TEXT_DIM = #606060
    COLOR_BORDER = #a0a0b0

    // Window colors for visual distinction
    WINDOW_COLORS = [
        #2d4a6d, #4a2d6d, #6d4a2d, #2d6d4a,
        #6d2d4a, #4a6d2d, #3d3d5d, #5d3d3d,
        #3d5d3d, #5d5d3d, #3d5d5d, #5d3d5d,
        #4d4d4d, #2d5d6d, #6d5d2d, #5d2d6d,
        #2d6d5d, #6d2d5d, #5d6d2d, #4d2d4d
    ]

    // ========================================
    // TEXT STYLES
    // ========================================

    TEXT_HEADER = <THEME_FONT_REGULAR> {
        font_size: 14.0
    }

    TEXT_LABEL = <THEME_FONT_REGULAR> {
        font_size: 11.0
    }

    TEXT_SMALL = <THEME_FONT_REGULAR> {
        font_size: 10.0
    }

    // ========================================
    // STYLED BUTTON
    // ========================================

    StudioButton = <Button> {
        width: Fit
        height: 28
        padding: { left: 12, right: 12 }

        draw_text: {
            text_style: <TEXT_LABEL> {}
            color: (COLOR_TEXT)
            fn get_color(self) -> vec4 {
                return mix(self.color, #fff, self.hover * 0.2);
            }
        }

        draw_bg: {
            color: #3a3a4a
            uniform color_hover: #4a4a5a

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0);
                let color = mix(self.color, self.color_hover, self.hover);
                sdf.fill(color);
                return sdf.result;
            }
        }
    }

    // ========================================
    // SUB-WINDOW WIDGET
    // ========================================

    SubWindow = {{SubWindow}} {
        width: 200
        height: 150

        show_bg: true
        draw_bg: {
            color: #2d4a6d
            uniform border_width: 2.0
            uniform border_color: #ffffff

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0
                );
                sdf.fill(self.color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }

        flow: Down
        padding: 0

        // Title bar
        title_bar = <View> {
            width: Fill
            height: 28
            padding: { left: 6, right: 6 }
            flow: Right
            align: { y: 0.5 }

            show_bg: true
            draw_bg: {
                color: #00000040
                fn pixel(self) -> vec4 {
                    return self.color;
                }
            }

            // Drag handle icon (6 dots in 2 columns)
            drag_handle = <View> {
                width: 16
                height: 20
                margin: { right: 6 }
                cursor: Hand

                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let dot_color = #ffffff60;
                        let dot_r = 1.5;

                        // Draw 6 dots (2 columns x 3 rows)
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
                    text_style: <TEXT_LABEL> {}
                    color: #ffffff
                }
                text: "Window 1"
            }

            <View> { width: Fill }

            max_btn = <Button> {
                width: 18
                height: 18
                padding: 0
                margin: { right: 6 }
                text: ""
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Draw square icon for maximize
                        let inset = 4.0;
                        sdf.rect(inset, inset, self.rect_size.x - inset * 2.0, self.rect_size.y - inset * 2.0);
                        let color = mix(#ffffff80, #44ff44, self.hover);
                        sdf.stroke(color, 1.5);
                        return sdf.result;
                    }
                }
            }

            restore_btn = <Button> {
                width: 18
                height: 18
                padding: 0
                margin: { right: 6 }
                visible: false
                text: ""
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let color = mix(#ffffff80, #44ff44, self.hover);
                        // Draw two overlapping squares for restore icon
                        let inset = 4.0;
                        let offset = 3.0;
                        // Back square (offset up-right)
                        sdf.rect(inset + offset, inset, self.rect_size.x - inset * 2.0 - offset, self.rect_size.y - inset * 2.0 - offset);
                        sdf.stroke(color, 1.2);
                        // Front square (offset down-left)
                        sdf.rect(inset, inset + offset, self.rect_size.x - inset * 2.0 - offset, self.rect_size.y - inset * 2.0 - offset);
                        sdf.stroke(color, 1.2);
                        return sdf.result;
                    }
                }
            }

            close_btn = <Button> {
                width: 18
                height: 18
                padding: 0
                margin: 0
                text: ""
                draw_bg: {
                    uniform line_color: #ffffff80
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Draw X icon
                        let inset = 5.0;
                        let color = mix(self.line_color, #ff4444, self.hover);
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

        // Content area
        content = <View> {
            width: Fill
            height: Fill
            padding: 10
            align: { x: 0.5, y: 0.5 }

            content_label = <Label> {
                draw_text: {
                    text_style: <TEXT_HEADER> {}
                    color: #ffffff60
                }
                text: "Content"
            }
        }
    }

    // ========================================
    // TAB BUTTON
    // ========================================

    TabButton = <Button> {
        width: Fit
        height: 32
        padding: { left: 16, right: 16 }

        draw_text: {
            text_style: <TEXT_LABEL> {}
            color: (COLOR_TEXT_DIM)
            fn get_color(self) -> vec4 {
                return mix(self.color, #fff, self.pressed + self.hover * 0.3);
            }
        }

        draw_bg: {
            color: #2a2a35
            color_selected: #3a3a4a
            radius: 0.0
            instance selected: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                let color = mix(self.color, self.color_selected, self.selected + self.hover * 0.3);
                sdf.fill(color);
                // Bottom border for selected
                if self.selected > 0.5 {
                    sdf.rect(0.0, self.rect_size.y - 2.0, self.rect_size.x, 2.0);
                    sdf.fill(#4a9eff);
                }
                return sdf.result;
            }
        }
    }

    // ========================================
    // CONTENT AREA (with layout modes)
    // ========================================

    ContentArea = {{ContentArea}} {
        width: Fill
        height: Fill
        padding: 8

        show_bg: true
        draw_bg: { color: (COLOR_BG_CONTENT) }

        // Drop preview overlay
        drop_preview: {
            draw_depth: 10.0
            color: #4080c080
        }

        // Container with explicit row structure for precise layout
        // Each row has 9 slots to allow all windows in one row if desired
        window_container = <View> {
            width: Fill
            height: Fill
            flow: Down

            // Row 1: up to 9 windows
            row1 = <View> {
                width: Fill
                height: Fill
                flow: Right

                s1_1 = <SubWindow> { width: Fill, height: Fill }
                s1_2 = <SubWindow> { width: Fill, height: Fill }
                s1_3 = <SubWindow> { width: Fill, height: Fill }
                s1_4 = <SubWindow> { width: Fill, height: Fill }
                s1_5 = <SubWindow> { width: Fill, height: Fill }
                s1_6 = <SubWindow> { width: Fill, height: Fill }
                s1_7 = <SubWindow> { width: Fill, height: Fill }
                s1_8 = <SubWindow> { width: Fill, height: Fill }
                s1_9 = <SubWindow> { width: Fill, height: Fill }
            }

            // Row 2: up to 9 windows
            row2 = <View> {
                width: Fill
                height: Fill
                flow: Right

                s2_1 = <SubWindow> { width: Fill, height: Fill }
                s2_2 = <SubWindow> { width: Fill, height: Fill }
                s2_3 = <SubWindow> { width: Fill, height: Fill }
                s2_4 = <SubWindow> { width: Fill, height: Fill }
                s2_5 = <SubWindow> { width: Fill, height: Fill }
                s2_6 = <SubWindow> { width: Fill, height: Fill }
                s2_7 = <SubWindow> { width: Fill, height: Fill }
                s2_8 = <SubWindow> { width: Fill, height: Fill }
                s2_9 = <SubWindow> { width: Fill, height: Fill }
            }

            // Row 3: up to 9 windows
            row3 = <View> {
                width: Fill
                height: Fill
                flow: Right

                s3_1 = <SubWindow> { width: Fill, height: Fill }
                s3_2 = <SubWindow> { width: Fill, height: Fill }
                s3_3 = <SubWindow> { width: Fill, height: Fill }
                s3_4 = <SubWindow> { width: Fill, height: Fill }
                s3_5 = <SubWindow> { width: Fill, height: Fill }
                s3_6 = <SubWindow> { width: Fill, height: Fill }
                s3_7 = <SubWindow> { width: Fill, height: Fill }
                s3_8 = <SubWindow> { width: Fill, height: Fill }
                s3_9 = <SubWindow> { width: Fill, height: Fill }
            }
        }
    }

    // ========================================
    // LEFT SIDEBAR HEADER
    // ========================================

    LeftSidebarHeader = <View> {
        width: Fill
        height: 40
        padding: { left: 16 }
        align: { y: 0.5 }

        show_bg: true
        draw_bg: { color: #00000020 }

        <Label> {
            draw_text: {
                text_style: <TEXT_HEADER> {}
                color: (COLOR_TEXT)
            }
            text: "Blueprint"
        }
    }

    // ========================================
    // LEFT SIDEBAR (Blueprint Tree)
    // ========================================

    LeftSidebar = {{LeftSidebar}} {
        file_tree: <FileTree> {
            width: Fill
            height: Fill

            node_height: 24.0

            scroll_bars: <ScrollBars> {
                show_scroll_x: false
                show_scroll_y: true
            }

            file_node: <FileTreeNode> {
                is_folder: false
                indent_width: 14.0

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(
                            mix(#0000, #00000010, self.hover),
                            #00000020,
                            self.active
                        ));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <TEXT_SMALL> {}
                    fn get_color(self) -> vec4 {
                        return mix(#333, #000, self.active);
                    }
                }

                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return mix(#4080c0, #2060a0, self.hover);
                    }
                }
            }

            folder_node: <FileTreeNode> {
                is_folder: true
                indent_width: 14.0

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(
                            mix(#0000, #00000010, self.hover),
                            #00000020,
                            self.active
                        ));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <TEXT_SMALL> {}
                    fn get_color(self) -> vec4 {
                        return mix(#222, #000, self.active);
                    }
                }

                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return mix(#c08040, #a06020, self.hover);
                    }
                }
            }

            filler: {
                fn pixel(self) -> vec4 {
                    return #0000;
                }
            }
        }
    }

    // ========================================
    // RIGHT SIDEBAR HEADER
    // ========================================

    RightSidebarHeader = <View> {
        width: Fill
        height: Fit
        flow: Down

        // Selection Header
        <View> {
            width: Fill
            height: 40
            padding: { left: 16 }
            align: { y: 0.5 }

            show_bg: true
            draw_bg: { color: #00000020 }

            <Label> {
                draw_text: {
                    text_style: <TEXT_HEADER> {}
                    color: (COLOR_TEXT)
                }
                text: "Selection"
            }
        }

        // Selected item indicator
        <View> {
            width: Fill
            height: 32
            padding: { left: 12, right: 12 }
            align: { y: 0.5 }

            show_bg: true
            draw_bg: { color: #4080c0 }

            <Label> {
                draw_text: {
                    text_style: <TEXT_LABEL> {}
                    color: #fff
                }
                text: "so_arm100 - Window 1"
            }
        }
    }

    // ========================================
    // RIGHT SIDEBAR (Properties Tree)
    // ========================================

    RightSidebar = {{RightSidebar}} {
        file_tree: <FileTree> {
            width: Fill
            height: Fill

            node_height: 22.0

            scroll_bars: <ScrollBars> {
                show_scroll_x: false
                show_scroll_y: true
            }

            file_node: <FileTreeNode> {
                is_folder: false
                indent_width: 14.0

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(
                            mix(#0000, #00000008, self.hover),
                            #00000015,
                            self.active
                        ));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <TEXT_SMALL> {}
                    fn get_color(self) -> vec4 {
                        return mix(#444, #000, self.active);
                    }
                }

                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return mix(#4080c0, #2060a0, self.hover);
                    }
                }
            }

            folder_node: <FileTreeNode> {
                is_folder: true
                indent_width: 14.0

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(
                            mix(#0000, #00000008, self.hover),
                            #00000015,
                            self.active
                        ));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <TEXT_SMALL> {}
                    fn get_color(self) -> vec4 {
                        return mix(#333, #000, self.active);
                    }
                }

                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return mix(#c08040, #a06020, self.hover);
                    }
                }
            }

            filler: {
                fn pixel(self) -> vec4 {
                    return #0000;
                }
            }
        }
    }

    // ========================================
    // HEADER
    // ========================================

    StudioHeader = <View> {
        width: Fill
        height: 48

        show_bg: true
        draw_bg: { color: (COLOR_BG_HEADER) }

        padding: { left: 16, right: 16 }
        flow: Right
        align: { y: 0.5 }
        spacing: 16

        <Label> {
            draw_text: {
                text_style: { font_size: 16.0 }
                color: (COLOR_TEXT)
            }
            text: "Flex Layout Studio"
        }

        <View> { width: Fill }

        <Label> {
            draw_text: {
                text_style: <TEXT_SMALL> {}
                color: (COLOR_TEXT_DIM)
            }
            text: "Makepad Resizable Layout Demo"
        }
    }

    // ========================================
    // FOOTER
    // ========================================

    StudioFooter = <View> {
        width: Fill
        height: Fill

        show_bg: true
        draw_bg: { color: (COLOR_BG_FOOTER) }

        padding: 12
        flow: Right
        align: { y: 0.5 }
        spacing: 16

        <Label> {
            draw_text: {
                text_style: <TEXT_LABEL> {}
                color: (COLOR_TEXT_DIM)
            }
            text: "Footer - Timeline / Status Bar"
        }

        <View> { width: Fill }

        <Label> {
            draw_text: {
                text_style: <TEXT_SMALL> {}
                color: (COLOR_TEXT_DIM)
            }
            text: "Drag top edge to resize"
        }
    }

    // ========================================
    // MAIN STUDIO LAYOUT
    // ========================================

    // Inner splitter: Center + Right
    CenterRightSplit = <Splitter> {
        width: Fill
        height: Fill
        axis: Horizontal
        align: FromB(300.0)

        a = <ContentArea> {}
        b = <RightSidebar> {}
    }

    // Middle section: Left + Center/Right
    MiddleSplit = <Splitter> {
        width: Fill
        height: Fill
        axis: Horizontal
        align: FromA(280.0)

        a = <LeftSidebar> {}
        b = <CenterRightSplit> {}
    }

    // Use Dock widget with vertical splitter for footer
    StudioLayout = {{StudioLayout}} {
        width: Fill
        height: Fill
        flow: Down

        show_bg: true
        draw_bg: { color: (COLOR_BG_APP) }

        // Fixed header
        <StudioHeader> {}

        // Main area using Dock with both horizontal and vertical splitters
        <Dock> {
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
                kind: left_sidebar
            }

            center_panel = Tab {
                name: ""
                kind: center_content
            }

            right_panel = Tab {
                name: ""
                kind: right_sidebar
            }

            footer_panel = Tab {
                name: ""
                kind: footer_content
            }

            left_sidebar = <View> {
                width: Fill
                height: Fill
                flow: Down
                show_bg: true
                draw_bg: { color: (COLOR_BG_SIDEBAR) }

                <LeftSidebarHeader> {}
                <LeftSidebar> {}
            }
            center_content = <ContentArea> {}
            right_sidebar = <View> {
                width: Fill
                height: Fill
                flow: Down
                show_bg: true
                draw_bg: { color: (COLOR_BG_SIDEBAR) }

                <RightSidebarHeader> {}
                <RightSidebar> {}
            }
            footer_content = <StudioFooter> {}
        }
    }

    // ========================================
    // APP ROOT
    // ========================================

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "Flex Layout Studio Demo"
                    inner_size: vec2(1400, 900)
                }

                show_bg: true
                draw_bg: { color: (COLOR_BG_APP) }

                body = <StudioLayout> {}
            }
        }
    }
}

// ============================================================================
// WIDGET IMPLEMENTATIONS
// ============================================================================

// ────────────────────────────────────────────────────────────────────────────
// SubWindow Action
// ────────────────────────────────────────────────────────────────────────────

/// Actions emitted by SubWindow widgets to communicate with parent containers.
/// These are dispatched via `cx.widget_action()` and handled by ContentArea.
#[derive(Clone, Debug, DefaultNone)]
pub enum SubWindowAction {
    /// Window close button clicked - contains window_id
    Close(usize),
    /// Maximize/restore button clicked - contains window_id
    Maximize(usize),
    /// Drag operation started (threshold exceeded) - contains window_id
    StartDrag(usize),
    None,
}

// ────────────────────────────────────────────────────────────────────────────
// SubWindow Widget
// ────────────────────────────────────────────────────────────────────────────

/// A draggable sub-window panel with title bar, maximize/close buttons, and content area.
///
/// ## Drag Behavior
/// - Drag can be initiated from the drag handle icon or title bar
/// - A 10-pixel threshold prevents accidental drags
/// - Emits `SubWindowAction::StartDrag` when drag threshold is exceeded
///
/// ## Visual Updates
/// Uses a `needs_visual_update` flag pattern to defer visual changes to the draw phase,
/// ensuring proper integration with Makepad's rendering pipeline.
#[derive(Live, LiveHook, Widget)]
pub struct SubWindow {
    #[deref]
    view: View,

    #[rust]
    window_id: usize,

    #[rust]
    is_maximized: bool,

    #[rust]
    is_dragging: bool,

    #[rust]
    drag_start: DVec2,

    /// Flag indicating window_id changed and needs visual update
    #[rust]
    needs_visual_update: bool,
}

impl Widget for SubWindow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Check for close button click
        if self.view.button(id!(title_bar.close_btn)).clicked(&actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SubWindowAction::Close(self.window_id),
            );
        }

        // Check for maximize button click (either max or restore)
        if self.view.button(id!(title_bar.max_btn)).clicked(&actions)
            || self.view.button(id!(title_bar.restore_btn)).clicked(&actions)
        {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SubWindowAction::Maximize(self.window_id),
            );
        }

        // Handle drag on drag handle or title bar
        let drag_handle = self.view.view(id!(title_bar.drag_handle));
        let title_bar = self.view.view(id!(title_bar));

        // Check drag handle first (higher priority)
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
                        SubWindowAction::StartDrag(self.window_id),
                    );
                }
                handled = true;
            }
            Hit::FingerUp(_) => {
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
                            SubWindowAction::StartDrag(self.window_id),
                        );
                    }
                }
                Hit::FingerUp(_) => {
                    self.is_dragging = false;
                }
                _ => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Apply any pending visual updates
        self.apply_visual_update(cx);

        // Show/hide max and restore buttons based on state
        self.view.button(id!(title_bar.max_btn)).set_visible(cx, !self.is_maximized);
        self.view.button(id!(title_bar.restore_btn)).set_visible(cx, self.is_maximized);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl SubWindow {
    pub fn set_window_id(&mut self, cx: &mut Cx, id: usize) {
        // Skip if same ID
        if self.window_id == id {
            return;
        }

        self.window_id = id;
        self.needs_visual_update = true;
        self.view.redraw(cx);
    }

    /// Apply visual updates based on window_id (called from draw_walk)
    fn apply_visual_update(&mut self, cx: &mut Cx2d) {
        if !self.needs_visual_update {
            return;
        }
        self.needs_visual_update = false;

        let id = self.window_id;

        // Distinct color palette for each window
        let colors = [
            vec4(0.8, 0.2, 0.2, 1.0),   // 0: Red
            vec4(0.2, 0.7, 0.2, 1.0),   // 1: Green
            vec4(0.2, 0.4, 0.8, 1.0),   // 2: Blue
            vec4(0.8, 0.7, 0.2, 1.0),   // 3: Yellow
            vec4(0.7, 0.2, 0.7, 1.0),   // 4: Magenta
            vec4(0.2, 0.7, 0.7, 1.0),   // 5: Cyan
            vec4(0.9, 0.5, 0.2, 1.0),   // 6: Orange
            vec4(0.5, 0.2, 0.8, 1.0),   // 7: Purple
            vec4(0.4, 0.8, 0.4, 1.0),   // 8: Light green
        ];
        let color = colors[id % colors.len()];

        // Apply color to background
        self.view.apply_over(cx, live! {
            draw_bg: { color: (color) }
        });

        // Update title
        let title = format!("Window {}", id + 1);
        self.view.label(id!(title_bar.title)).set_text(cx, &title);

        // Update content label
        let content = format!("#{}", id + 1);
        self.view.label(id!(content.content_label)).set_text(cx, &content);
    }
}

impl SubWindowRef {
    pub fn set_window_id(&self, cx: &mut Cx, id: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_window_id(cx, id);
        }
    }

    pub fn set_maximized(&self, maximized: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_maximized = maximized;
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Layout Mode Enum
// ────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LayoutMode {
    #[default]
    AutoGrid,
    HStack,
    VStack,
    Tabbed,
}

impl LayoutMode {
    pub fn name(&self) -> &'static str {
        match self {
            LayoutMode::AutoGrid => "Auto Grid",
            LayoutMode::HStack => "Horizontal",
            LayoutMode::VStack => "Vertical",
            LayoutMode::Tabbed => "Tabbed",
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// ContentArea Widget
// ────────────────────────────────────────────────────────────────────────────

/// Drop position calculated during drag operations.
/// Contains the target row/column and a rect for visual preview.
#[derive(Clone, Debug)]
struct DropPosition {
    /// Target row index (0, 1, or 2)
    row: usize,
    /// Target column index within the row
    col: usize,
    /// Rectangle for drawing drop preview overlay
    rect: Rect,
}

/// Container widget managing a grid of SubWindow widgets with drag-and-drop support.
///
/// ## Layout Model
/// Uses `row_assignments: [Vec<usize>; 3]` as the source of truth for layout.
/// Each row maintains its own list of window IDs, enabling true physical movement
/// of windows between rows.
///
/// ## Key Methods
/// - `find_drop_position`: Calculates drop target from cursor position
/// - `handle_drop`: Moves window from source row to target row
/// - `apply_row_layout`: Updates visibility and sizing based on row_assignments
///
/// ## Slot System
/// Each row has 9 pre-defined slots (s1_1 through s1_9, etc.). Windows are
/// assigned to slots dynamically based on row_assignments. Unused slots are
/// hidden with `width: 0, height: 0`.
#[derive(Live, LiveHook, Widget)]
pub struct ContentArea {
    #[deref]
    view: View,

    #[live]
    sub_window: Option<LivePtr>,

    #[live]
    drop_preview: DrawColor,

    #[rust]
    layout_mode: LayoutMode,

    #[rust]
    window_count: usize,

    #[rust]
    selected_tab: usize,

    #[rust]
    initialized: bool,

    /// Row assignments - each row contains a list of window IDs in order
    /// This is the source of truth for layout
    #[rust]
    row_assignments: [Vec<usize>; 3],

    /// Which windows are visible (closed windows are marked false)
    #[rust]
    window_visible: [bool; 9],

    #[rust]
    maximized_window: Option<usize>,

    #[rust]
    needs_layout_update: bool,

    /// Currently dragging window ID
    #[rust]
    dragging_window: Option<usize>,

    /// Current drop target position
    #[rust]
    drop_state: Option<DropPosition>,

}

impl Widget for ContentArea {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Handle SubWindow actions
        for action in actions.iter() {
            if let SubWindowAction::Close(id) = action.as_widget_action().cast() {
                self.close_window(cx, id);
            }
            if let SubWindowAction::Maximize(id) = action.as_widget_action().cast() {
                self.toggle_maximize(cx, id);
            }
            if let SubWindowAction::StartDrag(id) = action.as_widget_action().cast() {
                self.dragging_window = Some(id);
            }
        }

        // Handle internal drag via hits on the view
        // We need to capture finger events even when dragging
        match event.hits_with_capture_overload(cx, self.view.area(), self.dragging_window.is_some()) {
            Hit::FingerMove(fe) if self.dragging_window.is_some() => {
                // Update drop preview based on cursor position
                if let Some(pos) = self.find_drop_position(cx, fe.abs) {
                    self.drop_state = Some(pos);
                } else {
                    self.drop_state = None;
                }
                self.view.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if let Some(dragged_id) = self.dragging_window {
                    self.handle_drop(cx, fe.abs, dragged_id);
                }
                self.dragging_window = None;
                self.drop_state = None;
                self.view.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize on first draw
        if !self.initialized {
            self.initialized = true;
            // Initialize with 9 visible windows: 3 per row
            self.window_visible = [true; 9];
            self.row_assignments = [
                vec![0, 1, 2],  // Row 0: windows 0, 1, 2
                vec![3, 4, 5],  // Row 1: windows 3, 4, 5
                vec![6, 7, 8],  // Row 2: windows 6, 7, 8
            ];
            self.needs_layout_update = true;
        }

        // Apply layout before drawing
        if self.needs_layout_update {
            self.needs_layout_update = false;
            self.apply_row_layout(cx);
        }

        // Draw the main view
        let result = self.view.draw_walk(cx, scope, walk);

        // Draw drop preview overlay if dragging
        if let Some(ref pos) = self.drop_state {
            self.drop_preview.draw_abs(cx, pos.rect);
        }

        result
    }
}

impl ContentArea {
    pub fn set_layout_mode(&mut self, mode: LayoutMode) {
        self.layout_mode = mode;
        self.selected_tab = 0;
    }

    pub fn set_window_count(&mut self, count: usize) {
        self.window_count = count.clamp(1, 20);
        if self.selected_tab >= self.window_count {
            self.selected_tab = self.window_count - 1;
        }
    }

    pub fn window_count(&self) -> usize {
        self.window_count
    }

    fn close_window(&mut self, cx: &mut Cx, id: usize) {
        if id < 9 {
            self.window_visible[id] = false;
            // Remove from row_assignments
            for row in &mut self.row_assignments {
                if let Some(pos) = row.iter().position(|&w| w == id) {
                    row.remove(pos);
                    break;
                }
            }
            // If closing the maximized window, exit maximize mode
            if self.maximized_window == Some(id) {
                self.maximized_window = None;
            }
            self.needs_layout_update = true;
            self.view.redraw(cx);
        }
    }

    fn toggle_maximize(&mut self, cx: &mut Cx, id: usize) {
        if self.maximized_window == Some(id) {
            self.maximized_window = None;
        } else {
            self.maximized_window = Some(id);
        }
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Get visible windows for a specific row
    fn visible_windows_in_row(&self, row: usize) -> Vec<usize> {
        if row >= 3 {
            return vec![];
        }
        self.row_assignments[row].iter()
            .filter(|&&id| self.window_visible[id])
            .copied()
            .collect()
    }

    /// Find which row contains a window
    fn find_window_row(&self, window_id: usize) -> Option<(usize, usize)> {
        for (row_idx, row) in self.row_assignments.iter().enumerate() {
            if let Some(col_idx) = row.iter().position(|&id| id == window_id) {
                return Some((row_idx, col_idx));
            }
        }
        None
    }

    /// Find the drop position based on cursor location
    fn find_drop_position(&self, cx: &Cx, abs: DVec2) -> Option<DropPosition> {
        // Get visible windows per row
        let rows_with_windows: Vec<Vec<usize>> = (0..3)
            .map(|r| self.visible_windows_in_row(r))
            .filter(|row| !row.is_empty())
            .collect();

        let num_rows = rows_with_windows.len();
        if num_rows == 0 {
            return None;
        }

        // Get the container rect
        let container = self.view.view(id!(window_container));
        let container_rect = container.area().rect(cx);

        if !container_rect.contains(abs) {
            return None;
        }

        // Calculate which row the cursor is in
        let row_height = container_rect.size.y / num_rows as f64;
        let rel_y = abs.y - container_rect.pos.y;
        let visual_row = ((rel_y / row_height) as usize).min(num_rows - 1);

        // Map visual row back to actual row index (0, 1, or 2)
        let mut actual_row = 0;
        let mut visual_count = 0;
        for r in 0..3 {
            if !self.visible_windows_in_row(r).is_empty() {
                if visual_count == visual_row {
                    actual_row = r;
                    break;
                }
                visual_count += 1;
            }
        }

        // Calculate which column within that row
        let cols_in_row = rows_with_windows[visual_row].len().max(1);
        let col_width = container_rect.size.x / cols_in_row as f64;
        let rel_x = abs.x - container_rect.pos.x;
        let col = ((rel_x / col_width) as usize).min(cols_in_row);

        // Calculate the preview rectangle for this slot
        // If dropping at the end of a row, show preview at end position
        let preview_col = col.min(cols_in_row - 1);
        let rect = Rect {
            pos: DVec2 {
                x: container_rect.pos.x + preview_col as f64 * col_width,
                y: container_rect.pos.y + visual_row as f64 * row_height,
            },
            size: DVec2 {
                x: col_width,
                y: row_height,
            },
        };

        Some(DropPosition { row: actual_row, col, rect })
    }

    /// Handle a drop operation - move window to new row/position
    fn handle_drop(&mut self, cx: &mut Cx, abs: DVec2, dragged_window_id: usize) {
        let Some(drop_pos) = self.find_drop_position(cx, abs) else {
            return;
        };

        // Find current position of dragged window
        let Some((src_row, src_col)) = self.find_window_row(dragged_window_id) else {
            return;
        };

        let target_row = drop_pos.row;
        let target_col = drop_pos.col;

        // Don't do anything if dropping at the same position
        if src_row == target_row && src_col == target_col {
            return;
        }

        // Remove window from source row
        self.row_assignments[src_row].remove(src_col);

        // Calculate insert position in target row
        let visible_in_target = self.visible_windows_in_row(target_row).len();
        let insert_col = target_col.min(visible_in_target);

        // If same row but target was after source, adjust for removal
        let insert_col = if src_row == target_row && target_col > src_col {
            insert_col.saturating_sub(1)
        } else {
            insert_col
        };

        // Insert window at target position
        self.row_assignments[target_row].insert(insert_col, dragged_window_id);

        // Trigger layout update
        self.needs_layout_update = true;
        self.view.redraw(cx);
    }

    /// Apply row-based layout using visibility and Fill sizing
    /// Each row shows only the windows assigned to it, and rows with no windows are hidden
    fn apply_row_layout(&mut self, cx: &mut Cx) {
        // Slot IDs organized by row (9 slots per row)
        let row_slot_ids = [
            [
                id!(window_container.row1.s1_1),
                id!(window_container.row1.s1_2),
                id!(window_container.row1.s1_3),
                id!(window_container.row1.s1_4),
                id!(window_container.row1.s1_5),
                id!(window_container.row1.s1_6),
                id!(window_container.row1.s1_7),
                id!(window_container.row1.s1_8),
                id!(window_container.row1.s1_9),
            ],
            [
                id!(window_container.row2.s2_1),
                id!(window_container.row2.s2_2),
                id!(window_container.row2.s2_3),
                id!(window_container.row2.s2_4),
                id!(window_container.row2.s2_5),
                id!(window_container.row2.s2_6),
                id!(window_container.row2.s2_7),
                id!(window_container.row2.s2_8),
                id!(window_container.row2.s2_9),
            ],
            [
                id!(window_container.row3.s3_1),
                id!(window_container.row3.s3_2),
                id!(window_container.row3.s3_3),
                id!(window_container.row3.s3_4),
                id!(window_container.row3.s3_5),
                id!(window_container.row3.s3_6),
                id!(window_container.row3.s3_7),
                id!(window_container.row3.s3_8),
                id!(window_container.row3.s3_9),
            ],
        ];

        let row_view_ids = [
            id!(window_container.row1),
            id!(window_container.row2),
            id!(window_container.row3),
        ];

        // Get visible windows per row
        let visible_per_row: [Vec<usize>; 3] = [
            self.visible_windows_in_row(0),
            self.visible_windows_in_row(1),
            self.visible_windows_in_row(2),
        ];

        let total_visible: usize = visible_per_row.iter().map(|r| r.len()).sum();

        const SLOTS_PER_ROW: usize = 9;

        // Handle maximized window
        if let Some(max_id) = self.maximized_window {
            // Hide all slots and rows first
            for row_idx in 0..3 {
                for slot_idx in 0..SLOTS_PER_ROW {
                    self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                        visible: false, width: 0, height: 0
                    });
                }
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: false, height: 0
                });
            }

            // Find which row contains the maximized window
            if let Some((row_idx, _col_idx)) = self.find_window_row(max_id) {
                // Show only that row
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: true, height: Fill
                });

                // Show only the maximized window's slot (use first slot in that row)
                self.view.view(row_slot_ids[row_idx][0]).apply_over(cx, live! {
                    visible: true, width: Fill, height: Fill
                });
                self.view.sub_window(row_slot_ids[row_idx][0]).set_window_id(cx, max_id);
                self.view.sub_window(row_slot_ids[row_idx][0]).set_maximized(true);
            }
            return;
        }

        // Auto-maximize if only 1 window left
        if total_visible == 1 {
            // Hide all first
            for row_idx in 0..3 {
                for slot_idx in 0..SLOTS_PER_ROW {
                    self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                        visible: false, width: 0, height: 0
                    });
                }
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: false, height: 0
                });
            }

            // Find the only visible window
            for row_idx in 0..3 {
                if !visible_per_row[row_idx].is_empty() {
                    let window_id = visible_per_row[row_idx][0];
                    self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                        visible: true, height: Fill
                    });
                    self.view.view(row_slot_ids[row_idx][0]).apply_over(cx, live! {
                        visible: true, width: Fill, height: Fill
                    });
                    self.view.sub_window(row_slot_ids[row_idx][0]).set_window_id(cx, window_id);
                    break;
                }
            }
            return;
        }

        // Normal layout: each row shows its assigned windows
        // First hide all slots
        for row_idx in 0..3 {
            for slot_idx in 0..SLOTS_PER_ROW {
                self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                    visible: false, width: 0, height: 0
                });
                self.view.sub_window(row_slot_ids[row_idx][slot_idx]).set_maximized(false);
            }
        }

        // Configure each row
        for row_idx in 0..3 {
            let windows_in_row = &visible_per_row[row_idx];

            if windows_in_row.is_empty() {
                // Hide empty rows
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: false, height: 0
                });
            } else {
                // Show row
                self.view.view(row_view_ids[row_idx]).apply_over(cx, live! {
                    visible: true, height: Fill
                });

                // Show slots for windows in this row (up to 9 slots per row)
                for (slot_idx, &window_id) in windows_in_row.iter().take(SLOTS_PER_ROW).enumerate() {
                    self.view.view(row_slot_ids[row_idx][slot_idx]).apply_over(cx, live! {
                        visible: true, width: Fill, height: Fill
                    });
                    self.view.sub_window(row_slot_ids[row_idx][slot_idx]).set_window_id(cx, window_id);
                }
            }
        }
    }
}

impl ContentAreaRef {
    pub fn set_layout_mode(&self, cx: &mut Cx, mode: LayoutMode) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_layout_mode(mode);
            inner.view.redraw(cx);
        }
    }

    pub fn set_window_count(&self, cx: &mut Cx, count: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_window_count(count);
            inner.view.redraw(cx);
        }
    }

    pub fn window_count(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.window_count()
        } else {
            0
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// LeftSidebar Widget (Blueprint Tree)
// ────────────────────────────────────────────────────────────────────────────

// Simple file node structure for demo data
#[derive(Debug)]
pub struct DemoFileEdge {
    pub name: String,
    pub file_node_id: LiveId,
}

#[derive(Debug)]
pub struct DemoFileNode {
    pub name: String,
    pub child_edges: Option<Vec<DemoFileEdge>>,
}

impl DemoFileNode {
    pub fn is_folder(&self) -> bool {
        self.child_edges.is_some()
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct LeftSidebar {
    #[wrap]
    #[live]
    pub file_tree: FileTree,

    #[rust]
    pub file_nodes: LiveIdMap<LiveId, DemoFileNode>,

    #[rust]
    initialized: bool,
}

impl LeftSidebar {
    fn draw_file_node(cx: &mut Cx2d, file_node_id: LiveId, file_tree: &mut FileTree, file_nodes: &LiveIdMap<LiveId, DemoFileNode>) {
        if let Some(file_node) = file_nodes.get(&file_node_id) {
            match &file_node.child_edges {
                Some(child_edges) => {
                    if file_tree.begin_folder(cx, file_node_id, &file_node.name).is_ok() {
                        for child_edge in child_edges {
                            Self::draw_file_node(cx, child_edge.file_node_id, file_tree, file_nodes);
                        }
                        file_tree.end_folder();
                    }
                }
                None => {
                    file_tree.file(cx, file_node_id, &file_node.name);
                }
            }
        }
    }

    fn init_demo_data(&mut self) {
        // Clear existing data
        self.file_nodes.clear();

        // Build the tree structure:
        // root (Viewport)
        //   └── world (/ root)
        //       └── so_arm100
        //           └── base
        //               └── link1 (1)
        //                   ├── collision_0
        //                   ├── visual_0
        //                   └── visual_1
        //           └── transforms

        // Leaf files
        self.file_nodes.insert(live_id!(collision_0), DemoFileNode {
            name: "collision_0".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(visual_0), DemoFileNode {
            name: "visual_0".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(visual_1), DemoFileNode {
            name: "visual_1".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(transforms), DemoFileNode {
            name: "transforms".to_string(),
            child_edges: None,
        });

        // link1 folder
        self.file_nodes.insert(live_id!(link1), DemoFileNode {
            name: "1".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "collision_0".to_string(), file_node_id: live_id!(collision_0) },
                DemoFileEdge { name: "visual_0".to_string(), file_node_id: live_id!(visual_0) },
                DemoFileEdge { name: "visual_1".to_string(), file_node_id: live_id!(visual_1) },
            ]),
        });

        // base folder
        self.file_nodes.insert(live_id!(base), DemoFileNode {
            name: "base".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "1".to_string(), file_node_id: live_id!(link1) },
            ]),
        });

        // so_arm100 folder
        self.file_nodes.insert(live_id!(so_arm100), DemoFileNode {
            name: "so_arm100".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "base".to_string(), file_node_id: live_id!(base) },
                DemoFileEdge { name: "transforms".to_string(), file_node_id: live_id!(transforms) },
            ]),
        });

        // world folder (/ root)
        self.file_nodes.insert(live_id!(world), DemoFileNode {
            name: "/ (root)".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "so_arm100".to_string(), file_node_id: live_id!(so_arm100) },
            ]),
        });

        // root folder (Viewport)
        self.file_nodes.insert(live_id!(root), DemoFileNode {
            name: "Viewport (Tab container)".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "world".to_string(), file_node_id: live_id!(world) },
            ]),
        });
    }
}

impl Widget for LeftSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.file_tree.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize data on first draw
        if !self.initialized {
            self.init_demo_data();
            self.initialized = true;
        }

        while self.file_tree.draw_walk(cx, scope, walk).is_step() {
            self.file_tree.set_folder_is_open(cx, live_id!(root).into(), true, Animate::No);
            Self::draw_file_node(
                cx,
                live_id!(root).into(),
                &mut self.file_tree,
                &self.file_nodes,
            );
        }
        DrawStep::done()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// RightSidebar Widget (Selection / Properties)
// ────────────────────────────────────────────────────────────────────────────

/// Right sidebar with Properties tree (similar to Rerun's Selection panel)
#[derive(Live, LiveHook, Widget)]
pub struct RightSidebar {
    #[wrap]
    #[live]
    pub file_tree: FileTree,

    #[rust]
    pub file_nodes: LiveIdMap<LiveId, DemoFileNode>,

    #[rust]
    initialized: bool,
}

impl RightSidebar {
    fn draw_file_node(cx: &mut Cx2d, file_node_id: LiveId, file_tree: &mut FileTree, file_nodes: &LiveIdMap<LiveId, DemoFileNode>) {
        if let Some(file_node) = file_nodes.get(&file_node_id) {
            match &file_node.child_edges {
                Some(child_edges) => {
                    if file_tree.begin_folder(cx, file_node_id, &file_node.name).is_ok() {
                        for child_edge in child_edges {
                            Self::draw_file_node(cx, child_edge.file_node_id, file_tree, file_nodes);
                        }
                        file_tree.end_folder();
                    }
                }
                None => {
                    file_tree.file(cx, file_node_id, &file_node.name);
                }
            }
        }
    }

    fn init_demo_data(&mut self) {
        self.file_nodes.clear();

        // Build Properties tree:
        // root
        //   ├── Data
        //   │   ├── Recording ID
        //   │   ├── Application ID
        //   │   └── ...
        //   └── Properties
        //       └── RecordingInfo
        //           └── start_time

        // Data leaf files
        self.file_nodes.insert(live_id!(recording_id), DemoFileNode {
            name: "Recording ID: rec_015db9e2".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(app_id), DemoFileNode {
            name: "Application ID: rerun_example".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(source), DemoFileNode {
            name: "Source: File via SDK".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(version), DemoFileNode {
            name: "Source RRD: 0.29.0-alpha".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(kind), DemoFileNode {
            name: "Kind: Recording".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(modified), DemoFileNode {
            name: "Modified: 08:31:42.427849Z".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(duration), DemoFileNode {
            name: "Duration: 0.14s".to_string(),
            child_edges: None,
        });
        self.file_nodes.insert(live_id!(size), DemoFileNode {
            name: "Size: 10.7 MiB".to_string(),
            child_edges: None,
        });

        // Data folder
        self.file_nodes.insert(live_id!(data), DemoFileNode {
            name: "Data".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "recording_id".to_string(), file_node_id: live_id!(recording_id) },
                DemoFileEdge { name: "app_id".to_string(), file_node_id: live_id!(app_id) },
                DemoFileEdge { name: "source".to_string(), file_node_id: live_id!(source) },
                DemoFileEdge { name: "version".to_string(), file_node_id: live_id!(version) },
                DemoFileEdge { name: "kind".to_string(), file_node_id: live_id!(kind) },
                DemoFileEdge { name: "modified".to_string(), file_node_id: live_id!(modified) },
                DemoFileEdge { name: "duration".to_string(), file_node_id: live_id!(duration) },
                DemoFileEdge { name: "size".to_string(), file_node_id: live_id!(size) },
            ]),
        });

        // RecordingInfo content
        self.file_nodes.insert(live_id!(start_time), DemoFileNode {
            name: "start_time: 08:31:42.283293Z".to_string(),
            child_edges: None,
        });

        // RecordingInfo folder
        self.file_nodes.insert(live_id!(recording_info), DemoFileNode {
            name: "RecordingInfo".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "start_time".to_string(), file_node_id: live_id!(start_time) },
            ]),
        });

        // Properties folder
        self.file_nodes.insert(live_id!(properties), DemoFileNode {
            name: "Properties".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "recording_info".to_string(), file_node_id: live_id!(recording_info) },
            ]),
        });

        // Root folder (Selection)
        self.file_nodes.insert(live_id!(selection_root), DemoFileNode {
            name: "Selection".to_string(),
            child_edges: Some(vec![
                DemoFileEdge { name: "data".to_string(), file_node_id: live_id!(data) },
                DemoFileEdge { name: "properties".to_string(), file_node_id: live_id!(properties) },
            ]),
        });
    }
}

impl Widget for RightSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.file_tree.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize data on first draw
        if !self.initialized {
            self.init_demo_data();
            self.initialized = true;
        }

        while self.file_tree.draw_walk(cx, scope, walk).is_step() {
            self.file_tree.set_folder_is_open(cx, live_id!(selection_root).into(), true, Animate::No);
            Self::draw_file_node(
                cx,
                live_id!(selection_root).into(),
                &mut self.file_tree,
                &self.file_nodes,
            );
        }
        DrawStep::done()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// StudioLayout Widget (Main Container)
// ────────────────────────────────────────────────────────────────────────────

#[derive(Live, LiveHook, Widget)]
pub struct StudioLayout {
    #[deref]
    view: View,
}

impl Widget for StudioLayout {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// App
// ────────────────────────────────────────────────────────────────────────────

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
