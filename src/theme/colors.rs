//! Color palette for the app shell
//!
//! Tailwind-inspired color system with light/dark variants.
//! Colors are defined as live_design constants for use in shaders.

use makepad_widgets::*;

// ============================================================================
// SEMANTIC COLORS - LIGHT MODE
// ============================================================================

/// Main app background (light)
pub const BG_APP: Vec4 = vec4(0.96, 0.97, 0.98, 1.0);           // #f5f7fa

/// Header background (light)
pub const BG_HEADER: Vec4 = vec4(0.25, 0.50, 0.75, 1.0);        // #4080c0

/// Sidebar background (light)
pub const BG_SIDEBAR: Vec4 = vec4(0.50, 0.63, 0.82, 1.0);       // #80a0d0

/// Footer background (light)
pub const BG_FOOTER: Vec4 = vec4(0.38, 0.63, 0.38, 1.0);        // #60a060

/// Content area background (light)
pub const BG_CONTENT: Vec4 = vec4(0.91, 0.91, 0.94, 1.0);       // #e8e8f0

/// Panel/card background (light)
pub const BG_PANEL: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);            // #ffffff

/// Primary text (light)
pub const TEXT_PRIMARY: Vec4 = vec4(0.13, 0.13, 0.13, 1.0);     // #202020

/// Secondary/dim text (light)
pub const TEXT_SECONDARY: Vec4 = vec4(0.38, 0.38, 0.38, 1.0);   // #606060

/// Accent color (light)
pub const ACCENT: Vec4 = vec4(0.13, 0.38, 0.63, 1.0);           // #2060a0

/// Border color (light)
pub const BORDER: Vec4 = vec4(0.63, 0.63, 0.69, 1.0);           // #a0a0b0

// ============================================================================
// SEMANTIC COLORS - DARK MODE
// ============================================================================

/// Main app background (dark)
pub const BG_APP_DARK: Vec4 = vec4(0.06, 0.09, 0.16, 1.0);      // #0f172a

/// Header background (dark)
pub const BG_HEADER_DARK: Vec4 = vec4(0.15, 0.25, 0.38, 1.0);   // #264060

/// Sidebar background (dark)
pub const BG_SIDEBAR_DARK: Vec4 = vec4(0.12, 0.15, 0.20, 1.0);  // #1e2633

/// Footer background (dark)
pub const BG_FOOTER_DARK: Vec4 = vec4(0.15, 0.25, 0.15, 1.0);   // #264026

/// Content area background (dark)
pub const BG_CONTENT_DARK: Vec4 = vec4(0.10, 0.10, 0.12, 1.0);  // #1a1a1f

/// Panel/card background (dark)
pub const BG_PANEL_DARK: Vec4 = vec4(0.12, 0.16, 0.23, 1.0);    // #1f293b

/// Primary text (dark)
pub const TEXT_PRIMARY_DARK: Vec4 = vec4(0.95, 0.96, 0.98, 1.0); // #f1f5f9

/// Secondary/dim text (dark)
pub const TEXT_SECONDARY_DARK: Vec4 = vec4(0.58, 0.64, 0.72, 1.0); // #94a3b8

/// Accent color (dark)
pub const ACCENT_DARK: Vec4 = vec4(0.38, 0.65, 0.98, 1.0);      // #60a5fa

/// Border color (dark)
pub const BORDER_DARK: Vec4 = vec4(0.30, 0.30, 0.35, 1.0);      // #4d4d59

// ============================================================================
// TAILWIND SLATE PALETTE
// ============================================================================

pub const SLATE_50: Vec4 = vec4(0.97, 0.98, 0.99, 1.0);         // #f8fafc
pub const SLATE_100: Vec4 = vec4(0.95, 0.96, 0.98, 1.0);        // #f1f5f9
pub const SLATE_200: Vec4 = vec4(0.89, 0.91, 0.94, 1.0);        // #e2e8f0
pub const SLATE_300: Vec4 = vec4(0.80, 0.84, 0.89, 1.0);        // #cbd5e1
pub const SLATE_400: Vec4 = vec4(0.58, 0.64, 0.72, 1.0);        // #94a3b8
pub const SLATE_500: Vec4 = vec4(0.39, 0.45, 0.55, 1.0);        // #64748b
pub const SLATE_600: Vec4 = vec4(0.28, 0.33, 0.41, 1.0);        // #475569
pub const SLATE_700: Vec4 = vec4(0.20, 0.25, 0.33, 1.0);        // #334155
pub const SLATE_800: Vec4 = vec4(0.12, 0.16, 0.23, 1.0);        // #1e293b
pub const SLATE_900: Vec4 = vec4(0.06, 0.09, 0.16, 1.0);        // #0f172a
pub const SLATE_950: Vec4 = vec4(0.01, 0.03, 0.06, 1.0);        // #020617

// ============================================================================
// TAILWIND BLUE PALETTE
// ============================================================================

