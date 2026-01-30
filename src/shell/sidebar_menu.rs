//! Sidebar menu widgets with hover effects and icons

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::live_design::*;

    pub SidebarMenuButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 10, bottom: 10, left: 12, right: 12}
        margin: 0
        align: {x: 0.0, y: 0.5}
        icon_walk: {width: 18, height: 18, margin: {right: 10}}

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: { draw_bg: {pressed: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: { draw_bg: {pressed: 1.0} }
                }
            }
        }

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Light mode colors
                let normal_light = vec4(0.973, 0.980, 0.988, 1.0);
                let hover_light = vec4(0.886, 0.910, 0.941, 1.0);
                let selected_light = vec4(0.859, 0.914, 0.988, 1.0);
                let pressed_light = vec4(0.792, 0.835, 0.890, 1.0);
                // Dark mode colors
                let normal_dark = vec4(0.059, 0.090, 0.165, 1.0);
                let hover_dark = vec4(0.122, 0.161, 0.231, 1.0);
                let selected_dark = vec4(0.165, 0.220, 0.310, 1.0);
                let pressed_dark = vec4(0.200, 0.255, 0.350, 1.0);
                // Mix based on dark_mode
                let normal = mix(normal_light, normal_dark, self.dark_mode);
                let hover_color = mix(hover_light, hover_dark, self.dark_mode);
                let selected_color = mix(selected_light, selected_dark, self.dark_mode);
                let pressed_color = mix(pressed_light, pressed_dark, self.dark_mode);
                // Apply state
                let color = mix(normal, hover_color, self.hover);
                let color = mix(color, selected_color, self.selected);
                let color = mix(color, pressed_color, self.pressed);
                sdf.box(2.0, 2.0, self.rect_size.x - 4.0, self.rect_size.y - 4.0, 6.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                let light = vec4(0.247, 0.282, 0.333, 1.0);
                let dark = vec4(0.886, 0.910, 0.941, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        draw_icon: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                let light = vec4(0.392, 0.455, 0.545, 1.0);
                let dark = vec4(0.580, 0.639, 0.722, 1.0);
                return mix(light, dark, self.dark_mode);
            }
        }

        text: "Menu Item"
    }
}
