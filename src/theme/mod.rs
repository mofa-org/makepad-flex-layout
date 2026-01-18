//! Theme system for the app shell
//!
//! Provides dark/light mode switching with smooth animations using
//! shader instance variables and the `apply_over()` pattern.
//!
//! ## Usage
//!
//! ```rust,ignore
//! // In your widget's live_design:
//! MyWidget = <View> {
//!     draw_bg: {
//!         instance dark_mode: 0.0
//!         fn pixel(self) -> vec4 {
//!             return mix((BG_PANEL), (BG_PANEL_DARK), self.dark_mode);
//!         }
//!     }
//! }
//!
//! // At runtime:
//! widget.apply_over(cx, live!{
//!     draw_bg: { dark_mode: (theme.dark_mode_anim) }
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
///             inner.view.apply_over(cx, live!{
///                 draw_bg: { dark_mode: (dark_mode) }
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

// ============================================================================
// LIVE DESIGN MACROS
// ============================================================================

/// Generate live_design color constants
///
/// This macro is used internally to define colors in live_design! blocks.
#[macro_export]
macro_rules! shell_colors {
    () => {
        // Light mode semantic colors
        BG_APP = #f5f7fa
        BG_HEADER = #4080c0
        BG_SIDEBAR = #80a0d0
        BG_FOOTER = #60a060
        BG_CONTENT = #e8e8f0
        BG_PANEL = #ffffff
        TEXT_PRIMARY = #202020
        TEXT_SECONDARY = #606060
        ACCENT = #2060a0
        BORDER = #a0a0b0

        // Dark mode semantic colors
        BG_APP_DARK = #0f172a
        BG_HEADER_DARK = #264060
        BG_SIDEBAR_DARK = #1e2633
        BG_FOOTER_DARK = #264026
        BG_CONTENT_DARK = #1a1a1f
        BG_PANEL_DARK = #1f293b
        TEXT_PRIMARY_DARK = #f1f5f9
        TEXT_SECONDARY_DARK = #94a3b8
        ACCENT_DARK = #60a5fa
        BORDER_DARK = #4d4d59

        // Slate palette
        SLATE_50 = #f8fafc
        SLATE_100 = #f1f5f9
        SLATE_200 = #e2e8f0
        SLATE_300 = #cbd5e1
        SLATE_400 = #94a3b8
        SLATE_500 = #64748b
        SLATE_600 = #475569
        SLATE_700 = #334155
        SLATE_800 = #1e293b
        SLATE_900 = #0f172a

        // Blue palette
        BLUE_50 = #eff6ff
        BLUE_100 = #dbeafe
        BLUE_200 = #bfdbfe
        BLUE_300 = #93c5fd
        BLUE_400 = #60a5fa
        BLUE_500 = #3b82f6
        BLUE_600 = #2560db
        BLUE_700 = #1c4dd7

        // Common colors
        WHITE = #ffffff
        BLACK = #000000
        TRANSPARENT = #00000000
    };
}
