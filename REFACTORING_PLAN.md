# Makepad App Shell - Refactoring Plan

## Executive Summary

Transform the monolithic `src/app.rs` (~1946 lines) into a **reusable, publishable Makepad app shell crate** that other applications can adopt. The shell provides:

- Dock-based resizable layout (header, footer, left/right sidebars, content area)
- Draggable panel grid with maximize/close functionality
- Dark/light theme switching with smooth animations
- Layout state persistence
- Extensible content injection via traits

---

## Current State

### File Structure
```
makepad-flex-layout/
├── Cargo.toml          # Binary crate
├── src/
│   ├── main.rs         # Entry point (13 lines)
│   └── app.rs          # Everything else (1946 lines)
```

### Problems with Current Architecture

| Issue | Impact | Location |
|-------|--------|----------|
| Monolithic file | Hard to navigate, maintain | `app.rs` - all 1946 lines |
| Hardcoded demo data | Sidebars have baked-in tree data | Lines 1643-1718, 1783-1875 |
| Fixed content | SubWindow only shows a Label | Lines 267-280 |
| No theming | Colors hardcoded in live_design | Lines 34-52 |
| No persistence | Layout resets on restart | N/A |
| No callbacks | Apps can't react to layout changes | N/A |
| Tight coupling | Can't use shell without demo | Entire file |

---

## Target Architecture

```
makepad-app-shell/
├── Cargo.toml                      # Library crate
├── src/
│   ├── lib.rs                      # Public API exports
│   ├── live_design.rs              # Base live_design (themeable widgets)
│   │
│   ├── theme/
│   │   ├── mod.rs                  # ShellTheme, ThemeListener trait
│   │   ├── colors.rs               # Tailwind-style color palette
│   │   └── styles.rs               # Text styles, spacing constants
│   │
│   ├── shell/
│   │   ├── mod.rs
│   │   ├── layout.rs               # ShellLayout widget (main container)
│   │   ├── header.rs               # ShellHeader widget
│   │   ├── footer.rs               # ShellFooter widget
│   │   ├── sidebar.rs              # ShellSidebar widget
│   │   └── config.rs               # ShellConfig struct
│   │
│   ├── panel/
│   │   ├── mod.rs
│   │   ├── panel.rs                # Panel widget (was SubWindow)
│   │   ├── content.rs              # PanelContent trait
│   │   └── actions.rs              # PanelAction enum
│   │
│   ├── grid/
│   │   ├── mod.rs
│   │   ├── panel_grid.rs           # PanelGrid widget (was ContentArea)
│   │   ├── layout_state.rs         # LayoutState (serializable)
│   │   └── drop_handler.rs         # Drag-drop logic
│   │
│   ├── callbacks.rs                # ShellCallbacks trait
│   └── persistence.rs              # Save/load preferences
│
└── examples/
    └── flex-layout-demo/           # Demo app (current functionality)
        ├── Cargo.toml
        └── src/
            ├── main.rs
            ├── app.rs              # Demo App using the shell
            ├── demo_panels.rs      # DemoPanelContent implementation
            └── demo_sidebars.rs    # Demo FileTree data
```

---

## Key Design Decisions

### 1. Theme System (from mofa-studio patterns)

**Approach**: Shader instance variables with `mix()` for runtime theme switching.

```rust
// In live_design - themeable widget
ThemeableView = <View> {
    show_bg: true
    draw_bg: {
        instance dark_mode: 0.0  // 0.0 = light, 1.0 = dark

        fn pixel(self) -> vec4 {
            let light = #ffffff;
            let dark = #1f293b;
            return mix(light, dark, self.dark_mode);
        }
    }
}

// Runtime update via apply_over()
widget.apply_over(cx, live!{
    draw_bg: { dark_mode: (0.5) }  // Animated value
});
```

**Why this approach**:
- No recompilation needed for theme changes
- Smooth animated transitions (300ms ease-out)
- GPU-accelerated color interpolation
- Proven pattern from mofa-studio production app

### 2. Content Injection

**Approach**: `Option<LivePtr>` for custom content in live_design.

```rust
#[derive(Live, LiveHook, Widget)]
pub struct Panel {
    #[live]
    content: Option<LivePtr>,  // Custom content widget
    // ...
}

// Usage in live_design
MyApp = {{MyApp}} {
    <ShellLayout> {
        panel_template: <MyCustomPanel> {
            // Custom content here
        }
    }
}
```

**Why this approach**:
- Works with Makepad's live system
- Compile-time type safety
- Declarative in live_design

### 3. Callbacks vs Actions

**Approach**: Both - trait for direct callbacks, actions for widget communication.

