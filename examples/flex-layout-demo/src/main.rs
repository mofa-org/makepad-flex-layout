//! Flex Layout Demo
//!
//! Demo application showcasing the makepad-app-shell library.

// Re-export dependencies for live_design compatibility
pub use makepad_widgets;
pub use makepad_app_shell;

mod app;

fn main() {
    app::app_main();
}
