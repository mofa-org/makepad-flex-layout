# Flex Layout Shell - Integration Guide

This guide explains how to integrate your Makepad application with the Flex Layout Shell using the scope-based content injection pattern.

## Architecture Overview

The shell uses Makepad's `Scope` mechanism to pass data between the shell and your content widgets:

```
┌─────────────────────────────────────────────────────────┐
│                      Your App                            │
│                                                          │
│  struct AppData {                                        │
│      file_browser: FileBrowserState,                     │
│      editor: EditorState,                                │
│      console: ConsoleState,                              │
│  }                                                       │
│                                                          │
│  // Pass data through scope                              │
│  ui.handle_event(cx, event,                              │
│      &mut Scope::with_data(&mut self.data));             │
│                           │                              │
└───────────────────────────┼──────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────┐
│                    Shell (PanelGrid)                      │
│                                                           │
│  Draws panels with panel ID in scope path                 │
│  Each panel calls: scope.with_id(panel_id, |scope| ...)   │
│                                                           │
└───────────────────────────────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────┐
│                   Your Content Widget                     │
│                                                           │
│  fn draw_walk(&mut self, cx, scope, walk) {               │
│      // Get panel ID from scope path                      │
│      let panel_id = scope.path.from_end(0);               │
│                                                           │
│      // Get your app data from scope                      │
│      let data = scope.data.get::<AppData>().unwrap();     │
│                                                           │
│      // Draw based on panel ID                            │
│      match panel_id {                                     │
│          id if id == live_id!(files) => ...               │
│          id if id == live_id!(editor) => ...              │
│      }                                                    │
│  }                                                        │
└───────────────────────────────────────────────────────────┘
```

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

Override the panel content slots with your widgets:

```rust
live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_app_shell::widgets::*;

    // Import your widgets
    use crate::file_browser::MyFileBrowser;
    use crate::editor::MyEditor;
    use crate::console::MyConsole;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                body = <ShellLayout> {
                    // Override panel content
                    center_content = <PanelGrid> {
                        window_container = <View> {
                            row1 = <View> {
                                s1_1 = <Panel> {
                                    title: "Files"
                                    content = <View> {
                                        <MyFileBrowser> {}
                                    }
                                }
                                s1_2 = <Panel> {
                                    title: "Editor"
                                    content = <View> {
                                        <MyEditor> {}
                                    }
                                }
                                s1_3 = <Panel> {
                                    title: "Console"
                                    content = <View> {
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

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Pass your data through scope - panels receive it automatically
        self.ui.handle_event(cx, event, &mut Scope::with_data(&mut self.data));

        // Handle app-level actions
        self.handle_actions(cx);
    }
}
```

## How Panel ID Routing Works

Each panel pushes its semantic ID to the scope path before drawing content:

```rust
// Inside Panel::draw_walk
scope.with_id(self.panel_id, |scope| {
    self.view.draw_walk(cx, scope, walk)
})
```

Your content widgets can access this ID:

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    // Get the panel ID (LiveId)
    let panel_id = scope.path.from_end(0);

    // Compare with known IDs
    if panel_id == LiveId::from_str_lc("editor") {
        // This widget is in the editor panel
    }

    // Or use pattern matching
    match panel_id {
        id if id == live_id!(files) => self.draw_as_file_browser(cx, scope),
        id if id == live_id!(editor) => self.draw_as_editor(cx, scope),
        id if id == live_id!(console) => self.draw_as_console(cx, scope),
        _ => DrawStep::done(),
    }
}
```

## Multi-Instance Panels

For panels that can have multiple instances (like code editors with tabs), use the panel ID to route to the correct data:

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    let panel_id = scope.path.from_end(0);

    if let Some(data) = scope.data.get::<AppData>() {
        // Use panel ID to get the correct session
        if let Some(session) = data.editor_sessions.get(&panel_id) {
            self.draw_editor_content(cx, session);
        }
    }

    self.view.draw_walk(cx, scope, walk)
}
```

## Benefits of This Pattern

1. **No Trait Implementation Required** - Your app just provides data through scope
2. **Panels Are Self-Contained** - Each content widget knows what data it needs
3. **Easy to Add Panels** - Just add a new widget and place it in live_design
4. **Shell Independence** - Your widgets don't depend on shell internals
5. **Natural Makepad Pattern** - Uses the existing scope mechanism

## Comparison with Other Approaches

| Approach | Coupling | Boilerplate | Flexibility |
|----------|----------|-------------|-------------|
| Direct injection into slots | High (slot IDs) | Medium | Low |
| Trait-based provider | Medium (trait) | High | Medium |
| **Scope-based (this guide)** | **Low (just data)** | **Low** | **High** |

## Layout Persistence

The shell automatically handles layout persistence:

### Automatic Features
- **Save button** - Saves current layout to `~/Library/Application Support/<app-id>/shell_preferences.json` (macOS) or `~/.config/<app-id>/shell_preferences.json` (Linux)
- **Load on startup** - Layout is automatically restored when app launches
- **Reset button** - Returns to default 3x3 grid layout

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

## Example: Complete Integration

See `examples/flex-layout-demo` for a complete working example demonstrating:
- AppData structure
- Content widgets accessing scope
- Panel layout configuration
- Event handling through scope
- Layout persistence (Save/Load/Reset)
