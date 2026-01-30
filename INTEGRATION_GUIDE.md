# Flex Layout Shell - Integration Guide

This guide explains how to integrate your Makepad application with the Flex Layout Shell using the scope-based content injection pattern.

## Table of Contents

- [Quick Start](#quick-start)
- [Architecture Overview](#architecture-overview)
- [Shell Components](#shell-components)
- [Integration Steps](#integration-steps)
  - [Step 1: Define Your App Data](#step-1-define-your-app-data)
  - [Step 2: Create Your Content Widgets](#step-2-create-your-content-widgets)
  - [Step 3: Define Panel Layout](#step-3-define-panel-layout-in-live_design)
  - [Step 4: Wire Up Your App](#step-4-wire-up-your-app)
- [Window Configuration](#window-configuration)
- [Custom Panel Count](#custom-panel-count)
- [Slot Reference](#slot-reference)
- [Panel Configuration](#panel-configuration)
- [Sidebar Customization](#sidebar-customization)
- [Theme Support](#theme-support)
- [Layout Persistence](#layout-persistence)
- [Troubleshooting](#troubleshooting)
- [API Reference](#api-reference)

---

## Quick Start

Minimal working example with the shell layout:

```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_app_shell::shell::layout::ShellLayout;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "My App"
                    inner_size: vec2(1400, 900)
                }
                body = <ShellLayout> {}
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_app_shell::live_design(cx);
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

## Architecture Overview

The shell uses Makepad's `Scope` mechanism to pass data between the shell and your content widgets:

```
┌─────────────────────────────────────────────────────────────┐
│                      Your App                                │
│                                                              │
│  struct AppData {                                            │
│      file_browser: FileBrowserState,                         │
│      editor: EditorState,                                    │
│      console: ConsoleState,                                  │
│  }                                                           │
│                                                              │
│  // Pass data through scope                                  │
│  ui.handle_event(cx, event,                                  │
│      &mut Scope::with_data(&mut self.data));                 │
│                           │                                  │
└───────────────────────────┼──────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│                    Shell Layout (Dock-based)                  │
│                                                               │
│  ┌─────────┬────────────────────────────┬─────────┐          │
│  │ Header  │                            │         │          │
│  ├─────────┼─────────────┬──────────────┼─────────┤          │
│  │ Left    │ Center      │ Right        │ Overlay │          │
│  │ Sidebar │ (PanelGrid) │ Sidebar      │ Sidebar │          │
│  │         │             │              │         │          │
│  ├─────────┴─────────────┴──────────────┴─────────┤          │
│  │ Footer (FooterGrid)                            │          │
│  └────────────────────────────────────────────────┘          │
│                                                               │
│  Each panel calls: scope.with_id(panel_id, |scope| ...)      │
│                                                               │
└───────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│                   Your Content Widget                         │
│                                                               │
│  fn draw_walk(&mut self, cx, scope, walk) {                   │
│      // Get panel ID from scope path                          │
│      let panel_id = scope.path.from_end(0);                   │
│                                                               │
│      // Get your app data from scope                          │
│      let data = scope.data.get::<AppData>().unwrap();         │
│                                                               │
│      // Draw based on panel ID                                │
│      match panel_id {                                         │
│          id if id == live_id!(files) => ...                   │
│          id if id == live_id!(editor) => ...                  │
│      }                                                        │
│  }                                                            │
└───────────────────────────────────────────────────────────────┘
```

---

## Shell Components

The shell provides these components inside a Dock layout:

| Component | Widget | Purpose |
|-----------|--------|---------|
| Header | `ShellHeader` | Title, theme toggle, hamburger, save/reset |
| Left Sidebar | `ShellSidebar` | Navigation menu (Blueprint) |
| Center | `PanelGrid` | Main draggable panel grid (3 rows × 9 slots) |
| Right Sidebar | `ShellSidebar` | Properties/details panel |
| Footer | `FooterGrid` | Bottom panels with fullscreen support |
| Overlay Sidebar | Built-in | Hover-triggered quick actions menu |
| Pinned Sidebar | Built-in | Click-triggered sidebar with push animation |

---

## Integration Steps

### Step 1: Define Your App Data

Create a struct that holds all your application state:

```rust
use makepad_widgets::*;

// Your app data - passed to panels via scope
pub struct AppData {
    pub file_system: FileSystem,
    pub editor_sessions: HashMap<LiveId, EditorSession>,
    pub console_output: Vec<String>,
    // ... other state
}
```

### Step 2: Create Your Content Widgets

Create widgets that will be placed inside panels. They access data via scope:

```rust
#[derive(Live, LiveHook, Widget)]
pub struct MyFileBrowser {
    #[deref] view: View,
}

impl Widget for MyFileBrowser {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Get panel ID (which panel am I in?)
        let panel_id = scope.path.from_end(0);

        // Get app data
        if let Some(data) = scope.data.get_mut::<AppData>() {
            // Handle events with access to your data
            self.handle_file_events(cx, event, &mut data.file_system);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get app data and draw
        if let Some(data) = scope.data.get::<AppData>() {
            self.draw_file_tree(cx, &data.file_system);
        }

        self.view.draw_walk(cx, scope, walk)
    }
}
```

### Step 3: Define Panel Layout in live_design!

The shell uses a Dock with named content templates. Override the content inside PanelGrid:

```rust
live_design! {
    use link::theme::*;
    use link::widgets::*;

    // Shell imports (use specific paths)
    use makepad_app_shell::shell::layout::ShellLayout;
    use makepad_app_shell::grid::panel_grid::PanelGrid;
    use makepad_app_shell::panel::panel::Panel;

    // Import your widgets
    use crate::file_browser::MyFileBrowser;
    use crate::editor::MyEditor;
    use crate::console::MyConsole;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "My IDE"
                    inner_size: vec2(1400, 900)
                }

                body = <ShellLayout> {
                    // Override the center PanelGrid content
                    main_container = {
                        dock_wrapper = {
                            dock = {
                                center_content = <PanelGrid> {
                                    window_container = {
                                        row1 = {
                                            s1_1 = {
                                                title_bar = { title = { text: "Files" } }
                                                content = {
                                                    <MyFileBrowser> {}
                                                }
                                            }
                                            s1_2 = {
                                                title_bar = { title = { text: "Editor" } }
                                                content = {
                                                    <MyEditor> {}
                                                }
                                            }
                                            s1_3 = {
                                                title_bar = { title = { text: "Console" } }
                                                content = {
                                                    <MyConsole> {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### Step 4: Wire Up Your App

In your main App widget, pass data through scope:

```rust
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] data: AppData,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_app_shell::live_design(cx);

        // Register your widgets
        crate::file_browser::live_design(cx);
        crate::editor::live_design(cx);
        crate::console::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Initialize your app, configure panels, etc.
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle widget actions
        for action in actions.iter() {
            // Process your widget actions here
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Use MatchEvent for structured event handling
        self.match_event(cx, event);

        // Pass your data through scope - panels receive it automatically
        self.ui.handle_event(cx, event, &mut Scope::with_data(&mut self.data));
    }
}

app_main!(App);
```

---

## Window Configuration

The shell does NOT include a `<Window>` widget - your app must provide it.

### Setting Window Title and Size

```rust
live_design! {
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "My Application"      // OS window title bar
                    inner_size: vec2(1400, 900)  // Initial window size
                }
                body = <ShellLayout> { ... }
            }
        }
    }
}
```

### Changing Window Title Programmatically

```rust
// In your app code
self.ui.window(id!(main_window)).set_title(cx, "New Window Title");
```

### Header Title vs Window Title

| Title Type | Location | How to Set |
|------------|----------|------------|
| **OS Window Title** | Title bar | `window: { title: "..." }` |
| **Header Label** | Inside app header | `header = { title_label = { text: "..." } }` |

To set the header label:
```rust
body = <ShellLayout> {
    main_container = {
        header = {
            title_label = { text: "My App Name" }
        }
    }
}
```

---

## Custom Panel Count

By default, PanelGrid has 9 panels (3×3). To use fewer panels:

### Method 1: Programmatic Configuration (Recommended)

```rust
use makepad_app_shell::grid::panel_grid::PanelGridWidgetRefExt;
use makepad_app_shell::grid::LayoutState;

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Configure for 4 panels (2×2 grid)
        let panel_grid = self.ui.panel_grid(id!(center_content));
        panel_grid.set_layout_state(cx, LayoutState::with_panel_count(4));
    }
}
```

### Method 2: Hide Unused Slots in live_design!

```rust
center_content = <PanelGrid> {
    window_container = {
        row1 = {
            s1_1 = { /* your panel */ }
            s1_2 = { /* your panel */ }
            // Hide unused slots
            s1_3 = { visible: false, width: 0, height: 0 }
            s1_4 = { visible: false, width: 0, height: 0 }
            s1_5 = { visible: false, width: 0, height: 0 }
            s1_6 = { visible: false, width: 0, height: 0 }
            s1_7 = { visible: false, width: 0, height: 0 }
            s1_8 = { visible: false, width: 0, height: 0 }
            s1_9 = { visible: false, width: 0, height: 0 }
        }
        row2 = {
            s2_1 = { /* your panel */ }
            s2_2 = { /* your panel */ }
            // Hide unused slots in row2...
            s2_3 = { visible: false, width: 0, height: 0 }
            // ... etc
        }
        // Hide entire row3 if not needed
        row3 = { visible: false, height: 0 }
    }
}
```

### Method 3: Combine Both

For best results, use both methods together - declarative for content, programmatic for layout state:

```rust
// live_design! - define content and hide unused slots
// handle_startup - set LayoutState::with_panel_count(N)
```

---

## Slot Reference

### PanelGrid Slots

PanelGrid has 3 rows × 9 slots = 27 total slots.

**Naming convention:** `s{row}_{slot}` (1-indexed)

| Row | Slots |
|-----|-------|
| Row 1 | `s1_1`, `s1_2`, `s1_3`, `s1_4`, `s1_5`, `s1_6`, `s1_7`, `s1_8`, `s1_9` |
| Row 2 | `s2_1`, `s2_2`, `s2_3`, `s2_4`, `s2_5`, `s2_6`, `s2_7`, `s2_8`, `s2_9` |
| Row 3 | `s3_1`, `s3_2`, `s3_3`, `s3_4`, `s3_5`, `s3_6`, `s3_7`, `s3_8`, `s3_9` |

**Row containers:** `row1`, `row2`, `row3`

### FooterGrid Slots

FooterGrid has 7 horizontal slots, each can stack up to 5 panels.

**Naming convention:** `f1_{slot}` for slots, `p{index}` for panels within slot

| Slot | ID | Panels |
|------|----|--------|
| Slot 0 | `f1_0` | `p0`, `p1`, `p2`, `p3`, `p4` |
| Slot 1 | `f1_1` | `p0`, `p1`, `p2`, `p3`, `p4` |
| Slot 2 | `f1_2` | `p0`, `p1`, `p2`, `p3`, `p4` |
| Slot 3 | `f1_3` | `p0`, `p1`, `p2`, `p3`, `p4` |
| Slot 4 | `f1_4` | `p0`, `p1`, `p2`, `p3`, `p4` |
| Slot 5 | `f1_5` | `p0`, `p1`, `p2`, `p3`, `p4` |
| Slot 6 | `f1_6` | `p0`, `p1`, `p2`, `p3`, `p4` |

**Special areas:**
- `controller_content` - Left sidebar area in footer
- `panel_strip_content` - Container for all slots

### Example: Footer with 3 Panels

```rust
footer_content = <FooterGrid> {
    dock = {
        controller_content = <View> {
            // Left sidebar content (e.g., playback controls)
        }

        panel_strip_content = {
            flow: Down  // Stack vertically

            f1_0 = {
                p0 = {
                    title_bar = { title = { text: "State Plot" } }
                    content = { <MyPlotWidget> {} }
                }
            }
            f1_1 = {
                p0 = {
                    title_bar = { title = { text: "Action Plot" } }
                    content = { <MyPlotWidget> {} }
                }
            }
            f1_2 = {
                p0 = {
                    title_bar = { title = { text: "Timeline" } }
                    content = { <MyTimeline> {} }
                }
            }
            // Hide unused slots
            f1_3 = { visible: false, width: 0 }
            f1_4 = { visible: false, width: 0 }
            f1_5 = { visible: false, width: 0 }
            f1_6 = { visible: false, width: 0 }
        }
    }
}
```

---

## Panel Configuration

### Setting Panel Titles

#### Declarative (in live_design!)

```rust
center_content = <PanelGrid> {
    window_container = {
        row1 = {
            s1_1 = {
                title_bar = { title = { text: "My Panel Title" } }
                content = { ... }
            }
        }
    }
}
```

#### Programmatic (Panel.set_title)

```rust
use makepad_app_shell::panel::panel::PanelWidgetExt;

// Set title on a specific panel
self.ui.panel(id!(s1_1)).set_title(cx, "New Title");
```

#### Persistent Titles (PanelGrid.set_panel_titles)

When panels are dragged and dropped, titles set via `Panel.set_title` may revert to default "Panel X" names because the layout state gets reconfigured. To set titles that persist across drag/drop operations, use `PanelGridRef.set_panel_titles()`:

```rust
use makepad_app_shell::grid::panel_grid::PanelGridWidgetRefExt;

impl MatchEvent for MyApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let panel_grid = self.ui.panel_grid(id!(center_content));

        // Set layout state first
        panel_grid.set_layout_state(cx, LayoutState::with_panel_count(4));

        // Set persistent panel titles - these survive drag/drop operations
        panel_grid.set_panel_titles(&[
            ("panel_0", "Camera High"),
            ("panel_1", "3D View"),
            ("panel_2", "Left Wrist"),
            ("panel_3", "Right Wrist"),
        ]);
    }
}
```

**How it works:**
- Titles are stored in `PanelGrid.panel_titles` HashMap, separate from `LayoutState`
- When layout is reconfigured (e.g., after drag/drop), titles are reapplied from this HashMap
- Uses thread-local storage to defer title setting if called before widget initialization

**Individual title setting:**

```rust
// Set a single panel title
panel_grid.set_panel_title("panel_0", "My Custom Title");
```

**Panel ID mapping:**
| Panel ID | Default Slot |
|----------|--------------|
| `panel_0` | Row 1, Column 1 (s1_1) |
| `panel_1` | Row 1, Column 2 (s1_2) |
| `panel_2` | Row 1, Column 3 (s1_3) |
| `panel_3` | Row 2, Column 1 (s2_1) |
| ... | ... |

### Setting Footer Panel Titles

FooterGrid panels require special handling because their initialization may overwrite titles set in `handle_startup`.

#### Using FooterGrid.set_panel_title

```rust
use makepad_app_shell::grid::footer_grid::FooterGridWidgetRefExt;

impl MatchEvent for MyApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let footer_grid = self.ui.footer_grid(id!(footer_content));

        // Configure layout first
        footer_grid.set_layout_state(cx, footer_state);

        // Then set titles - these are deferred if widget isn't ready
        footer_grid.set_panel_title(cx, 0, 0, "State Plot");   // slot 0, panel 0
        footer_grid.set_panel_title(cx, 1, 0, "Action Plot");  // slot 1, panel 0
        footer_grid.set_panel_title(cx, 2, 0, "Timeline");     // slot 2, panel 0
    }
}
```

**Parameters:**
- `slot_index`: 0-6 for f1_0 through f1_6
- `panel_index`: 0-4 for p0 through p4 within the slot (usually 0)
- `title`: The panel title string

---

## Panel Close Behavior

When a user closes a panel (clicks the X button), the panel is hidden **in place** rather than compacted. This preserves the mapping between slot positions and content widgets.

### Why This Matters

If your app places different content widgets in each slot:

```rust
center_content = <PanelGrid> {
    window_container = {
        row1 = {
            s1_1 = { content = { <VideoPlayer> {} } }   // panel_0
            s1_2 = { content = { <RobotViewer> {} } }   // panel_1
        }
        row2 = {
            s2_1 = { content = { <VideoPlayer> {} } }   // panel_2
            s2_2 = { content = { <VideoPlayer> {} } }   // panel_3
        }
    }
}
```

Closing panel_0 will:
- Hide slot s1_1 (preserving its VideoPlayer content)
- Keep panel_1 in s1_2 (RobotViewer stays in place)

Without this behavior, closing panel_0 would shift panel_1 into s1_1, but the content (RobotViewer) would still be in s1_2, causing a mismatch.

### Optimizing Hidden Panels

Hidden panels continue to exist in the widget tree. To avoid wasting resources on hidden content (e.g., video decoding), check visibility before updating:

```rust
fn update_content(&mut self, cx: &mut Cx) {
    let panel_grid = self.ui.panel_grid(id!(center_content));
    let layout_state = panel_grid.layout_state();

    // Helper to check if a panel is visible
    let is_visible = |panel_id: &str| -> bool {
        layout_state.as_ref()
            .map(|s| s.visible_panels.contains(panel_id))
            .unwrap_or(true) // Default to visible if state unavailable
    };

    // Only update visible video panels
    if is_visible("panel_0") {
        let video = self.ui.video_player(id!(video_main));
        video.show_frame_at_time(cx, self.current_time);
    }

    if is_visible("panel_2") {
        let video = self.ui.video_player(id!(video_cam1));
        video.show_frame_at_time(cx, self.current_time);
    }
}
```

### Re-opening Closed Panels

Currently, closed panels remain hidden until the layout is reset. To re-open a panel programmatically:

```rust
// Get current layout state
let panel_grid = self.ui.panel_grid(id!(center_content));
if let Some(mut state) = panel_grid.layout_state() {
    // Add panel back to visible set
    state.visible_panels.insert("panel_0".to_string());
    panel_grid.set_layout_state(cx, state);
}
```

---

## Sidebar Customization

### Overriding Left Sidebar Content

```rust
body = <ShellLayout> {
    main_container = {
        dock_wrapper = {
            dock = {
                left_sidebar_content = <View> {
                    width: Fill, height: Fill
                    flow: Down

                    show_bg: true
                    draw_bg: { color: #151518 }

                    // Header
                    <View> {
                        width: Fill, height: 40
                        padding: { left: 16 }
                        align: { y: 0.5 }
                        <Label> { text: "My Sidebar" }
                    }

                    // Your content
                    <MySidebarContent> {}
                }
            }
        }
    }
}
```

### Overriding Right Sidebar Content

```rust
right_sidebar_content = <View> {
    width: Fill, height: Fill
    flow: Down

    <MyPropertiesPanel> {}
}
```

### Setting Sidebar Titles Programmatically

```rust
use makepad_app_shell::shell::sidebar::ShellSidebarWidgetExt;

// Set left sidebar title
self.ui.shell_sidebar(id!(left_sidebar_content)).set_title(cx, "Files");

// Set right sidebar title
self.ui.shell_sidebar(id!(right_sidebar_content)).set_title(cx, "Properties");
```

---

## Theme Support

The shell provides comprehensive dark/light theme support.

### Global Theme State

Widgets inside the Dock can't be directly accessed via ID lookup. The shell uses thread-local storage for global theme state:

```rust
use makepad_app_shell::theme::get_global_dark_mode;

impl Widget for MyWidget {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Read global theme on every draw
        let dm = get_global_dark_mode();

        // Apply to widget using apply_over
        self.view.apply_over(cx, live!{
            draw_bg: { dark_mode: (dm) }
        });

        self.view.draw_walk(cx, scope, walk)
    }
}
```

### Shader-Based Dark Mode

Use `instance dark_mode` with `mix()` for smooth transitions:

```rust
live_design! {
    MyWidget = <View> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0);         // white
                let dark_bg = vec4(0.059, 0.090, 0.165, 1.0);    // slate-900
                return mix(light_bg, dark_bg, self.dark_mode);
            }
        }
    }
}
```

### Complete Theme Integration Example

```rust
use makepad_app_shell::theme::get_global_dark_mode;

#[derive(Live, LiveHook, Widget)]
pub struct ThemedWidget {
    #[deref] view: View,
}

impl Widget for ThemedWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get current theme value (0.0 = light, 1.0 = dark)
        let dm = get_global_dark_mode();

        // Apply to all themed elements
        self.view.apply_over(cx, live!{
            draw_bg: { dark_mode: (dm) }
        });
        self.view.label(id!(my_label)).apply_over(cx, live!{
            draw_text: { dark_mode: (dm) }
        });

        self.view.draw_walk(cx, scope, walk)
    }
}
```

---

## Layout Persistence

The shell automatically handles layout persistence.

### Automatic Features

- **Save button** - Saves current layout to config directory
- **Load on startup** - Layout is automatically restored
- **Reset button** - Returns to default layout

### Storage Locations

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/<app-id>/shell_preferences.json` |
| Linux | `~/.config/<app-id>/shell_preferences.json` |
| Windows | `%APPDATA%/<app-id>/shell_preferences.json` |

### What Gets Persisted

```json
{
  "dark_mode": false,
  "layout": {
    "row_assignments": [["panel_0", "panel_1"], ["panel_2"]],
    "visible_panels": ["panel_0", "panel_1", "panel_2"],
    "maximized_panel": null
  },
  "footer_layout": {
    "slots": [
      {"visible": true, "panel_ids": ["footer_panel_0", "footer_panel_1"]}
    ]
  }
}
```

### Panel IDs

Panels use semantic string IDs (`panel_0`, `panel_1`, `footer_panel_0`, etc.) which are:
- Converted to `LiveId` via `LiveId::from_str_lc()`
- Pushed to scope path for content widget identification
- Used as keys in persistence

---

## Dynamic Layout Reset

When your application needs to change the panel layout at runtime (e.g., loading a new dataset with a different number of cameras), use `set_layout_state()` to reset the layout.

### How Layout State Works

The `PanelGrid` uses a thread-local fallback mechanism for setting layout state. This is necessary because `WidgetRef.borrow_mut()` can fail during certain event handling phases in Makepad.

```
set_layout_state() called
         │
         ▼
    Can borrow widget?
         │
    ┌────┴────┐
    │ Yes     │ No
    ▼         ▼
Apply       Store in PENDING_LAYOUT
immediately (thread-local)
              │
              ▼
         Applied on next draw_walk()
```

### Resetting Layout When Loading New Data

```rust
impl MyApp {
    fn load_new_dataset(&mut self, cx: &mut Cx, path: &str) {
        // 1. Clear existing content
        self.clear_current_content(cx);

        // 2. Load new data and determine panel count
        let dataset = load_dataset(path);
        let camera_count = dataset.cameras.len();

        // 3. Reset layout to match new data
        let panel_grid = self.ui.panel_grid(id!(center_content));
        panel_grid.set_layout_state(cx, LayoutState::with_panel_count(camera_count));

        // 4. Set panel titles for the new data
        let titles: Vec<(&str, &str)> = dataset.cameras
            .iter()
            .enumerate()
            .map(|(i, cam)| (format!("panel_{}", i).leak(), cam.name.as_str()))
            .collect();
        panel_grid.set_panel_titles(&titles);

        // 5. Initialize content in panels
        self.init_panel_content(cx, &dataset);
    }
}
```

### Important: Layout Resets Are Deferred

Because `set_layout_state()` may use the thread-local fallback, the layout change is applied on the next draw cycle, not immediately. This means:

1. **Don't assume immediate effect** - The layout state may not be applied until the next frame
2. **Widget borrows may fail** - If you try to access panels immediately after reset, they may not be configured yet
3. **Use redraw to trigger application** - Call `cx.redraw_all()` or `self.view.redraw(cx)` to ensure the pending layout is applied

### Thread-Local Storage Details

The `PENDING_LAYOUT` thread-local stores layout state when direct widget access fails:

```rust
// In panel_grid.rs
thread_local! {
    static PENDING_LAYOUT: RefCell<Option<LayoutState>> = RefCell::new(None);
}

impl PanelGridRef {
    pub fn set_layout_state(&self, cx: &mut Cx, state: LayoutState) {
        if let Some(mut inner) = self.borrow_mut() {
            // Direct application
            inner.layout_state = state;
            inner.needs_layout_update = true;
            inner.view.redraw(cx);
        } else {
            // Fallback: store for next draw
            PENDING_LAYOUT.with(|p| *p.borrow_mut() = Some(state));
        }
    }
}
```

The pending layout is checked on **every draw cycle** (not just initialization), so layout resets work even after the widget has been fully initialized.

### Common Pattern: Clear and Reset

```rust
fn clear_and_reset(&mut self, cx: &mut Cx) {
    // Clear all video players / content
    self.ui.video_player(id!(video_main)).clear(cx);
    self.ui.video_player(id!(video_cam1)).clear(cx);

    // Reset to new layout (4 panels in 2x2 grid)
    let panel_grid = self.ui.panel_grid(id!(center_content));
    panel_grid.set_layout_state(cx, LayoutState::with_panel_count(4));

    // Trigger redraw to apply pending layout
    self.ui.redraw(cx);
}
```

---

## Troubleshooting

### Common Issues

#### 1. "Widget not found" errors

**Problem:** Can't access widgets inside the Dock by ID.

**Solution:** Widgets inside Dock are not directly accessible. Use the provided extension traits:
```rust
use makepad_app_shell::grid::panel_grid::PanelGridWidgetRefExt;
self.ui.panel_grid(id!(center_content))  // Works
self.ui.view(id!(center_content))        // May not work
```

#### 2. Panel titles not updating

**Problem:** Titles set in `handle_startup` are overwritten.

**Solution:** Use deferred title setting via FooterGrid:
```rust
footer_grid.set_panel_title(cx, slot, panel, "Title");
```

#### 3. Theme toggle doesn't affect my widgets

**Problem:** Custom widgets don't respond to dark/light mode switch.

**Solution:** Widgets must read and apply theme on every draw:
```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    let dm = get_global_dark_mode();
    self.view.apply_over(cx, live!{ draw_bg: { dark_mode: (dm) } });
    self.view.draw_walk(cx, scope, walk)
}
```

#### 4. Too many panels showing

**Problem:** App shows all 9 default panels.

**Solution:** Either hide unused slots declaratively or use `LayoutState::with_panel_count()`:
```rust
panel_grid.set_layout_state(cx, LayoutState::with_panel_count(4));
```

#### 5. Scope data is None

**Problem:** `scope.data.get::<AppData>()` returns None.

**Solution:** Ensure you're passing data in `handle_event`:
```rust
self.ui.handle_event(cx, event, &mut Scope::with_data(&mut self.data));
```

#### 6. Layout not resetting when loading new data

**Problem:** After closing panels and loading a new dataset, the layout doesn't reset - closed panels stay closed.

**Cause:** `set_layout_state()` couldn't borrow the widget directly and stored the layout in `PENDING_LAYOUT`, but the pending layout wasn't being applied.

**Solution:** This was fixed in the panel_grid.rs to check `PENDING_LAYOUT` on every draw cycle. If you're on an older version:
1. Update makepad-app-shell to the latest version
2. Ensure `PENDING_LAYOUT` is checked outside the `if !self.initialized` block in `draw_walk()`

**Verification:** Add logging to confirm layout is being applied:
```rust
fn load_dataset(&mut self, cx: &mut Cx, path: &str) {
    // Reset layout
    let panel_grid = self.ui.panel_grid(id!(center_content));
    panel_grid.set_layout_state(cx, LayoutState::with_panel_count(4));
    log::info!("Layout reset requested for 4 panels");

    // Force redraw
    self.ui.redraw(cx);
}
```

#### 7. Closing panel causes wrong content in other panels

**Problem:** When closing a panel, another panel suddenly shows the wrong content (e.g., closing a camera panel makes the 3D view show video).

**Cause:** Previously, closing a panel would compact the remaining panels - panel IDs were reassigned to slots starting from index 0. But content widgets are statically placed in slots, so the panel title/ID would change while the content stayed the same.

**Solution:** Fixed in panel_grid.rs and layout_state.rs:
- `close_panel()` now keeps the panel in `row_assignments` (only removes from `visible_panels`)
- `apply_row_layout()` now iterates by position, hiding closed panels in place rather than compacting

**Result:** Closing a panel hides its slot in place, preserving the mapping between slots and their content widgets.

#### 8. Hidden panels still consuming resources

**Problem:** After closing a video panel, video decoding continues in the background.

**Solution:** Check panel visibility before updating content:
```rust
fn update_videos(&mut self, cx: &mut Cx) {
    let panel_grid = self.ui.panel_grid(id!(center_content));
    let layout_state = panel_grid.layout_state();

    let is_visible = |panel_id: &str| -> bool {
        layout_state.as_ref()
            .map(|s| s.visible_panels.contains(panel_id))
            .unwrap_or(true)
    };

    // Only update visible panels
    if is_visible("panel_0") {
        self.ui.video_player(id!(video_main)).show_frame_at_time(cx, time);
    }
}
```

#### 9. Import errors

**Problem:** Can't find shell components.

**Solution:** Use the correct import paths:
```rust
// Correct paths
use makepad_app_shell::shell::layout::ShellLayout;
use makepad_app_shell::grid::panel_grid::PanelGrid;
use makepad_app_shell::grid::footer_grid::FooterGrid;
use makepad_app_shell::panel::panel::Panel;
use makepad_app_shell::shell::sidebar::ShellSidebar;

// In live_design!, also use crate paths
use makepad_app_shell::shell::layout::ShellLayout;
```

---

## API Reference

### Import Paths

```rust
// Main shell layout
use makepad_app_shell::shell::layout::ShellLayout;

// Grid components
use makepad_app_shell::grid::panel_grid::{PanelGrid, PanelGridWidgetRefExt};
use makepad_app_shell::grid::footer_grid::{FooterGrid, FooterGridWidgetRefExt};
use makepad_app_shell::grid::{LayoutState, FooterLayoutState, FooterSlotState};

// Panel
use makepad_app_shell::panel::panel::{Panel, PanelWidgetExt};

// Sidebar
use makepad_app_shell::shell::sidebar::{ShellSidebar, ShellSidebarWidgetExt};

// Header
use makepad_app_shell::shell::header::ShellHeader;

// Theme
use makepad_app_shell::theme::get_global_dark_mode;

// Config
use makepad_app_shell::shell::config::{ShellConfig, ShellConfigBuilder};

// Registry
use makepad_app_shell::registry::{PanelDefinition, PanelRegistry};
```

### Key Methods

| Component | Method | Description |
|-----------|--------|-------------|
| `PanelGridRef` | `set_layout_state(cx, state)` | Set panel layout |
| `PanelGridRef` | `reset_layout(cx)` | Reset to default |
| `FooterGridRef` | `set_layout_state(cx, state)` | Set footer layout |
| `FooterGridRef` | `set_panel_title(cx, slot, panel, title)` | Set panel title |
| `PanelRef` | `set_title(cx, title)` | Set panel title |
| `ShellSidebarRef` | `set_title(cx, title)` | Set sidebar title |
| `theme` | `get_global_dark_mode()` | Get current theme (0.0-1.0) |

### LayoutState

```rust
// Create with specific panel count
let state = LayoutState::with_panel_count(4);

// Access properties
state.visible_count()           // Number of visible panels
state.is_visible("panel_0")     // Check if panel visible
state.find_panel_row("panel_0") // Get (row, col) position
```

### FooterLayoutState

```rust
let footer_state = FooterLayoutState {
    slots: vec![
        FooterSlotState { visible: true, panel_ids: vec!["footer_panel_0".into()] },
        FooterSlotState { visible: true, panel_ids: vec!["footer_panel_1".into()] },
    ],
    fullscreen_panel: None,
};
```

---

## Benefits of This Pattern

1. **No Trait Implementation Required** - Your app just provides data through scope
2. **Panels Are Self-Contained** - Each content widget knows what data it needs
3. **Easy to Add Panels** - Just add a new widget and place it in live_design
4. **Shell Independence** - Your widgets don't depend on shell internals
5. **Natural Makepad Pattern** - Uses the existing scope mechanism
6. **Built-in Theme Support** - Global dark mode works across all widgets

## Comparison with Other Approaches

| Approach | Coupling | Boilerplate | Flexibility |
|----------|----------|-------------|-------------|
| Direct injection into slots | High (slot IDs) | Medium | Low |
| Trait-based provider | Medium (trait) | High | Medium |
| **Scope-based (this guide)** | **Low (just data)** | **Low** | **High** |

---

## Example: Complete Integration

See `examples/flex-layout-demo` for a minimal working example demonstrating:
- Shell layout with all components
- Theme switching
- Layout persistence (Save/Load/Reset)

For a full integration with custom content widgets, create your own app following Steps 1-4 above.
