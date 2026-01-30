//! Grid module - panel grid container with drag-and-drop
//!
//! This module provides the `PanelGrid` widget which manages a grid
//! of draggable panels, and `FooterGrid` for the footer panel strip.

mod drop_handler;
mod layout_state;
pub mod panel_grid;
pub mod footer_grid;

pub use drop_handler::DropPosition;
pub use layout_state::{LayoutMode, LayoutState, SplitterPositions, FooterLayoutState, FooterSlotState};
pub use panel_grid::{PanelGrid, PanelGridRef, PanelGridWidgetRefExt};
pub use footer_grid::{FooterGrid, FooterGridRef, FooterGridWidgetRefExt};
