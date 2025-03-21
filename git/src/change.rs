use git2::StatusEntry;

use crate::ChangeStatus;

#[derive(Debug)]
pub struct Change {
    pub status: ChangeStatus,
    pub file:   String,
    pub staged: bool,
}

impl From<StatusEntry<'_>> for Change {
    fn from(value: StatusEntry) -> Self {
        Self {
            status: value.status().into(),
            file:   value.path().unwrap_or("PATH IS NOT VALID UTF-8 STRING").to_owned(),
            staged: check_staged(&value),
        }
    }
}

fn check_staged(entry: &StatusEntry) -> bool {
    entry.status().is_index_new()
        || entry.status().is_index_deleted()
        || entry.status().is_index_modified()
        || entry.status().is_index_renamed()
        || entry.status().is_index_typechange()
}
