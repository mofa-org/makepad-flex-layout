# Makepad App Shell - Architecture Documentation

A reusable IDE/studio layout shell library for Makepad applications.

## Table of Contents

1. [Overview](#overview)
2. [Module Structure](#module-structure)
3. [Widget Hierarchy](#widget-hierarchy)
4. [Core Types](#core-types)
5. [Event and Action Flow](#event-and-action-flow)
6. [Drag and Drop System](#drag-and-drop-system)
7. [Theme System](#theme-system)
8. [Persistence](#persistence)
9. [Public API](#public-api)
10. [Design Patterns](#design-patterns)
11. [Integration Guide](#integration-guide)

---

## Overview

The Makepad App Shell is a modular library providing a complete IDE-style layout with:

- **Draggable panel grid** with 3 rows x 9 slots
- **Footer panel strip** with vertical stacking support
- **Resizable sidebars** (left/right)
- **Dark/light theme** with smooth animated transitions
- **Layout persistence** to JSON
- **Customizable header** with action buttons

---

## Module Structure

```
src/
├── lib.rs                 # Public API exports and live_design registration
├── live_design.rs         # Base color and style definitions
├── callbacks.rs           # ShellCallbacks trait for app integration
├── persistence.rs         # JSON save/load to platform config directory
│
├── shell/                 # Main layout components
│   ├── mod.rs
│   ├── layout.rs          # ShellLayout - main container widget
│   ├── header.rs          # ShellHeader - top bar with controls
│   ├── footer.rs          # ShellFooter - bottom status bar
│   ├── sidebar.rs         # ShellSidebar - collapsible side panels
│   └── config.rs          # ShellConfig with builder pattern
│
├── panel/                 # Individual panel widgets
│   ├── mod.rs
│   ├── panel.rs           # Panel - draggable window with title bar
│   └── actions.rs         # PanelAction enum for events
│
├── grid/                  # Layout containers
│   ├── mod.rs
│   ├── panel_grid.rs      # PanelGrid - main 3x9 grid with drag-drop
│   ├── footer_grid.rs     # FooterGrid - horizontal strip with stacking
│   ├── layout_state.rs    # Serializable layout state structs
│   └── drop_handler.rs    # Drop position calculation utilities
│
└── theme/                 # Theming system
    ├── mod.rs             # ShellTheme struct and animation
    ├── colors.rs          # Tailwind-inspired color palette
    └── styles.rs          # Text styles and spacing constants
```

---

## Widget Hierarchy

### Visual Layout

```
ShellLayout (main container)
│
├── ShellHeader (48px, fixed top)
│   ├── Title label
│   ├── Theme toggle button
│   ├── Reset layout button
│   └── Save layout button
│
└── Dock (splitter-based layout)
    │
    ├── Left Sidebar (ShellSidebar, 280px)
    │
    ├── Center Area
    │   └── PanelGrid (3 rows x 9 slots)
    │       ├── Row 1: Panel slots s1_1 through s1_9
    │       ├── Row 2: Panel slots s2_1 through s2_9
    │       └── Row 3: Panel slots s3_1 through s3_9
    │
    ├── Right Sidebar (ShellSidebar, 300px)
    │
    └── Footer Area
        └── FooterGrid
            ├── Controller Sidebar (200px)
            └── Panel Strip (7 slots, each with up to 5 stacked panels)
                ├── f1_0: [p0, p1, p2, p3, p4]
                ├── f1_1: [p0, p1, p2, p3, p4]
                └── ... through f1_6
```

### Panel Composition

```
Panel (draggable window)
│
├── title_bar (32px height)
│   ├── drag_handle (6-dot grip icon, 16x20)
│   ├── title (Label)
│   ├── Spacer (Fill)
│   ├── fullscreen_btn (for footer panels)
│   ├── restore_fullscreen_btn
│   ├── max_btn (for main grid panels)
│   ├── restore_btn
│   └── close_btn
│
└── content (Fill)
    └── content_label (panel number display)
```

---

## Core Types

### Widget Types

| Type | Location | Purpose |
|------|----------|---------|
| `ShellLayout` | `shell/layout.rs` | Main container; manages theme, persistence, layout state |
| `ShellHeader` | `shell/header.rs` | Top bar with title and action buttons |
| `ShellSidebar` | `shell/sidebar.rs` | Collapsible side panel |
| `Panel` | `panel/panel.rs` | Draggable window with title bar and content |
| `PanelGrid` | `grid/panel_grid.rs` | 3x9 grid container with drag-drop support |
| `FooterGrid` | `grid/footer_grid.rs` | Horizontal panel strip with vertical stacking |

### State Types

| Type | Location | Purpose |
|------|----------|---------|
| `LayoutState` | `grid/layout_state.rs` | Serializable main grid state |
| `FooterLayoutState` | `grid/layout_state.rs` | Serializable footer grid state |
| `FooterSlotState` | `grid/layout_state.rs` | Single footer slot with panel IDs |
| `LayoutMode` | `grid/layout_state.rs` | AutoGrid, HStack, VStack, Tabbed |
| `DropPosition` | `grid/drop_handler.rs` | Calculated drop target (row, col, rect) |
| `ShellTheme` | `theme/mod.rs` | Dark mode state and animation progress |
| `ShellConfig` | `shell/config.rs` | Builder-pattern configuration |
| `ShellPreferences` | `persistence.rs` | Persisted user preferences |

### Action Types

```rust
pub enum PanelAction {
    Close(LiveId),                          // Panel close button clicked
    Maximize(LiveId),                       // Maximize/restore (main grid)
    Fullscreen(LiveId),                     // Fullscreen (footer grid)
    StartDrag(LiveId),                      // Drag threshold exceeded
    EndDrag(LiveId, DVec2),                 // Finger released with position
    LayoutChanged(LayoutState),             // Main grid layout changed
    FooterLayoutChanged(FooterLayoutState), // Footer layout changed
    None,
}

pub enum ShellHeaderAction {
    ToggleDarkMode,
    ResetLayout,
    SaveLayout,
    None,
}
```

---

## Event and Action Flow

### Hierarchical Event Processing

```
User Input (touch/mouse)
    │
    ▼
Panel.handle_event()
    │
    ├── Button clicks → emit PanelAction (Close, Maximize, Fullscreen)
    │
    └── Drag on drag_handle/title_bar
        ├── FingerDown → store drag_start
        ├── FingerMove (distance > 10px) → emit StartDrag
        └── FingerUp (if dragging) → emit EndDrag(id, position)
            │
            ▼
PanelGrid/FooterGrid.handle_event()
    │
    ├── StartDrag → set dragging_panel = Some(id)
    ├── EndDrag → calculate drop position, move panel, emit LayoutChanged
    ├── Close → remove panel from layout
    └── Maximize/Fullscreen → toggle state
        │
        ▼
ShellLayout.handle_event()
    │
    ├── LayoutChanged → store for persistence
    ├── FooterLayoutChanged → store for persistence
    └── ShellHeaderAction
        ├── ToggleDarkMode → animate theme transition
        ├── ResetLayout → restore defaults
        └── SaveLayout → write to disk
```

### Action Capture Pattern

```rust
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    // 1. Capture actions from child widgets
    let actions = cx.capture_actions(|cx| {
        self.view.handle_event(cx, event, scope);
    });

    // 2. Process captured actions
    for action in actions.iter() {
        match action.as_widget_action().cast::<PanelAction>() {
            PanelAction::Close(id) => { /* handle */ }
            // ...
        }
    }

    // 3. Emit actions to parent
    cx.widget_action(
        self.widget_uid(),
        &scope.path,
        PanelAction::LayoutChanged(self.layout_state.clone()),
    );
}
```

---

## Drag and Drop System

### Event Flow

```
Panel                           PanelGrid
  │                                │
  │ FingerDown on drag_handle      │
  ├────────────────────────────────┤
  │ Store drag_start position      │
  │                                │
  │ FingerMove (distance > 10px)   │
  ├──── StartDrag(panel_id) ──────►│
  │                                │ dragging_panel = Some(id)
  │                                │
  │ FingerMove (during drag)       │
  │                                │ hits_with_capture_overload()
  │                                │ Update drop_state preview
  │                                │ Draw blue overlay rectangle
  │                                │
  │ FingerUp                       │
  ├──── EndDrag(id, abs_pos) ─────►│
  │                                │ calculate_drop_position()
  │                                │ layout_state.move_panel()
  │                                │ emit LayoutChanged
  │                                │
```

### Why EndDrag Action is Required

Panel captures the finger on FingerDown via `event.hits()`. Subsequent FingerMove and FingerUp events go to Panel, not PanelGrid. The `hits_with_capture_overload` cannot retroactively steal capture.

**Solution**: Panel emits `EndDrag(panel_id, cursor_position)` on FingerUp, allowing PanelGrid to complete the drop via the action system.

### Drop Position Calculation

```rust
pub fn calculate_drop_position(
    abs: DVec2,                    // Cursor position
    container_rect: Rect,          // Grid container bounds
    rows_with_panels: &[Vec<u64>], // Panel IDs per visible row
    row_to_actual: &[usize],       // Visual row → actual row mapping
) -> Option<DropPosition> {
    // 1. Calculate visual row from Y position
    let row_height = container_rect.size.y / num_rows;
    let visual_row = (rel_y / row_height) as usize;

    // 2. Calculate column from X position
    let col_width = container_rect.size.x / cols_in_row;
    let col = (rel_x / col_width) as usize;

    // 3. Build preview rectangle
    Some(DropPosition { row, col, rect })
}
```

---

## Theme System

### Architecture

```rust
pub struct ShellTheme {
    pub dark_mode: bool,      // Target state
    pub dark_mode_anim: f64,  // Animation progress (0.0 = light, 1.0 = dark)
}
```

### Shader-Based Theming

All widgets use shader instance variables for theme colors:

```glsl
instance dark_mode: 0.0

fn pixel(self) -> vec4 {
    let light = vec4(1.0, 1.0, 1.0, 1.0);      // White
    let dark = vec4(0.059, 0.090, 0.165, 1.0); // Dark slate
    return mix(light, dark, self.dark_mode);
}
```

### Animation Flow

```
toggle_dark_mode()
    │
    ├── dark_mode = !dark_mode
    ├── dark_mode_animating = true
    ├── dark_mode_anim_start = now()
    └── request NextFrame
        │
        ▼
    Each NextFrame:
        │
        ├── elapsed = now() - anim_start
        ├── t = clamp(elapsed / DURATION, 0.0, 1.0)
        ├── eased_t = ease_out_cubic(t)  // 1 - (1-t)³
        ├── dark_mode_anim = lerp(start, target, eased_t)
        ├── apply_theme(cx)  // Update all shader instances
        │
        └── if t < 1.0: request another NextFrame
            else: dark_mode_animating = false
```

### Applying Theme to Widgets

```rust
fn apply_theme(&mut self, cx: &mut Cx) {
    let dm = self.theme.dark_mode_anim;

    // Apply to view background
    self.view.apply_over(cx, live! {
        draw_bg: { dark_mode: (dm) }
    });

    // Apply to child widgets
    self.view.shell_sidebar(id!(left_sidebar)).apply_dark_mode(cx, dm);
    self.view.panel_grid(id!(center_content)).apply_dark_mode(cx, dm);
    self.view.footer_grid(id!(footer_content)).apply_dark_mode(cx, dm);
}
```

---

## Persistence

### Storage Location

```
Platform        Path
────────        ────
macOS           ~/Library/Application Support/{app_id}/shell_preferences.json
Linux           ~/.config/{app_id}/shell_preferences.json
Windows         %APPDATA%/{app_id}/shell_preferences.json
```

### Data Structure

```rust
#[derive(Serialize, Deserialize)]
pub struct ShellPreferences {
    pub dark_mode: bool,
    pub layout: Option<LayoutState>,
    pub footer_layout: Option<FooterLayoutState>,
    pub splitter_positions: Option<SplitterPositions>,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutState {
    pub row_assignments: Vec<Vec<u64>>,  // Panel IDs per row
    pub visible_panels: Vec<u64>,        // Currently visible panels
    pub maximized_panel: Option<u64>,    // Maximized panel ID
    pub layout_mode: LayoutMode,
    pub selected_tab: usize,
}

#[derive(Serialize, Deserialize)]
pub struct FooterLayoutState {
    pub slots: Vec<FooterSlotState>,
    pub fullscreen_panel: Option<u64>,
}
```

### Load/Save Flow

```
Startup:
    ShellLayout.draw_walk() [first call]
        └── load_preferences(APP_ID)
            └── Read JSON from disk
            └── Apply dark_mode
            └── Apply layout to PanelGrid
            └── Apply footer_layout to FooterGrid

Save:
    User clicks "Save Layout"
        └── ShellHeaderAction::SaveLayout
            └── ShellLayout.save_layout()
                └── Collect current states
                └── Serialize to JSON
                └── Write to disk
```

---

## Public API

### Main Exports

```rust
// lib.rs
pub mod prelude {
    pub use crate::theme::{ShellTheme, ThemeListener};
    pub use crate::shell::config::ShellConfig;
    pub use crate::panel::{Panel, PanelAction};
    pub use crate::grid::{PanelGrid, FooterGrid, LayoutState, FooterLayoutState};
    pub use crate::callbacks::ShellCallbacks;
    pub use crate::persistence::ShellPreferences;
}

pub mod widgets {
    pub use crate::shell::layout::{ShellLayout, ShellLayoutRef};
    pub use crate::shell::header::{ShellHeader, ShellHeaderRef};
    pub use crate::shell::footer::{ShellFooter, ShellFooterRef};
    pub use crate::shell::sidebar::{ShellSidebar, ShellSidebarRef};
    pub use crate::panel::panel::{Panel, PanelRef};
    pub use crate::grid::panel_grid::{PanelGrid, PanelGridRef};
    pub use crate::grid::footer_grid::{FooterGrid, FooterGridRef};
}

pub fn live_design(cx: &mut Cx);  // Register all widgets
```

### Key Methods

**ShellLayout**
```rust
pub fn toggle_dark_mode(&mut self, cx: &mut Cx)
pub fn set_dark_mode(&mut self, cx: &mut Cx, dark: bool)
pub fn is_dark_mode(&self) -> bool
pub fn reset_layout(&mut self, cx: &mut Cx)
pub fn save_layout(&mut self, cx: &mut Cx)
```

**PanelGrid / FooterGrid**
```rust
pub fn layout_state(&self) -> &LayoutState
pub fn set_layout_state(&mut self, cx: &mut Cx, state: LayoutState)
pub fn reset_layout(&mut self, cx: &mut Cx)
pub fn apply_dark_mode(&self, cx: &mut Cx, dark_mode: f64)
```

**ShellConfig (Builder)**
```rust
ShellConfig::builder()
    .title("My App")
    .window_size(1400.0, 900.0)
    .dark_mode()
    .enable_persistence()
    .build()
```

---

## Design Patterns

### 1. Deref Widget Pattern

Widgets wrap Makepad's `View` and delegate standard operations:

```rust
#[derive(Live, LiveHook, Widget)]
pub struct Panel {
    #[deref]
    view: View,  // Delegate Widget trait methods

    #[rust]
    panel_id: LiveId,

    #[live]
    closable: bool,
}
```

### 2. Widget Ref Extension Traits

Extension traits provide convenient access to child widgets:

```rust
pub trait PanelGridWidgetExt {
    fn panel_grid(&self, path: &[LiveId]) -> PanelGridRef;
}

impl PanelGridWidgetExt for WidgetRef {
    fn panel_grid(&self, path: &[LiveId]) -> PanelGridRef {
        self.widget(path).as_panel_grid()
    }
}
```

### 3. Apply Over for Dynamic Updates

Update shader instances at runtime without recompilation:

```rust
self.view.apply_over(cx, live! {
    draw_bg: { dark_mode: (0.5) }  // Animate to 50%
});
```

### 4. Action Emission and Capture

```rust
// Capture from children
let actions = cx.capture_actions(|cx| {
    self.view.handle_event(cx, event, scope);
});

// Emit to parent
cx.widget_action(self.widget_uid(), &scope.path, MyAction::Something);
```

---

## Integration Guide

### Basic Integration

```rust
use makepad_widgets::*;
use makepad_app_shell::prelude::*;

live_design! {
    use makepad_app_shell::widgets::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                body = <ShellLayout> {}
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_app_shell::live_design(cx);  // Register shell widgets
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
```

### Dependencies

```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "rik" }
makepad-app-shell = { path = "../makepad-app-shell" }
```

---

## File Summary

| File | Lines | Purpose |
|------|-------|---------|
| `lib.rs` | ~50 | Public exports, live_design registration |
| `shell/layout.rs` | ~400 | Main container, theme animation, persistence |
| `shell/header.rs` | ~350 | Header bar with action buttons |
| `panel/panel.rs` | ~570 | Draggable panel widget |
| `panel/actions.rs` | ~40 | PanelAction enum |
| `grid/panel_grid.rs` | ~500 | Main grid with drag-drop |
| `grid/footer_grid.rs` | ~680 | Footer grid with vertical stacking |
| `grid/layout_state.rs` | ~210 | Serializable state structures |
| `theme/mod.rs` | ~100 | Theme state and animation |
| `theme/colors.rs` | ~170 | Color palette |
| `persistence.rs` | ~110 | JSON save/load |

**Total**: ~4,500 lines of Rust code
