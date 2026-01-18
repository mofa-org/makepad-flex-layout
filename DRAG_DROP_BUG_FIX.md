# Drag-and-Drop Bug Fix: First Drag Fails, Second Succeeds

## Bug Description

When dragging a panel in the grid:
1. First drag attempt always fails (panel doesn't move)
2. Second drag attempt succeeds
3. Sometimes dragging causes wrong panel to move ("switches window position")

## Root Cause Analysis

The bug is caused by a **finger capture and stale state issue** between `Panel` and `PanelGrid` widgets.

### Makepad Event Capture Model

In Makepad, when a widget receives `Hit::FingerDown` and processes it, that widget "captures" the finger. All subsequent events for that finger (`FingerMove`, `FingerUp`) are sent to the capturing widget until released.

### The Problem

#### Event Flow - First Drag (Fails)

```
1. FingerDown on Panel's drag_handle
   -> Panel calls event.hits() and CAPTURES the finger

2. FingerMove (distance > 10px threshold)
   -> Panel detects drag, emits PanelAction::StartDrag
   -> PanelGrid receives action, sets dragging_panel = Some(id)
   -> PanelGrid calls hits_with_capture_overload(..., true)
   -> BUT: Panel already captured the finger on FingerDown!

3. FingerUp
   -> Goes to Panel (which has finger capture), NOT to PanelGrid
   -> Panel just sets is_dragging = false
   -> PanelGrid NEVER receives FingerUp
   -> dragging_panel stays Some(id) - STALE STATE!

4. Result: Drop never happens, stale dragging_panel remains
```

#### Why "Switches Window Position"

When `dragging_panel` is stale (left over from failed first drag):
1. User clicks anywhere on the screen
2. `hits_with_capture_overload(..., true)` captures that random FingerUp
3. PanelGrid's FingerUp handler runs with stale `dragging_panel`
4. Calls `handle_drop()` with wrong panel ID and random position
5. Wrong panel moves to unexpected location!

### Code Locations

**Panel (`src/panel/panel.rs`):**
- Captures finger on FingerDown via `event.hits(cx, drag_handle.area())`
- On FingerUp, only set `is_dragging = false` without notifying PanelGrid

**PanelGrid (`src/grid/panel_grid.rs`):**
- Uses `hits_with_capture_overload(cx, self.view.area(), self.dragging_panel.is_some())`
- `capture_overload=true` only AFTER StartDrag, but Panel already has capture
- FingerUp handler calls `handle_drop()` for ANY FingerUp when `dragging_panel` is Some

## Solution

The fix ensures that:
1. Panel explicitly notifies PanelGrid when a drag ends via `EndDrag` action
2. PanelGrid only processes drops from `EndDrag` action, not from arbitrary FingerUp events
3. FingerUp in PanelGrid only clears state as a fallback, doesn't trigger drops

### Changes Made

1. **Added `EndDrag(LiveId, DVec2)` action** (`src/panel/actions.rs`):
   - Emitted by Panel when FingerUp occurs during active drag
   - Contains panel ID and cursor position for drop calculation

2. **Panel emits EndDrag on FingerUp** (`src/panel/panel.rs`):
   ```rust
   Hit::FingerUp(fe) => {
       if self.is_dragging {
           cx.widget_action(..., PanelAction::EndDrag(self.panel_id, fe.abs));
       }
       self.is_dragging = false;
   }
   ```

3. **PanelGrid handles EndDrag for drops** (`src/grid/panel_grid.rs`):
   ```rust
   PanelAction::EndDrag(id, abs) => {
       if self.dragging_panel == Some(id.0) {
           self.handle_drop(cx, abs, id.0);
           layout_changed = true;
       }
       self.dragging_panel = None;
       self.drop_state = None;
   }
   ```

4. **PanelGrid FingerUp only clears state**:
   ```rust
   Hit::FingerUp(_) => {
       // Clear state on any FingerUp as fallback
       // (actual drop is handled via EndDrag action from Panel)
       self.dragging_panel = None;
       self.drop_state = None;
   }
   ```

5. **FooterGrid updated similarly** for footer panel drops.

## Why This Fix Works

- Panel maintains finger capture (no architectural change needed)
- Panel explicitly signals drag end with position via action
- PanelGrid receives drop info through action system, not unreliable FingerUp hits
- No stale `dragging_panel` state causing wrong drops
- Drops complete reliably on first attempt

## Key Insight

The original code assumed `hits_with_capture_overload` could steal capture from child widgets. In practice, once a widget captures a finger on FingerDown, subsequent events for that finger go to the capturing widget regardless of `capture_overload` calls by parent widgets.

The solution is to have the capturing widget (Panel) participate in the drop flow by emitting an action with the necessary position information.
