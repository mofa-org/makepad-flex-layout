//! Script module registration for the app shell
//!
//! Theme system based on mofa-studio patterns with Manrope font.

use makepad_widgets::*;

script_mod! {
    use mod.text.*
    use mod.res.*

    // Namespace object for the shell's shared style constants
    mod.widgets.shell = {}

    // ============================================================================
    // FONT DEFINITIONS (Manrope)
    // ============================================================================

    mod.widgets.shell.FONT_REGULAR = TextStyle{
        font_family: FontFamily{
            latin := FontMember{res: crate_resource("self:resources/Manrope-Regular.ttf") asc: 0.0 desc: 0.0}
        }
    }
    mod.widgets.shell.FONT_MEDIUM = TextStyle{
        font_family: FontFamily{
            latin := FontMember{res: crate_resource("self:resources/Manrope-Medium.ttf") asc: 0.0 desc: 0.0}
        }
    }
    mod.widgets.shell.FONT_SEMIBOLD = TextStyle{
        font_family: FontFamily{
            latin := FontMember{res: crate_resource("self:resources/Manrope-SemiBold.ttf") asc: 0.0 desc: 0.0}
        }
    }
    mod.widgets.shell.FONT_BOLD = TextStyle{
        font_family: FontFamily{
            latin := FontMember{res: crate_resource("self:resources/Manrope-Bold.ttf") asc: 0.0 desc: 0.0}
        }
    }

    // ============================================================================
    // TEXT STYLES
    // ============================================================================

    mod.widgets.shell.TEXT_HEADER = mod.widgets.shell.FONT_SEMIBOLD{
        font_size: 14.0
    }

    mod.widgets.shell.TEXT_LABEL = mod.widgets.shell.FONT_REGULAR{
        font_size: 12.0
    }

    mod.widgets.shell.TEXT_SMALL = mod.widgets.shell.FONT_REGULAR{
        font_size: 11.0
    }
}
