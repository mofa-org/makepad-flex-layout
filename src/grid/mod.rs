//! Grid module - panel grid container with drag-and-drop
//!
//! This module provides the `PanelGrid` widget which manages a grid
//! of draggable panels.

mod drop_handler;
mod layout_state;
pub mod panel_grid;

pub use drop_handler::DropPosition;
pub use layout_state::{LayoutMode, LayoutState, SplitterPositions};
pub use panel_grid::{PanelGrid, PanelGridRef};
