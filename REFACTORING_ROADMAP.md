# Flex Layout Library - Refactoring Roadmap

A prioritized plan to transform the flex layout shell into an easily adoptable framework for existing Makepad applications.

## Design Goal

Enable this adoption pattern:

```rust
// Existing app can wrap their views in flex panels with minimal code:
live_design! {
    App = {{App}} {
        <FlexShell> {
            <FlexPanel id="file_browser" title="Files"> {
                <MyExistingFileBrowser> {}  // User's existing widget
            }
            <FlexPanel id="editor" title="Editor"> {
                <MyExistingCodeEditor> {}   // User's existing widget
            }
            <FlexPanel id="console" title="Console"> {
                <MyExistingConsole> {}      // User's existing widget
            }
        }
    }
}
```

---

## Priority Levels

| Priority | Meaning | Timeframe |
|----------|---------|-----------|
| **P0** | Critical - Blocks all adoption | Immediate |
| **P1** | High - Required for real-world use | Short-term |
| **P2** | Medium - Improves developer experience | Medium-term |
| **P3** | Low - Future enhancements | Long-term |

---

## P0: Critical - Content Injection System

**Goal**: Allow users to place their own widgets inside panels.

### P0.1: Panel Content Slot

**File**: `src/panel/panel.rs`

**Current**:
```rust
content = <View> {
    content_label = <Label> { text: "#1" }  // Hardcoded
}
```

**Change to**:
```rust
content = <View> {
    width: Fill, height: Fill
    // Empty - content injected at runtime or via live_design
}
```

**Implementation**:
```rust
#[derive(Live, LiveHook, Widget)]
pub struct Panel {
    #[deref]
    view: View,

    // New: reference to user-provided content widget
    #[rust]
    content_widget: Option<WidgetRef>,
}

impl Panel {
    /// Set custom content widget
    pub fn set_content(&mut self, widget: WidgetRef) {
        self.content_widget = Some(widget);
    }

    /// Get the content area view for adding children
    pub fn content_view(&self) -> ViewRef {
        self.view.view(id!(content))
    }
}
```

**Effort**: Small (1-2 hours)

---

### P0.2: Semantic Panel IDs

**File**: `src/grid/layout_state.rs`

**Current**:
```rust
pub row_assignments: Vec<Vec<u64>>,  // [0, 1, 2]
```

**Change to**:
```rust
pub row_assignments: Vec<Vec<String>>,  // ["file_browser", "editor", "console"]
```

**Implementation**:
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutState {
    /// Panel IDs per row (semantic string IDs)
    pub row_assignments: Vec<Vec<String>>,

    /// Which panels are visible
    pub visible_panels: HashSet<String>,

    /// Currently maximized panel
    pub maximized_panel: Option<String>,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            row_assignments: vec![
                vec!["panel_0".into(), "panel_1".into(), "panel_2".into()],
                vec!["panel_3".into(), "panel_4".into(), "panel_5".into()],
            ],
            visible_panels: (0..6).map(|i| format!("panel_{}", i)).collect(),
            maximized_panel: None,
        }
    }
}
```

**Effort**: Medium (2-4 hours) - requires updating PanelGrid, FooterGrid, persistence

---

### P0.3: Panel Registry

**New File**: `src/registry.rs`

**Purpose**: Central place to register panel definitions with content providers.

```rust
use std::collections::HashMap;
use makepad_widgets::*;

/// Defines a panel type that can be instantiated in the grid
pub struct PanelDefinition {
    pub id: String,
    pub title: String,
    pub closable: bool,
    pub maximizable: bool,
}

/// Registry for panel definitions
#[derive(Default)]
pub struct PanelRegistry {
    definitions: HashMap<String, PanelDefinition>,
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a panel definition
    pub fn register(&mut self, id: impl Into<String>, def: PanelDefinition) {
        self.definitions.insert(id.into(), def);
    }

    /// Get panel definition by ID
    pub fn get(&self, id: &str) -> Option<&PanelDefinition> {
        self.definitions.get(id)
    }

    /// Get all registered panel IDs
    pub fn panel_ids(&self) -> impl Iterator<Item = &String> {
        self.definitions.keys()
    }
}
```

**Effort**: Small (1-2 hours)

---

## P1: High Priority - Flexible Layout

### P1.1: Optional Shell Components

**File**: `src/shell/layout.rs`

**Goal**: Make header, sidebars, and footer optional.

**Implementation**:
```rust
#[derive(Live, LiveHook, Widget)]
pub struct ShellLayout {
    #[deref]
    view: View,

    #[live(true)]
    show_header: bool,

    #[live(true)]
    show_left_sidebar: bool,

    #[live(true)]
    show_right_sidebar: bool,

    #[live(true)]
    show_footer: bool,
}

