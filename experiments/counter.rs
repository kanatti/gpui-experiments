use gpui::*;

struct Counter {
    count: usize,
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
            .child(
                div()
                    .text_color(rgb(0xffffff))
                    .text_size(px(24.))
                    .child(format!("Count: {}", self.count))
            )
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x3b82f6))
                    .rounded_md()
                    .child("Increment")
                    .on_mouse_down(MouseButton::Left, cx.listener(|view, _event, _window, cx| {
                        view.count += 1;
                        cx.notify();
                    }))
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| Counter { count: 0 })
        }).unwrap();
    });
}
