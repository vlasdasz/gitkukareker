use test_engine::ui::ToLabel;

#[derive(Debug, Copy, Clone)]
pub enum ChangeStatus {
    New,
    Changed,
    Deleted,
}

impl ToLabel for ChangeStatus {
    fn to_label(&self) -> String {
        match self {
            ChangeStatus::New => "+",
            ChangeStatus::Changed => "E",
            ChangeStatus::Deleted => "-",
        }
        .to_string()
    }
}

impl From<git2::Status> for ChangeStatus {
    fn from(value: git2::Status) -> Self {
        if value.is_wt_new() {
            return Self::New;
        }

        if value.is_wt_modified() {
            return Self::Changed;
        }

        if value.is_wt_deleted() {
            return Self::Deleted;
        }

        todo!()
    }
}
