//! Shell module - main layout components for the app shell
//!
//! This module provides the core layout widgets:
//! - `ShellLayout` - Main container with header, footer, sidebars, and content
//! - `ShellHeader` - Top header bar
//! - `ShellFooter` - Bottom footer/status bar
//! - `ShellSidebar` - Left and right sidebars
//! - `ShellConfig` - Configuration options

pub mod config;
pub mod header;
pub mod footer;
pub mod sidebar;
pub mod layout;

// Re-export live_design functions
pub use header::live_design as header_live_design;
pub use footer::live_design as footer_live_design;
pub use sidebar::live_design as sidebar_live_design;
pub use layout::live_design as layout_live_design;

pub use config::{ShellConfig, ShellConfigBuilder};
pub use header::{ShellHeader, ShellHeaderRef};
pub use footer::{ShellFooter, ShellFooterRef};
pub use sidebar::{ShellSidebar, ShellSidebarRef};
pub use layout::{ShellLayout, ShellLayoutRef};
