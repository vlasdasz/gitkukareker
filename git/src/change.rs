use git2::StatusEntry;

use crate::ChangeStatus;

#[derive(Debug)]
pub struct Change {
    pub status: ChangeStatus,
    pub file:   String,
}

impl From<StatusEntry<'_>> for Change {
    fn from(value: StatusEntry) -> Self {
        Self {
            status: value.status().into(),
            file:   value.path().unwrap_or("PATH IS NOT VALID UTF-8 STRING").to_owned(),
        }
    }
}
