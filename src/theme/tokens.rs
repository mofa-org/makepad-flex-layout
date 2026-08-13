//! The shell's palette, as DSL tokens an application can replace.
//!
//! ## Why this exists
//!
//! Every shell widget used to write its colours as literals inside its own
//! shader body:
//!
//! ```text
//! pixel: fn() {
//!     let light = vec4(1.0, 1.0, 1.0, 1.0)
//!     let dark  = vec4(0.059, 0.090, 0.165, 1.0)
//!     return mix(light, dark, self.dark_mode)
//! }
//! ```
//!
//! That is unreachable from an application, and not for the reason it looks.
//! Redefining `mod.widgets.ShellHeader` in an app's own `script_mod!` compiles,
//! runs, and silently does nothing — because `ShellLayout` composes
//! `header := ShellHeader{}` into its tree when *this crate* registers, so the
//! instance captured the prototype as it stood then. A later redefinition has
//! nothing left to affect.
//!
//! So the colours move out of the shader bodies and become **uniforms with
//! token defaults**. Restyling is then setting values, which composes, rather
//! than replacing shader source, which does not.
//!
//! ## Using it
//!
//! Call [`crate::script_mod_with_theme`] and pass a closure that overrides the
//! tokens you care about. It runs between the defaults below and the widgets
//! that bind them:
//!
//! ```ignore
//! makepad_app_shell::script_mod_with_theme(vm, |vm| {
//!     script_mod! {
//!         mod.widgets.shell.BG_HEADER_L = #xFBFAF8
//!         mod.widgets.shell.BG_HEADER_D = #x141312
//!     }(vm)
//! });
//! ```
//!
//! ## Defaults are the current look, deliberately
//!
//! The values below are lifted from what the shaders actually drew, not from
//! `colors.rs` — whose constants had drifted away from the rendering (it claims
//! a blue `#4080c0` header while the shader drew white) and which nothing read.
//! Hoisting is therefore a visual no-op: same pixels, now reachable.
//!
//! `_L` is the light-theme value, `_D` the dark one; widgets mix between them
//! on `dark_mode`, so both halves of a pair want setting.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    // The namespace is created by `live_design`, which also puts the fonts in
    // it. Deliberately not re-created here: `mod.widgets.shell = {}` would
    // reset it and take the fonts with it.

    // -------------------------------------------------------------- chrome --
    // Header: white over slate-900.
    mod.widgets.shell.BG_HEADER_L = #xFFFFFF
    mod.widgets.shell.BG_HEADER_D = #x0F172A

    // Sidebar and its menu rail.
    mod.widgets.shell.BG_SIDEBAR_L = #xF8FAFC
    mod.widgets.shell.BG_SIDEBAR_D = #x1E2633

    // Footer strip.
    mod.widgets.shell.BG_FOOTER_L = #xF8FAFC
    mod.widgets.shell.BG_FOOTER_D = #x1A1A1F

    // The ground everything sits on.
    mod.widgets.shell.BG_CONTENT_L = #xE8E8F0
    mod.widgets.shell.BG_CONTENT_D = #x0F172A

    // --------------------------------------------------------------- panel --
    // Panel body, its title bar, and the hairline between them.
    mod.widgets.shell.BG_PANEL_L = #xFFFFFF
    mod.widgets.shell.BG_PANEL_D = #x1A1F29
    mod.widgets.shell.BG_PANEL_TITLE_L = #xF1F5F9
    mod.widgets.shell.BG_PANEL_TITLE_D = #x262D3B

    // ------------------------------------------------------------- borders --
    mod.widgets.shell.BORDER_L = #xE3E7EF
    mod.widgets.shell.BORDER_D = #x262D3B

    // ---------------------------------------------------------------- text --
    mod.widgets.shell.TEXT_L = #x202020
    mod.widgets.shell.TEXT_D = #xF1F5F9
    mod.widgets.shell.TEXT_DIM_L = #x606060
    mod.widgets.shell.TEXT_DIM_D = #x94A3B8

    // --------------------------------------------------------------- accent --
    mod.widgets.shell.ACCENT_L = #x2060A0
    mod.widgets.shell.ACCENT_D = #x60A5FA
}
