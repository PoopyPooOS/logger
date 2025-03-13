use std::{
    fmt::{self, Debug, Display},
    fs, io,
    ops::RangeInclusive,
    path::PathBuf,
};

#[derive(Clone)]
pub struct Section(pub RangeInclusive<(usize, usize)>);

impl Section {
    pub fn new(lines: RangeInclusive<usize>, cols: RangeInclusive<usize>) -> Self {
        Self((*lines.start(), *cols.start())..=(*lines.end(), *cols.end()))
    }

    pub fn merge_start_end(start: &Self, end: &Self) -> Self {
        Self(
            (*start.lines().start(), *start.cols().start())
                ..=(*end.lines().end(), *end.cols().end()),
        )
    }

    pub fn full() -> Self {
        Self::new(usize::MIN..=usize::MAX, usize::MIN..=usize::MAX)
    }

    pub fn lines(&self) -> RangeInclusive<usize> {
        self.0.start().0..=self.0.end().0
    }

    pub fn cols(&self) -> RangeInclusive<usize> {
        self.0.start().1..=self.0.end().1
    }

    /// Sets the line range of the section, preserving the column ranges.
    pub fn set_lines(&mut self, lines: RangeInclusive<usize>) {
        let col_start = self.0.start().1;
        let col_end = self.0.end().1;

        self.0 = (*lines.start(), col_start)..=(*lines.end(), col_end);
    }

    /// Sets the column range of the section for the start and end line.
    /// This assumes the section does not span multiple lines.
    pub fn set_cols(&mut self, cols: RangeInclusive<usize>) {
        let line_start = self.0.start().0;
        let line_end = self.0.end().0;

        self.0 = (line_start, *cols.start())..=(line_end, *cols.end());
    }
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.lines() == other.lines() && self.cols() == other.cols()
    }
}

impl Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "[{}, {}] - [{}, {}]",
            self.lines().start(),
            self.cols().start(),
            self.lines().end(),
            self.cols().end()
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub path: Option<PathBuf>,
    pub text: String,
    pub section: Option<Section>,
}

impl Location {
    /// # Errors
    /// This function will propagate errors from [`std::fs::read_to_string`]
    pub fn from_path(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        let text = fs::read_to_string(&path)?;

        Ok(Self {
            path: Some(path),
            text,
            section: None,
        })
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        Self {
            path: None,
            text: text.into(),
            section: None,
        }
    }

    pub fn section(mut self, section: Section) -> Self {
        self.section = Some(section);
        self
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(path) = &self.path {
                path.display().to_string()
            } else {
                "unknown".to_string()
            },
            if let Some(section) = &self.section {
                format!(
                    ":{}:{}",
                    section.lines().end() + 1,
                    section.cols().end() + 1
                )
            } else {
                String::new()
            }
        )
    }
}
