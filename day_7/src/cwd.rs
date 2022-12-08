use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug)]
pub struct CWDError {}

impl CWDError {
    pub fn new() -> Self {
        CWDError {}
    }
}

impl Display for CWDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "..")
    }
}

impl Error for CWDError {
    fn description(&self) -> &str {
        return "Error: Going to directory failed.";
    }
}

pub struct CurrentWorkingDirectory {
    path: PathBuf,
}

impl Display for CurrentWorkingDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.as_path().to_str().unwrap())
    }
}

impl CurrentWorkingDirectory {
    pub fn new() -> Self {
        CurrentWorkingDirectory {
            path: PathBuf::new(),
        }
    }

    pub fn mv(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        match path {
            "/" => self.path = PathBuf::from("/"),
            ".." => {
                if !self.path.pop() {
                    return Err(Box::new(CWDError::new()));
                }
            }
            name => self.path.push(name),
        }

        Ok(())
    }

    pub fn get(&self) -> &str {
        self.path.to_str().unwrap()
    }
}
