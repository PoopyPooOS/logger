use std::{fmt::Display, fs, io, ops::RangeInclusive, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Location {
    pub path: Option<PathBuf>,
    pub text: String,
    pub lines: RangeInclusive<usize>,
    pub section: Option<RangeInclusive<usize>>,
}

impl Location {
    /// # Errors
    /// This function will propagate errors from [`std::fs::read_to_string`]
    pub fn from_path(path: impl Into<PathBuf>, lines: RangeInclusive<usize>) -> io::Result<Self> {
        let path = path.into();
        let text = fs::read_to_string(&path)?;

        Ok(Self {
            path: Some(path),
            text,
            lines,
            section: None,
        })
    }

    pub fn from_text(text: impl Into<String>, lines: RangeInclusive<usize>) -> Self {
        Self {
            path: None,
            text: text.into(),
            lines,
            section: None,
        }
    }

    pub fn lines(mut self, lines: RangeInclusive<usize>) -> Self {
        self.lines = lines;
        self
    }

    pub fn section(mut self, section: RangeInclusive<usize>) -> Self {
        self.section = Some(section);
        self
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}{}",
            if let Some(path) = &self.path {
                path.display().to_string()
            } else {
                "unknown".to_string()
            },
            self.lines.end() + 1,
            if let Some(section) = &self.section {
                format!(":{}", section.end() + 1)
            } else {
                String::new()
            }
        )
    }
}
