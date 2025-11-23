use gpui::prelude::*;
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
fn gray() -> Hsla {
    rgb(0x6b7280).into()
}
fn blue() -> Hsla {
    rgb(0x3b82f6).into()
}

struct TodoList {
    items: Vec<String>,
    input_text: String,
    focus_handle: FocusHandle,
    is_focused: bool,
}

impl TodoList {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            items: vec![],
            input_text: String::new(),
            focus_handle: cx.focus_handle(),
            is_focused: false,
        }
    }

    fn handle_key(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        match event.keystroke.key.as_ref() {
            "enter" if !self.input_text.is_empty() => {
                self.items.push(self.input_text.clone());
                self.input_text.clear();
            }
            "backspace" => {
                self.input_text.pop();
            }
            "space" => {
                self.input_text.push(' ');
            }
            key if key.len() == 1 && !event.keystroke.modifiers.control => {
                if let Some(ch) = key.chars().next() {
                    self.input_text.push(ch);
                }
            }
            _ => return,
        }
        cx.notify();
    }

    fn delete_item(&mut self, index: usize, cx: &mut Context<Self>) {
        self.items.remove(index);
        cx.notify();
    }

    fn focus_input(
        &mut self,
        _event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_focused = true;
        window.focus(&self.focus_handle);
        cx.notify();
    }

    fn render_header(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(div().text_xl().child("Todo List"))
    }

    fn render_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let show_placeholder = self.input_text.is_empty() && !self.is_focused;
        let display_text = if show_placeholder {
            "Type and press Enter...".to_string()
        } else if self.is_focused {
            format!("{}|", self.input_text)
        } else {
            self.input_text.clone()
        };

        div()
            .w_full()
            .px_3()
            .py_2()
            .border_1()
            .border_color(blue())
            .rounded_md()
            .cursor_text()
            .text_color(if show_placeholder { gray() } else { white() })
            .child(display_text)
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::focus_input))
    }

    fn render_empty_state(&self) -> impl IntoElement {
        div()
            .text_color(gray())
            .child("No todos yet. Type something and press Enter!")
    }

    fn render_todo_item(
        &self,
        text: &str,
        on_delete: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .p_2()
            .border_1()
            .border_color(gray())
            .rounded_md()
            .child(div().flex_1().child(text.to_string()))
            .child(
                div()
                    .px_2()
                    .py_1()
                    .bg(red())
                    .rounded_md()
                    .cursor_pointer()
                    .child("Delete")
                    .on_mouse_down(MouseButton::Left, on_delete),
            )
    }
}

impl Render for TodoList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.items.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .p_4()
            .gap_4()
            .text_color(white())
            .child(self.render_header())
            .child(self.render_input(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(items.iter().enumerate().map(|(i, item)| {
                        let on_delete = cx.listener(move |this, _event, _window, cx| {
                            this.delete_item(i, cx);
                        });
                        self.render_todo_item(item, on_delete)
                    })),
            )
            .when(items.is_empty(), |this| {
                this.child(self.render_empty_state())
            })
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(TodoList::new)
        })
        .unwrap();
    });
}
