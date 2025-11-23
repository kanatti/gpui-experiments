use gpui::*;

fn white() -> Hsla {
    rgb(0xffffff).into()
}
fn green() -> Hsla {
    rgb(0x22c55e).into()
}
fn red() -> Hsla {
    rgb(0xef4444).into()
}

struct Counter {
    count: usize,
}

impl Counter {
    fn render_count(&self) -> impl IntoElement {
        div()
            .text_color(white())
            .text_size(px(24.))
            .child(format!("Count: {}", self.count))
    }

    fn render_button(
        &self,
        label: impl Into<SharedString>,
        color: Hsla,
        cx: &mut Context<Self>,
        on_click: impl Fn(&mut Counter, &mut Context<Counter>) + 'static,
    ) -> impl IntoElement {
        div()
            .px_4()
            .py_2()
            .bg(color)
            .rounded_md()
            .child(label.into())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |view, _event, _window, cx| {
                    on_click(view, cx);
                }),
            )
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(self.render_count())
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(self.render_button("Decrement", red(), cx, |view, cx| {
                        if view.count > 0 {
                            view.count -= 1;
                        }
                        cx.notify();
                    }))
                    .child(self.render_button("Increment", green(), cx, |view, cx| {
                        view.count += 1;
                        cx.notify();
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| Counter { count: 0 })
        })
        .unwrap();
    });
}
