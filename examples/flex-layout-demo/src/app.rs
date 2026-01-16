//! Flex Layout Demo Application
//!
//! Demonstrates the makepad-app-shell library with a resizable studio layout.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Import shell components from makepad_app_shell crate
    use makepad_app_shell::shell::layout::ShellLayout;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    title: "Flex Layout Studio Demo"
                    inner_size: vec2(1400, 900)
                }
                body = <ShellLayout> {}
            }
        }
    }
}

// ============================================================================
// APP IMPLEMENTATION
// ============================================================================

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::makepad_app_shell::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
