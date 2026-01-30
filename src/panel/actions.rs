//! Panel actions for widget-to-widget communication

use makepad_widgets::*;
use crate::grid::{LayoutState, FooterLayoutState};

/// Actions emitted by Panel widgets to communicate with parent containers.
///
/// These are dispatched via `cx.widget_action()` and handled by PanelGrid/FooterGrid.
#[derive(Clone, Debug, DefaultNone)]
pub enum PanelAction {
    /// Panel close button clicked
    Close(LiveId),

    /// Maximize/restore button clicked (for main grid panels)
    Maximize(LiveId),

    /// Fullscreen button clicked (panel takes entire dock space)
    Fullscreen(LiveId),

    /// Drag operation started (threshold exceeded)
    StartDrag(LiveId),

    /// Drag operation ended - finger released at position
    ///
    /// Emitted by Panel when FingerUp occurs during an active drag.
    /// Contains the panel ID and the absolute cursor position for drop calculation.
    /// PanelGrid/FooterGrid handle this to complete the drop operation.
    EndDrag(LiveId, DVec2),

    /// Layout has changed (emitted by PanelGrid for persistence)
    LayoutChanged(LayoutState),

    /// Footer layout has changed (emitted by FooterGrid for persistence)
    FooterLayoutChanged(FooterLayoutState),

    /// Request to reset layout to default (emitted by ShellLayout, handled by grids)
    ResetLayout,

    /// No action
    None,
}