```rust
// Trait for app-level callbacks
pub trait ShellCallbacks: 'static {
    fn on_panel_closed(&mut self, cx: &mut Cx, panel_id: LiveId);
    fn on_layout_changed(&mut self, cx: &mut Cx, state: &LayoutState);
    fn on_dark_mode_changed(&mut self, cx: &mut Cx, dark_mode: bool);
}

// Actions for widget-to-widget communication
#[derive(Clone, Debug, DefaultNone)]
pub enum PanelAction {
    Close(LiveId),
    Maximize(LiveId),
    StartDrag(LiveId),
    None,
}
```

### 4. State Persistence

**Approach**: JSON file in platform config directory.

```rust
// Location: ~/.config/{app_id}/shell_preferences.json (Linux)
//           ~/Library/Application Support/{app_id}/shell_preferences.json (macOS)

#[derive(Serialize, Deserialize)]
pub struct ShellPreferences {
    pub dark_mode: bool,
    pub layout: Option<LayoutState>,
}

#[derive(Serialize, Deserialize)]
pub struct LayoutState {
    pub row_assignments: Vec<Vec<u64>>,
    pub visible_panels: Vec<u64>,
    pub maximized_panel: Option<u64>,
    pub splitter_positions: SplitterPositions,
}
```

---

## Public API

### Exported Types

```rust
// src/lib.rs

pub mod prelude {
    // Theme
    pub use crate::theme::{ShellTheme, ThemeListener};

    // Configuration
    pub use crate::shell::config::ShellConfig;

    // Panel
    pub use crate::panel::{Panel, PanelAction, PanelContent};

    // Grid
    pub use crate::grid::{PanelGrid, LayoutState};

    // Callbacks
    pub use crate::callbacks::ShellCallbacks;

    // Persistence
    pub use crate::persistence::ShellPreferences;
}

pub mod widgets {
    pub use crate::shell::layout::ShellLayout;
    pub use crate::shell::header::ShellHeader;
    pub use crate::shell::footer::ShellFooter;
    pub use crate::shell::sidebar::ShellSidebar;
    pub use crate::panel::Panel;
    pub use crate::grid::PanelGrid;
}

// Register live_design
pub fn live_design(cx: &mut Cx) {
    crate::live_design::live_design(cx);
}
```

### Configuration

```rust
let config = ShellConfig {
    title: "My App".to_string(),
    window_size: (1400.0, 900.0),

    // Visibility
    show_header: true,
    show_footer: true,
    show_left_sidebar: true,
    show_right_sidebar: true,

    // Initial sizes
    left_sidebar_width: 280.0,
    right_sidebar_width: 300.0,
    footer_height: 100.0,

    // Grid configuration
    max_rows: 3,
    max_slots_per_row: 9,

    // Features
    enable_panel_close: true,
    enable_panel_maximize: true,
    enable_panel_drag: true,
    enable_persistence: true,

    // Theme
    dark_mode: false,
};
```

---

## Example: Adopter Usage

```rust
// my_app/src/app.rs
use makepad_widgets::*;
use makepad_app_shell::prelude::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_app_shell::widgets::*;

    // Custom panel content
    MyPanelContent = <View> {
        show_bg: true
        draw_bg: { instance dark_mode: 0.0 }

        <Label> {
            text: "My Custom Content"
            draw_text: { instance dark_mode: 0.0 }
        }
    }

    // App using the shell
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { title: "My App" }

                body = <ShellLayout> {
                    // Override panel template
                    panel_template: <MyPanelContent> {}

                    // Custom header
                    header = <ShellHeader> {
                        title: "My Application"
                    }
                }
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
        makepad_app_shell::live_design(cx);  // Register shell
    }
}

impl ShellCallbacks for App {
    fn on_panel_closed(&mut self, cx: &mut Cx, panel_id: LiveId) {
        log!("Panel {:?} closed", panel_id);
    }

    fn on_dark_mode_changed(&mut self, cx: &mut Cx, dark_mode: bool) {
        // Save preference
        let mut prefs = ShellPreferences::load("my_app");
        prefs.dark_mode = dark_mode;
        prefs.save("my_app").ok();
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
```

---

## Implementation Phases

### Phase 1: Create Crate Structure
- Update `Cargo.toml` to be a library
- Create `src/lib.rs` with module declarations
- Create directory structure

### Phase 2: Extract Theme System
- Create `src/theme/colors.rs` with Tailwind-style palette
- Create `src/theme/styles.rs` with text styles
- Create `src/theme/mod.rs` with `ShellTheme` and `ThemeListener`