impl ShellLayout {
    fn apply_visibility(&mut self, cx: &mut Cx) {
        // Hide/show components based on flags
        self.view.view(id!(header)).set_visible(cx, self.show_header);
        self.view.view(id!(left_sidebar)).set_visible(cx, self.show_left_sidebar);
        // etc.
    }
}
```

**Live Design Override**:
```rust
live_design! {
    MyApp = {{MyApp}} {
        <ShellLayout> {
            show_header: false
            show_left_sidebar: false
            show_right_sidebar: false
            // Only panel grid + footer
        }
    }
}
```

**Effort**: Medium (2-3 hours)

---

### P1.2: Configurable Grid Size

**File**: `src/grid/panel_grid.rs`

**Current**: Hardcoded 3 rows × 9 slots

**Goal**: Dynamic row/column configuration

**Implementation**:
```rust
#[derive(Live, LiveHook, Widget)]
pub struct PanelGrid {
    #[deref]
    view: View,

    #[live(3)]
    max_rows: usize,

    #[live(6)]
    max_cols: usize,

    #[rust]
    panel_slots: Vec<Vec<WidgetRef>>,  // Dynamic instead of hardcoded
}

impl PanelGrid {
    fn initialize_slots(&mut self, cx: &mut Cx) {
        // Generate slots dynamically based on max_rows/max_cols
        self.panel_slots = (0..self.max_rows)
            .map(|row| {
                (0..self.max_cols)
                    .map(|col| self.create_panel_slot(cx, row, col))
                    .collect()
            })
            .collect();
    }
}
```

**Effort**: Large (4-6 hours) - significant refactoring of slot management

---

### P1.3: Live Design Content Injection

**Goal**: Allow users to define panel content directly in live_design.

**New Pattern**:
```rust
live_design! {
    use makepad_flex_shell::widgets::*;

    App = {{App}} {
        <ShellLayout> {
            // Override panel grid content
            center_content = <PanelGrid> {
                // Define panels inline with custom content
                panels: {
                    file_browser: <Panel> {
                        title: "Files"
                        content = <View> {
                            <MyFileBrowserWidget> {}
                        }
                    }
                    editor: <Panel> {
                        title: "Editor"
                        content = <View> {
                            <MyEditorWidget> {}
                        }
                    }
                }
            }
        }
    }
}
```

**Implementation**: Requires Panel to support nested content in live_design.

**Effort**: Medium (3-4 hours)

---

## P2: Medium Priority - Developer Experience

### P2.1: Panel Content Provider Trait

**New File**: `src/content_provider.rs`

**Purpose**: Programmatic content injection for dynamic panels.

```rust
/// Trait for providing panel content dynamically
pub trait PanelContentProvider: 'static {
    /// Called when panel needs to create its content widget
    fn create_content(&self, cx: &mut Cx, panel_id: &str) -> WidgetRef;

    /// Called during draw to render content
    fn draw_content(
        &mut self,
        cx: &mut Cx2d,
        panel_id: &str,
        scope: &mut Scope
    ) -> DrawStep;

    /// Handle events for this panel's content
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        panel_id: &str,
        event: &Event,
        scope: &mut Scope
    );
}

/// Default implementation that does nothing
pub struct EmptyContentProvider;

impl PanelContentProvider for EmptyContentProvider {
    fn create_content(&self, _cx: &mut Cx, _panel_id: &str) -> WidgetRef {
        WidgetRef::empty()
    }

    fn draw_content(&mut self, _cx: &mut Cx2d, _panel_id: &str, _scope: &mut Scope) -> DrawStep {
        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _panel_id: &str, _event: &Event, _scope: &mut Scope) {}
}
```

**Usage**:
```rust
struct MyApp {
    shell: ShellLayoutRef,
    file_browser: FileBrowser,
    editor: CodeEditor,
}

impl PanelContentProvider for MyApp {
    fn create_content(&self, cx: &mut Cx, panel_id: &str) -> WidgetRef {
        match panel_id {
            "file_browser" => self.file_browser.widget_ref(),
            "editor" => self.editor.widget_ref(),
            _ => WidgetRef::empty(),
        }
    }

    fn draw_content(&mut self, cx: &mut Cx2d, panel_id: &str, scope: &mut Scope) -> DrawStep {
        match panel_id {
            "file_browser" => self.file_browser.draw(cx, scope),
            "editor" => self.editor.draw(cx, scope),
            _ => DrawStep::done(),
        }
    }
}
```

**Effort**: Medium (3-4 hours)

---

### P2.2: Enhanced Callbacks

**File**: `src/callbacks.rs`

**Add render callback**:
```rust
pub trait ShellCallbacks {
    // Existing...
    fn on_panel_closed(&mut self, cx: &mut Cx, panel_id: &str) {}
    fn on_panel_maximized(&mut self, cx: &mut Cx, panel_id: &str, maximized: bool) {}

