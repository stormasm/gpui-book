use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, Window, WindowOptions,
    deferred, div,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(deferred(div()).with_priority(10))
    }
}

fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| {
            app.new(|_cx| RootView)
        })
        .unwrap();
    });
}
