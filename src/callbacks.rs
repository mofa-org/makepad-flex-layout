//! Callback traits for shell events
//!
//! Implement these traits to receive notifications about layout changes.

use makepad_widgets::*;
use crate::grid::LayoutState;

/// Callback trait for shell events
///
/// Implement this trait on your App to receive notifications about layout changes.
///
/// ## Example
///
/// ```rust,ignore
/// impl ShellCallbacks for MyApp {
///     fn on_panel_closed(&mut self, cx: &mut Cx, panel_id: LiveId) {
///         log!("Panel {:?} was closed", panel_id);
///     }
///
///     fn on_layout_changed(&mut self, cx: &mut Cx, state: &LayoutState) {
///         // Save layout to disk
///         save_layout("my_app", state).ok();
///     }
/// }
/// ```
pub trait ShellCallbacks {
    /// Called when a panel is closed
    fn on_panel_closed(&mut self, _cx: &mut Cx, _panel_id: LiveId) {}

    /// Called when a panel is maximized or restored
    fn on_panel_maximized(&mut self, _cx: &mut Cx, _panel_id: LiveId, _maximized: bool) {}

    /// Called when a panel is moved (drag-drop)
    fn on_panel_moved(
        &mut self,
        _cx: &mut Cx,
        _panel_id: LiveId,
        _from: (usize, usize),
        _to: (usize, usize),
    ) {
    }

    /// Called when layout state changes (for persistence)
    fn on_layout_changed(&mut self, _cx: &mut Cx, _state: &LayoutState) {}

    /// Called when dark mode setting changes
    fn on_dark_mode_changed(&mut self, _cx: &mut Cx, _dark_mode: bool) {}

    /// Called when a splitter position changes
    fn on_splitter_changed(&mut self, _cx: &mut Cx, _splitter: SplitterId, _position: f64) {}
}

/// Identifier for splitters in the shell layout
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitterId {
    /// Left sidebar splitter
    LeftSidebar,
    /// Right sidebar splitter
    RightSidebar,
    /// Footer splitter
    Footer,
}