    // New: content rendering hooks
    fn on_panel_draw(
        &mut self,
        cx: &mut Cx2d,
        panel_id: &str,
        content_rect: Rect
    ) -> Option<DrawStep> {
        None  // Return None to use default rendering
    }

    fn on_panel_event(
        &mut self,
        cx: &mut Cx,
        panel_id: &str,
        event: &Event
    ) -> bool {
        false  // Return true if handled
    }
}
```

**Effort**: Small (1-2 hours)

---

### P2.3: FlexPanel Convenience Widget

**New File**: `src/flex_panel.rs`

**Purpose**: A higher-level widget that wraps Panel with easier content injection.

```rust
live_design! {
    FlexPanel = {{FlexPanel}} {
        width: Fill
        height: Fill

        panel = <Panel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct FlexPanel {
    #[deref]
    view: View,

    #[live]
    id: String,

    #[live]
    title: String,

    #[live(true)]
    closable: bool,

    #[live(true)]
    maximizable: bool,
}

impl FlexPanel {
    /// Get the content area for adding child widgets
    pub fn content(&self) -> ViewRef {
        self.view.panel(id!(panel)).content_view()
    }
}
```

**Usage**:
```rust
live_design! {
    <FlexPanel id="my_panel" title="My Panel"> {
        // Child widgets go directly here
        <Button> { text: "Click me" }
        <Label> { text: "Hello" }
    }
}
```

**Effort**: Medium (2-3 hours)

---

## P3: Low Priority - Future Enhancements

### P3.1: Unified Grid System

Merge `PanelGrid` and `FooterGrid` into a single configurable `FlexGrid`.

**Effort**: Large (6-8 hours)

---

### P3.2: Tab Support in Panels

Allow multiple views in a single panel with tabs.

```rust
<FlexPanel id="tools"> {
    tabs: ["Search", "Replace", "Find in Files"]
    <SearchPanel> {}
    <ReplacePanel> {}
    <FindInFilesPanel> {}
}
```

**Effort**: Large (8-10 hours)

---

### P3.3: Drag Panels Between Grids

Allow dragging panels from main grid to footer and vice versa.

**Effort**: Medium (4-6 hours)

---

### P3.4: Panel State Persistence

Save/restore per-panel state (scroll position, selection, etc.).

```rust
pub trait PanelState: Serialize + Deserialize {
    fn save_state(&self) -> serde_json::Value;
    fn restore_state(&mut self, state: serde_json::Value);
}
```

**Effort**: Medium (3-4 hours)

---

### P3.5: Floating Panels

Detach panels from grid into floating windows.

**Effort**: Large (8-10 hours)

---

## Implementation Order

### Phase 1: Minimum Viable Adoption (P0)
```
Week 1:
├── P0.1: Panel Content Slot
├── P0.2: Semantic Panel IDs
└── P0.3: Panel Registry
```

### Phase 2: Flexible Configuration (P1)
```
Week 2:
├── P1.1: Optional Shell Components
├── P1.2: Configurable Grid Size
└── P1.3: Live Design Content Injection
```

### Phase 3: Developer Experience (P2)
```
Week 3:
├── P2.1: Panel Content Provider Trait
├── P2.2: Enhanced Callbacks
└── P2.3: FlexPanel Convenience Widget
```

### Phase 4: Advanced Features (P3)
```
Future:
├── P3.1: Unified Grid System
├── P3.2: Tab Support
├── P3.3: Cross-Grid Drag
├── P3.4: Panel State Persistence
└── P3.5: Floating Panels
```

---

## Effort Summary

| Priority | Items | Total Effort |
|----------|-------|--------------|
| P0 | 3 | 5-8 hours |
| P1 | 3 | 9-13 hours |
| P2 | 3 | 6-9 hours |
| P3 | 5 | 29-42 hours |

**MVP (P0 + P1)**: ~14-21 hours of development
**Full DX (+ P2)**: ~20-30 hours of development

---

## Success Criteria

After P0 + P1 implementation, users should be able to:

1. **Drop in their widgets** without modifying library code
2. **Configure layout** (rows, cols, optional components) via live_design
3. **Identify panels semantically** ("editor" not "panel_3")
4. **Persist layouts** with meaningful panel associations
5. **Integrate in < 50 lines** of code for basic use case

Example final integration:
```rust
use makepad_flex_shell::prelude::*;

live_design! {
    App = {{App}} {
        <ShellLayout> {
            show_left_sidebar: false
            show_right_sidebar: false

            center_content = <PanelGrid> {
                max_rows: 2
                max_cols: 3

                <FlexPanel id="files" title="Files"> {
                    <MyFileBrowser> {}
                }
                <FlexPanel id="editor" title="Code"> {
                    <MyCodeEditor> {}
                }
                <FlexPanel id="preview" title="Preview"> {
                    <MyPreviewPane> {}
                }
            }
        }
    }
}
```
