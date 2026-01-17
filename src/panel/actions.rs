//! Panel actions for widget-to-widget communication

use makepad_widgets::*;

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

    /// No action
    None,
}
