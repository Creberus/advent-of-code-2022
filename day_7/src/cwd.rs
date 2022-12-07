pub struct CurrentWorkingDirectory {
    path: String,
}

impl CurrentWorkingDirectory {
    pub fn new() -> Self {
        CurrentWorkingDirectory {
            path: String::with_capacity(256),
        }
    }

    pub fn mv(path: String) -> Self {
        // TODO: Implement move logic
        // 3 cases:
        // "/"
        // ".."
        // "<name>"
        CurrentWorkingDirectory { path }
    }
}