pub const BLUE_50: Vec4 = vec4(0.94, 0.96, 1.0, 1.0);           // #eff6ff
pub const BLUE_100: Vec4 = vec4(0.86, 0.92, 0.99, 1.0);         // #dbeafe
pub const BLUE_200: Vec4 = vec4(0.74, 0.85, 0.98, 1.0);         // #bfdbfe
pub const BLUE_300: Vec4 = vec4(0.58, 0.75, 0.96, 1.0);         // #93c5fd
pub const BLUE_400: Vec4 = vec4(0.38, 0.65, 0.98, 1.0);         // #60a5fa
pub const BLUE_500: Vec4 = vec4(0.23, 0.51, 0.96, 1.0);         // #3b82f6
pub const BLUE_600: Vec4 = vec4(0.15, 0.39, 0.92, 1.0);         // #2563eb
pub const BLUE_700: Vec4 = vec4(0.11, 0.31, 0.85, 1.0);         // #1d4ed8
pub const BLUE_800: Vec4 = vec4(0.12, 0.25, 0.69, 1.0);         // #1e40af
pub const BLUE_900: Vec4 = vec4(0.12, 0.23, 0.55, 1.0);         // #1e3a8a

// ============================================================================
// TAILWIND GREEN PALETTE
// ============================================================================

pub const GREEN_50: Vec4 = vec4(0.94, 0.99, 0.96, 1.0);         // #f0fdf4
pub const GREEN_100: Vec4 = vec4(0.86, 0.98, 0.91, 1.0);        // #dcfce7
pub const GREEN_200: Vec4 = vec4(0.73, 0.95, 0.82, 1.0);        // #bbf7d0
pub const GREEN_300: Vec4 = vec4(0.52, 0.90, 0.68, 1.0);        // #86efac
pub const GREEN_400: Vec4 = vec4(0.29, 0.82, 0.55, 1.0);        // #4ade80
pub const GREEN_500: Vec4 = vec4(0.13, 0.72, 0.44, 1.0);        // #22c55e
pub const GREEN_600: Vec4 = vec4(0.09, 0.60, 0.36, 1.0);        // #16a34a
pub const GREEN_700: Vec4 = vec4(0.08, 0.49, 0.31, 1.0);        // #15803d
pub const GREEN_800: Vec4 = vec4(0.09, 0.39, 0.27, 1.0);        // #166534
pub const GREEN_900: Vec4 = vec4(0.08, 0.32, 0.23, 1.0);        // #14532d

// ============================================================================
// TAILWIND RED PALETTE
// ============================================================================

pub const RED_50: Vec4 = vec4(0.99, 0.95, 0.95, 1.0);           // #fef2f2
pub const RED_100: Vec4 = vec4(0.99, 0.89, 0.89, 1.0);          // #fee2e2
pub const RED_200: Vec4 = vec4(0.99, 0.79, 0.79, 1.0);          // #fecaca
pub const RED_300: Vec4 = vec4(0.99, 0.65, 0.65, 1.0);          // #fca5a5
pub const RED_400: Vec4 = vec4(0.97, 0.44, 0.44, 1.0);          // #f87171
pub const RED_500: Vec4 = vec4(0.94, 0.27, 0.27, 1.0);          // #ef4444
pub const RED_600: Vec4 = vec4(0.86, 0.15, 0.15, 1.0);          // #dc2626
pub const RED_700: Vec4 = vec4(0.73, 0.11, 0.11, 1.0);          // #b91c1c
pub const RED_800: Vec4 = vec4(0.60, 0.11, 0.11, 1.0);          // #991b1b
pub const RED_900: Vec4 = vec4(0.50, 0.13, 0.13, 1.0);          // #7f1d1d

// ============================================================================
// PANEL COLORS (for distinguishing panels in grid)
// ============================================================================

/// Default panel color palette (light, professional colors)
pub fn panel_colors() -> [Vec4; 9] {
    [
        vec4(1.0, 1.0, 1.0, 1.0),       // White
        vec4(0.973, 0.980, 0.988, 1.0), // Slate-50
        vec4(0.945, 0.961, 0.976, 1.0), // Slate-100
        vec4(0.886, 0.910, 0.941, 1.0), // Slate-200
        vec4(0.796, 0.835, 0.882, 1.0), // Slate-300
        vec4(0.937, 0.965, 1.0, 1.0),   // Blue-50
        vec4(0.859, 0.918, 0.996, 1.0), // Blue-100
        vec4(0.941, 0.988, 0.961, 1.0), // Green-50
        vec4(0.855, 0.973, 0.906, 1.0), // Green-100
    ]
}

/// Dark mode panel color palette (slightly brighter)
pub fn panel_colors_dark() -> [Vec4; 9] {
    [
        vec4(0.90, 0.35, 0.35, 1.0),   // Red
        vec4(0.35, 0.80, 0.35, 1.0),   // Green
        vec4(0.35, 0.55, 0.90, 1.0),   // Blue
        vec4(0.90, 0.80, 0.35, 1.0),   // Yellow
        vec4(0.80, 0.35, 0.80, 1.0),   // Magenta
        vec4(0.35, 0.80, 0.80, 1.0),   // Cyan
        vec4(1.00, 0.60, 0.35, 1.0),   // Orange
        vec4(0.60, 0.35, 0.90, 1.0),   // Purple
        vec4(0.50, 0.90, 0.50, 1.0),   // Light green
    ]
}
