use test_engine::{
    Event,
    refs::Weak,
    ui::{Button, HasText, Setup, TextField, ViewData, view},
};

#[view]
pub struct CommitView {
    pressed: Event<String>,

    #[init]
    commit_message: TextField,
    push_button:    Button,
}

impl CommitView {
    pub fn on_push_pressed(&self, pressed: impl FnMut(String) + Send + 'static) {
        self.pressed.val(pressed);
    }
}

impl Setup for CommitView {
    fn setup(mut self: Weak<Self>) {
        self.place().all_ver();

        self.commit_message.set_placeholder("Write commit message");
        self.push_button.set_text("Push");
        self.push_button.on_tap(move || {
            let text = self.commit_message.text().to_string();
            self.pressed.trigger(text);
            self.commit_message.clear();
        });
    }
}
