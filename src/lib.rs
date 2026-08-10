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
//! script_mod! {
//!     use mod.prelude.widgets.*
//!     use mod.widgets.*
//!
//!     startup() do #(App::script_component(vm)){
//!         ui: Root{
//!             main_window := Window{
//!                 body +: { ShellLayout{} }
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
pub mod registry;

mod live_design;

use makepad_widgets::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::theme::{ShellTheme, ThemeListener};
    pub use crate::shell::config::ShellConfig;
    pub use crate::panel::{Panel, PanelAction};
    pub use crate::grid::{PanelGrid, FooterGrid, LayoutState};
    pub use crate::callbacks::ShellCallbacks;
    pub use crate::persistence::ShellPreferences;
    pub use crate::registry::{PanelDefinition, PanelRegistry};
}

/// Widget exports for use in script_mod!
pub mod widgets {
    pub use crate::shell::layout::{ShellLayout, ShellLayoutRef};
    pub use crate::shell::header::{ShellHeader, ShellHeaderRef};
    pub use crate::shell::footer::{ShellFooter, ShellFooterRef};
    pub use crate::shell::sidebar::{ShellSidebar, ShellSidebarRef, SidebarAction, SidebarSelection};
    pub use crate::panel::{Panel, PanelRef};
    pub use crate::grid::{PanelGrid, PanelGridRef, FooterGrid, FooterGridRef};
}

/// Register all script modules with Makepad
///
/// Note: The calling application should call `makepad_widgets::script_mod(vm)` before
/// calling this function.
pub fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
    // Register base script module (fonts, text styles)
    crate::live_design::script_mod(vm);

    // Register panel widget
    crate::panel::panel::script_mod(vm);

    // Register shell components (sidebar_menu must come before sidebar;
    // sidebar must come before the grids, whose DSL references it)
    crate::shell::header::script_mod(vm);
    crate::shell::footer::script_mod(vm);
    crate::shell::sidebar_menu::script_mod(vm);
    crate::shell::sidebar::script_mod(vm);

    // Register grid widgets
    crate::grid::panel_grid::script_mod(vm);
    crate::grid::footer_grid::script_mod(vm);

    // The layout ties everything together and must come last
    crate::shell::layout::script_mod(vm)
}