### Phase 3: Extract Panel Widget
- Move `SubWindow` to `src/panel/panel.rs` as `Panel`
- Move `SubWindowAction` to `src/panel/actions.rs` as `PanelAction`
- Create `src/panel/content.rs` with `PanelContent` trait

### Phase 4: Extract Grid Widget
- Move `ContentArea` to `src/grid/panel_grid.rs` as `PanelGrid`
- Extract `DropPosition` to `src/grid/drop_handler.rs`
- Create `src/grid/layout_state.rs` with serializable state

### Phase 5: Extract Shell Components
- Create `src/shell/layout.rs` with `ShellLayout`
- Create `src/shell/header.rs`, `footer.rs`, `sidebar.rs`
- Create `src/shell/config.rs` with `ShellConfig`

### Phase 6: Add Callbacks
- Create `src/callbacks.rs` with `ShellCallbacks` trait

### Phase 7: Add Persistence
- Create `src/persistence.rs` with save/load functions

### Phase 8: Create Base live_design
- Create `src/live_design.rs` with themeable base widgets
- Remove demo-specific content from live_design

### Phase 9: Move Demo to Examples
- Create `examples/flex-layout-demo/`
- Move demo sidebar data to `demo_sidebars.rs`
- Move demo panel content to `demo_panels.rs`

### Phase 10: Test and Document
- Verify build succeeds
- Verify demo runs identically
- Test dark/light mode toggle
- Test drag-drop functionality
- Test persistence

---

## Code Migration Map

| Current Location | Lines | New Location |
|------------------|-------|--------------|
| `app.rs` color constants | 34-52 | `src/theme/colors.rs` |
| `app.rs` text styles | 58-68 | `src/theme/styles.rs` |
| `app.rs` StudioButton | 74-99 | `src/live_design.rs` |
| `app.rs` SubWindow live_design | 105-281 | `src/panel/panel.rs` (live_design!) |
| `app.rs` TabButton | 287-319 | `src/live_design.rs` |
| `app.rs` ContentArea live_design | 325-397 | `src/grid/panel_grid.rs` (live_design!) |
| `app.rs` LeftSidebarHeader | 403-419 | `src/shell/sidebar.rs` |
| `app.rs` LeftSidebar live_design | 425-505 | `examples/.../demo_sidebars.rs` |
| `app.rs` RightSidebarHeader | 511-553 | `src/shell/sidebar.rs` |
| `app.rs` RightSidebar live_design | 559-639 | `examples/.../demo_sidebars.rs` |
| `app.rs` StudioHeader | 645-674 | `src/shell/header.rs` |
| `app.rs` StudioFooter | 680-709 | `src/shell/footer.rs` |
| `app.rs` Dock layout | 715-820 | `src/shell/layout.rs` |
| `app.rs` App live_design | 826-841 | `examples/.../app.rs` |
| `app.rs` SubWindowAction | 854-862 | `src/panel/actions.rs` |
| `app.rs` SubWindow impl | 878-1056 | `src/panel/panel.rs` |
| `app.rs` LayoutMode | 1062-1080 | `src/grid/layout_state.rs` |
| `app.rs` DropPosition | 1088-1096 | `src/grid/drop_handler.rs` |
| `app.rs` ContentArea impl | 1114-1586 | `src/grid/panel_grid.rs` |
| `app.rs` DemoFileNode | 1593-1609 | `examples/.../demo_sidebars.rs` |
| `app.rs` LeftSidebar impl | 1611-1744 | `examples/.../demo_sidebars.rs` |
| `app.rs` RightSidebar impl | 1750-1901 | `examples/.../demo_sidebars.rs` |
| `app.rs` StudioLayout impl | 1907-1921 | `src/shell/layout.rs` |
| `app.rs` App impl | 1927-1946 | `examples/.../app.rs` |

---

## Verification Checklist

- [ ] `cargo build` succeeds for library crate
- [ ] `cargo build --example flex-layout-demo` succeeds
- [ ] Demo app launches and displays correctly
- [ ] All 9 panels render with correct colors
- [ ] Panels can be dragged between rows
- [ ] Panels can be closed (X button)
- [ ] Panels can be maximized/restored
- [ ] Splitters resize correctly
- [ ] Dark/light mode toggles with animation
- [ ] Layout persists across app restart
- [ ] Theme preference persists across app restart

---

## Dependencies

### Library Crate
```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "rik" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"  # For config directory paths
```

### Example Crate
```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "rik" }
makepad-app-shell = { path = "../.." }
```

---

## Future Enhancements (Out of Scope)

- Tab groups within panels
- Panel docking/undocking
- Multiple windows
- Keyboard shortcuts for layout management
- Layout presets/templates
- Undo/redo for layout changes
