use gpui::{Application, Global, ReadGlobal};

pub struct SomeState {
    some_value: bool,
}

impl Global for SomeState {}

fn main() {
    Application::new().run(|app| {
        app.set_global::<SomeState>(SomeState { some_value: true });

        let some_state = app.global::<SomeState>();

        // OR

        let some_state = SomeState::global(app);
    });
}
