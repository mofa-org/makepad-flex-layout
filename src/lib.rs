//! Makepad App Shell
//!
//! A reusable IDE/studio layout shell for Makepad applications.
//!
//! ## Features
//!
//! - Dock-based resizable layout (header, footer, left/right sidebars, content area)
//! - Draggable panel grid with maximize/close functionality
//! - Dark/light theme switching with smooth animations
//! - Layout state persistence
//! - Extensible content injection via traits
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use makepad_widgets::*;
//! use makepad_app_shell::prelude::*;
//!
//! live_design! {
//!     use makepad_app_shell::widgets::*;
//!
//!     App = {{App}} {
//!         ui: <Root> {
//!             main_window = <Window> {
//!                 body = <ShellLayout> {}
//!             }
//!         }
//!     }
//! }
//! ```

pub mod theme;
pub mod shell;
pub mod panel;
pub mod grid;
pub mod callbacks;
pub mod persistence;

mod live_design;

use makepad_widgets::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::theme::{ShellTheme, ThemeListener};
    pub use crate::shell::config::ShellConfig;
    pub use crate::panel::{Panel, PanelAction};
    pub use crate::grid::{PanelGrid, LayoutState};
    pub use crate::callbacks::ShellCallbacks;
    pub use crate::persistence::ShellPreferences;
}

/// Widget exports for use in live_design!
pub mod widgets {
    pub use crate::shell::layout::{ShellLayout, ShellLayoutRef};
    pub use crate::shell::header::{ShellHeader, ShellHeaderRef};
    pub use crate::shell::footer::{ShellFooter, ShellFooterRef};
    pub use crate::shell::sidebar::{ShellSidebar, ShellSidebarRef};
    pub use crate::panel::{Panel, PanelRef};
    pub use crate::grid::{PanelGrid, PanelGridRef};
}

/// Register all live_design components with Makepad
///
/// Note: The calling application should call `makepad_widgets::live_design(cx)` before
/// calling this function.
pub fn live_design(cx: &mut Cx) {
    // Register base live_design (colors, styles)
    crate::live_design::live_design(cx);

    // Register panel widget
    crate::panel::panel::live_design(cx);

    // Register grid widget
    crate::grid::panel_grid::live_design(cx);

    // Register shell components
    crate::shell::header::live_design(cx);
    crate::shell::footer::live_design(cx);
    crate::shell::sidebar::live_design(cx);
    crate::shell::layout::live_design(cx);
}
