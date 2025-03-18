use test_engine::ui::{view, Button};

#[view]
pub struct Main {
    #[init]
    custom: Button,
}
