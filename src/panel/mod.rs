//! Panel module - draggable window panels for the app shell
//!
//! This module provides the `Panel` widget which represents a single
//! draggable panel in the grid layout.

mod actions;
pub mod panel;

pub use actions::PanelAction;
pub use panel::{Panel, PanelRef};
