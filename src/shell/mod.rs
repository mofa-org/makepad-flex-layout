//! Shell module - main layout components for the app shell
//!
//! This module provides the core layout widgets:
//! - `ShellLayout` - Main container with header, footer, sidebars, and content
//! - `ShellHeader` - Top header bar
//! - `ShellFooter` - Bottom footer/status bar
//! - `ShellSidebar` - Left and right sidebars
//! - `SidebarMenuItem` - Menu items with hover effects
//! - `ShellConfig` - Configuration options

pub mod config;
pub mod header;
pub mod footer;
pub mod sidebar;
pub mod sidebar_menu;
pub mod layout;

// Re-export script_mod functions
pub use header::script_mod as header_script_mod;
pub use footer::script_mod as footer_script_mod;
pub use sidebar::script_mod as sidebar_script_mod;
pub use sidebar_menu::script_mod as sidebar_menu_script_mod;
pub use layout::script_mod as layout_script_mod;

pub use config::{ShellConfig, ShellConfigBuilder};
pub use header::{ShellHeader, ShellHeaderRef};
pub use footer::{ShellFooter, ShellFooterRef};
pub use sidebar::{ShellSidebar, ShellSidebarRef};
// sidebar_menu exports live_design templates (SidebarMenuButton, ShowMoreContainer, MoreAppsSection)
pub use layout::{ShellLayout, ShellLayoutRef};
