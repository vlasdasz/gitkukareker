pub struct CommitHistory {
    pub hash:    String,
    pub author:  String,
    pub email:   String,
    pub message: String,
}

impl From<git2::Commit<'_>> for CommitHistory {
    fn from(commit: git2::Commit) -> Self {
        Self {
            hash:    commit.id().to_string(),
            author:  commit.author().name().unwrap_or("IS NOT VALID UTF-8 STRING").to_owned(),
            email:   commit.author().email().unwrap_or("IS NOT VALID UTF-8 STRING").to_owned(),
            message: commit.message().unwrap_or("IS NOT VALID UTF-8 STRING").to_owned(),
        }
    }
}
