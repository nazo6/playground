#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;
use ui::{button::Button, init};

struct Root {
    count: i64,
}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .justify_center()
            .items_center()
            .bg(white())
            .child(
                div()
                    .flex()
                    .child(Button::new("minus").label("-").on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.count -= 1;
                            cx.notify();
                        }),
                    ))
                    .child(Button::new("plus").label("+").on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.count += 1;
                            cx.notify();
                        }),
                    )),
            )
            .child(format!("Count: {}", self.count))
    }
}

fn main() -> Result<()> {
    let app = Application::new();

    app.run(move |cx| {
        init(cx);

        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| Root { count: 0 })
        })
        .unwrap();
    });

    Ok(())
}
