use crate::location::Section;
use colored::{Color, Colorize};
use std::ops::RangeInclusive;

pub fn thing(
    input: impl Into<String>,
    section: &Section,
    style: impl Fn(&str) -> String,
) -> String {
    let input: String = input.into();

    let line_start = *section.lines().start();
    let line_end = *section.lines().end();
    let col_start = *section.cols().start();
    let col_end = *section.cols().end();

    let lines: Vec<&str> = input.lines().collect();

    let mut highlighted = String::new();

    for (i, line) in lines.iter().enumerate() {
        if i < line_start || i > line_end {
            highlighted.push_str(line);
        } else {
            let start_col = if i == line_start { col_start } else { 0 };
            let end_col = if i == line_end { col_end } else { line.len() };

            let pre_highlight = &line[..start_col.min(line.len())];
            let to_highlight = &line[start_col.min(line.len())..end_col.min(line.len())];
            let post_highlight = &line[end_col.min(line.len())..];

            highlighted.push_str(pre_highlight);
            highlighted.push_str(&style(to_highlight));
            highlighted.push_str(post_highlight);
        }

        if i < lines.len() - 1 {
            highlighted.push('\n');
        }
    }

    highlighted
}

pub fn highlight(input: impl Into<String>, section: &Section, color: Color) -> String {
    thing(input, section, |s| s.color(color).bold().to_string())
}

pub fn bold_highlight(input: impl Into<String>, section: &Section) -> String {
    thing(input, section, |s| s.bold().to_string())
}

pub fn range_contains(range: &RangeInclusive<usize>, idx: usize) -> bool {
    range.start() <= &idx && range.end() >= &idx
}

pub fn remove_excess_tabs(input: impl Into<String>) -> String {
    let input: String = input.into();
    let min_whitespace = input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| {
            line.chars()
                .take_while(|&c| c == ' ' || c == '\t')
                .collect::<String>()
        })
        .min_by_key(String::len)
        .unwrap_or_default();

    let min_len = min_whitespace.len();

    // Remove the minimum amount of leading whitespace from each line
    input
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new() // Keep empty lines as they are
            } else {
                line.chars().skip(min_len).collect::<String>()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
