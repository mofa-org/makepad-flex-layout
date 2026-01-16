//! Live design registration for the app shell
//!
//! Theme system based on mofa-studio patterns with Manrope font.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ============================================================================
    // FONT DEFINITIONS (Manrope)
    // ============================================================================

    pub FONT_REGULAR = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }
    pub FONT_MEDIUM = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Medium.ttf", 0.0, 0.0),
        }
    }
    pub FONT_SEMIBOLD = {
        font_family: {
            latin = font("crate://self/resources/Manrope-SemiBold.ttf", 0.0, 0.0),
        }
    }
    pub FONT_BOLD = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Bold.ttf", 0.0, 0.0),
        }
    }

    // ============================================================================
    // TEXT STYLES
    // ============================================================================

    pub TEXT_HEADER = <FONT_SEMIBOLD> {
        font_size: 14.0
    }

    pub TEXT_LABEL = <FONT_REGULAR> {
        font_size: 12.0
    }

    pub TEXT_SMALL = <FONT_REGULAR> {
        font_size: 11.0
    }
}
