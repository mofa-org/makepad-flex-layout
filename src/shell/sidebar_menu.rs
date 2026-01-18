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

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let normal = vec4(0.973, 0.980, 0.988, 1.0);
                let hover_color = vec4(0.886, 0.910, 0.941, 1.0);
                let selected_color = vec4(0.859, 0.914, 0.988, 1.0);
                let pressed_color = vec4(0.792, 0.835, 0.890, 1.0);
                let color = mix(normal, hover_color, self.hover);
                let color = mix(color, selected_color, self.selected);
                let color = mix(color, pressed_color, self.pressed);
                sdf.box(2.0, 2.0, self.rect_size.x - 4.0, self.rect_size.y - 4.0, 6.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return vec4(0.247, 0.282, 0.333, 1.0);
            }
        }

        draw_icon: {
            fn get_color(self) -> vec4 {
                return vec4(0.392, 0.455, 0.545, 1.0);
            }
        }

        text: "Menu Item"
    }
}
