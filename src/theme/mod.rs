//! Theme system for the app shell
//!
//! Provides dark/light mode switching with smooth animations using
//! shader instance variables and the `script_apply_eval!()` pattern.
//!
//! ## Usage
//!
//! ```rust,ignore
//! // In your widget's script_mod:
//! mod.widgets.MyWidget = View{
//!     draw_bg +: {
//!         dark_mode: instance(0.0)
//!         pixel: fn() {
//!             return mix(vec4(1.0, 1.0, 1.0, 1.0), vec4(0.12, 0.16, 0.23, 1.0), self.dark_mode)
//!         }
//!     }
//! }
//!
//! // At runtime:
//! script_apply_eval!(cx, widget, {
//!     draw_bg: { dark_mode: #(theme.dark_mode_anim) }
//! });
//! ```

pub mod colors;
pub mod styles;

pub use colors::*;
pub use styles::*;

use makepad_widgets::*;
use std::cell::RefCell;

// ============================================================================
// GLOBAL THEME STATE (for widgets that can't be accessed via id lookup)
// ============================================================================

thread_local! {
    /// Global dark mode animation value (0.0 = light, 1.0 = dark)
    /// Widgets can read this during draw to get the current theme state
    static GLOBAL_DARK_MODE: RefCell<f64> = RefCell::new(0.0);
}

/// Set the global dark mode value (called by ShellLayout during theme changes)
pub fn set_global_dark_mode(value: f64) {
    GLOBAL_DARK_MODE.with(|dm| *dm.borrow_mut() = value);
}

/// Get the current global dark mode value (called by widgets during draw)
pub fn get_global_dark_mode() -> f64 {
    GLOBAL_DARK_MODE.with(|dm| *dm.borrow())
}

// ============================================================================
// SHELL THEME
// ============================================================================

/// Theme state for the shell
///
/// Tracks dark mode setting and animation progress for smooth transitions.
#[derive(Clone, Debug)]
pub struct ShellTheme {
    /// Whether dark mode is enabled
    pub dark_mode: bool,

    /// Animation progress (0.0 = light, 1.0 = dark)
    /// Used for smooth transitions between themes
    pub dark_mode_anim: f64,
}

impl Default for ShellTheme {
    fn default() -> Self {
        Self {
            dark_mode: false,
            dark_mode_anim: 0.0,
        }
    }
}

impl ShellTheme {
    /// Create a new theme with light mode
    pub fn light() -> Self {
        Self {
            dark_mode: false,
            dark_mode_anim: 0.0,
        }
    }

    /// Create a new theme with dark mode
    pub fn dark() -> Self {
        Self {
            dark_mode: true,
            dark_mode_anim: 1.0,
        }
    }

    /// Set dark mode state (immediately, no animation)
    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
        self.dark_mode_anim = if dark { 1.0 } else { 0.0 };
    }

    /// Get the target animation value based on current dark_mode setting
    pub fn target_anim(&self) -> f64 {
        if self.dark_mode { 1.0 } else { 0.0 }
    }

    /// Update animation with easing (call every frame during transition)
    ///
    /// Returns true if animation is still in progress
    pub fn update_animation(&mut self, elapsed: f64, duration: f64) -> bool {
        let t = (elapsed / duration).min(1.0);

        // Ease-out cubic: 1 - (1 - t)^3
        let eased = 1.0 - (1.0 - t).powi(3);

        let target = self.target_anim();
        let start = if self.dark_mode { 0.0 } else { 1.0 };
        self.dark_mode_anim = start + (target - start) * eased;

        t < 1.0
    }
}

// ============================================================================
// THEME LISTENER TRAIT
// ============================================================================

/// Trait for widgets that respond to theme changes
///
/// Implement this trait on your widget's Ref type to receive theme updates.
///
/// ## Example
///
/// ```rust,ignore
/// impl ThemeListener for MyWidgetRef {
///     fn on_dark_mode_change(&self, cx: &mut Cx, dark_mode: f64) {
///         if let Some(mut inner) = self.borrow_mut() {
///             script_apply_eval!(cx, inner.view, {
///                 draw_bg: { dark_mode: #(dark_mode) }
///             });
///         }
///     }
/// }
/// ```
pub trait ThemeListener {
    /// Called when dark mode value changes
    ///
    /// # Arguments
    /// * `cx` - Makepad context for applying UI updates
    /// * `dark_mode` - Animation value (0.0 = light, 1.0 = dark)
    fn on_dark_mode_change(&self, cx: &mut Cx, dark_mode: f64);
}

