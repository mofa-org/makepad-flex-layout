# Flex Layout Demo - Implementation Plan

## Overview

This MVP demonstrates a fully resizable studio layout with:
- Full-width header (fixed height)
- Full-width footer (resizable height via splitter)
- Full-height left sidebar (resizable width via splitter)
- Full-height right sidebar (resizable width via splitter)
- Main content area with up to 20 sub-windows supporting multiple layout modes

## Target Layout

```
┌─────────────────────────────────────────────────────────────────┐
│                        HEADER (48px fixed)                       │
├─────────┬───────────────────────────────────────────┬───────────┤
│         │                                           │           │
│  LEFT   │              MAIN CONTENT                 │   RIGHT   │
│ SIDEBAR │  ┌─────┬─────┬─────┬─────┐              │  SIDEBAR  │
│         │  │ W1  │ W2  │ W3  │ W4  │              │           │
│  280px  │  ├─────┼─────┼─────┼─────┤              │   300px   │
│ default │  │ W5  │ W6  │ W7  │ W8  │              │  default  │
│         │  ├─────┴─────┴─────┴─────┤              │           │
│ ←─────→ │  │    (auto-grid or      │              │ ←───────→ │
│ 180-400 │  │   tabs/h-stack/v-stack)│              │  200-450  │
│         │  └────────────────────────┘              │           │
├─────────┴───────────────────────────────────────────┴───────────┤
│                       FOOTER (100px default)                     │
│                          ↕ 60-200px                              │
└─────────────────────────────────────────────────────────────────┘
```

## Implementation Steps

### Step 1: Project Setup (5 min)
- [x] Create `Cargo.toml` with makepad-widgets dependency
- [ ] Create `src/main.rs` with app boilerplate
- [ ] Create `src/app.rs` with live_design registration

### Step 2: Basic Layout Structure (30 min)
- [ ] Implement outer frame with header/footer/content split
- [ ] Add vertical splitter for footer resize
- [ ] Test basic header/content/footer layout

### Step 3: Sidebar Splitters (30 min)
- [ ] Add left sidebar with horizontal splitter
- [ ] Add right sidebar with horizontal splitter
- [ ] Wire up splitter min/max constraints
- [ ] Test 3-column resizable layout

### Step 4: Sub-Window Component (30 min)
- [ ] Create `SubWindow` widget with:
  - Title bar with window number
  - Content area with colored background
  - Close button (optional)
- [ ] Test single sub-window rendering

### Step 5: Layout Modes (1 hour)
- [ ] Implement `AutoGrid` layout (responsive grid)
- [ ] Implement `HStack` layout (horizontal flow)
- [ ] Implement `VStack` layout (vertical flow)
- [ ] Implement `Tabbed` layout (tab bar + page flip)
- [ ] Create layout mode selector in right sidebar

### Step 6: Window Management (30 min)
- [ ] Add "Add Window" button
- [ ] Add "Remove Window" button
- [ ] Track window count (1-20)
- [ ] Dynamic window creation

### Step 7: Polish & Testing (30 min)
- [ ] Add visual feedback on splitter hover
- [ ] Test all layout modes with various window counts
- [ ] Test extreme resize scenarios
- [ ] Add keyboard shortcuts (optional)

## File Structure

```
examples/flex-layout-demo/
├── Cargo.toml
├── IMPLEMENTATION_PLAN.md
└── src/
    ├── main.rs           # Entry point
    ├── app.rs            # App + live_design
    ├── layout.rs         # ResizableStudioLayout widget
    ├── sub_window.rs     # SubWindow widget
    └── content_area.rs   # ContentArea with layout modes
```

## Key Widgets

### 1. ResizableStudioLayout
Main container with nested splitters:
```
Splitter(Vertical) [content | footer]
└─ Splitter(Horizontal) [left | center+right]
   └─ Splitter(Horizontal) [center | right]
```

### 2. SubWindow
Individual content panel:
- Numbered title bar
- Colored content area
- Drag handle (future)

### 3. ContentArea
Manages sub-window layout:
- Tracks layout mode (Auto/HStack/VStack/Tabbed)
- Creates/destroys sub-windows
- Handles layout switching

## Layout Mode Details

### Auto Grid
- Calculates optimal columns based on width
- Minimum window size: 200x150
- Responsive reflow on resize

### Horizontal Stack (HStack)
- All windows in single row
- Equal width distribution
- Horizontal scroll if needed

### Vertical Stack (VStack)
- All windows in single column
- Equal height distribution
- Vertical scroll if needed

### Tabbed
- Tab bar at top
- Single window visible at a time
- Tab switching via click

## Splitter Constraints

| Splitter | Axis | Min | Max | Default |
|----------|------|-----|-----|---------|
| Footer | Vertical | 60px | 200px | 100px |
| Left Sidebar | Horizontal | 180px | 400px | 280px |
| Right Sidebar | Horizontal | 200px | 450px | 300px |

## Success Criteria

1. All splitters are draggable and respect constraints
2. Layout modes switch correctly
3. Windows can be added/removed (1-20)
4. No visual glitches during resize
5. App doesn't crash on edge cases

## Estimated Time: 3-4 hours

---

## Quick Start

```bash
cd examples/flex-layout-demo
cargo run
```

## Controls

- Drag splitter edges to resize panels
- Use layout mode dropdown in right sidebar
- Click "+" to add windows (max 20)
- Click "-" to remove windows (min 1)
