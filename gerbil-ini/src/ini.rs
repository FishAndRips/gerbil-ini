use alloc::borrow::ToOwned;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter};

const COMMENT_CHARS: &[char] = &[';', '#'];

/// Describes a method for parsing ini files.
///
/// The ini format isn't a universally agreed upon standard, and it can have different rules depending on the program
/// it is written for.
///
/// Some ini files will successfully parse on some programs, but not so on others.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IniMode {
    /// Simple X=Y, where everything is passed through.
    ///
    /// There are some restrictions to this:
    /// * Keys, values, and sections cannot be multi-line
    /// * Keys cannot contain `=` characters
    /// * Keys cannot start with `;`, `#`, or `[`
    /// * Comments must exist in their own lines with no whitespace before the comment delimiter
    Simple,

    /// Same as `Simple`, but trim whitespace for keys and values.
    ///
    /// For example, `key = value` has the same meaning as `key=value`.
    ///
    /// This adds two additional restrictions:
    /// * Keys cannot end with whitespace
    /// * Values cannot begin with whitespace
    SimpleTrimmed
}

/// Ini parser.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Ini {
    sections: BTreeMap<String, IniSection>
}

/// Section for an ini.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct IniSection {
    values: BTreeMap<String, String>
}

impl Ini {
    /// Parse the ini.
    pub fn parse(string: &str, config: IniMode) -> Result<Self, IniParsingError> {
        match config {
            IniMode::Simple => Self::parse_simple(string, config),
            IniMode::SimpleTrimmed => Self::parse_simple(string, config),
        }
    }

    /// Get the section.
    ///
    /// Returns `None` if the section does not exist in the ini.
    pub fn get_section(&self, section: &str) -> Option<&IniSection> {
        self.sections.get(section)
    }

    fn parse_simple(string: &str, config: IniMode) -> Result<Self, IniParsingError> {
        let mut ini = Ini::default();

        let mut lines = string.lines().enumerate();
        let mut section = None;

        while let Some((line_number, line)) = lines.next() {
            if line.chars().next().iter().any(|i| COMMENT_CHARS.contains(i)) || line.is_empty() || line.chars().all(|c| c.is_whitespace()) {
                continue
            }

            if line.starts_with('[') {
                let end = line.find(']').ok_or(IniParsingError::BrokenSectionTitle { line_number })?;
                let title = line[1..end].to_owned();
                if ini.sections.contains_key(&title) {
                    return Err(IniParsingError::DuplicateSection { line_number, section: title })
                }
                section = Some(title.clone());
                ini.sections.insert(title, Default::default());
                continue
            }

            let Some(section) = section.as_ref() else {
                return Err(IniParsingError::ExpectedSectionTitle { line_number })
            };

            let l = line.find('=').ok_or(IniParsingError::MissingEquals { line_number })?;
            let (key_str, value_eq) = line.split_at(l);
            let value_str = &value_eq[1..];

            let key: String;
            let value: String;

            match config {
                IniMode::Simple => {
                    key = key_str.to_owned();
                    value = value_str.to_owned();
                }
                IniMode::SimpleTrimmed => {
                    key = key_str.trim_end().to_owned();
                    value = value_str.trim_start().to_owned();
                }
            }

            let s = ini.sections.get_mut(section).unwrap();
            if s.values.contains_key(&key) {
                return Err(IniParsingError::DuplicateSectionKey { line_number, section: section.to_string(), key })
            }
            s.values.insert(key, value);
        }

        Ok(ini)
    }
}

impl IniSection {
    /// Get the value for a key.
    ///
    /// Returns `None` if the key is not present.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

/// An error generated by the ini parser.
#[derive(Clone, Debug, PartialEq)]
pub enum IniParsingError {
    MissingEquals { line_number: usize },
    ExpectedSectionTitle { line_number: usize },
    BrokenSectionTitle { line_number: usize },
    DuplicateSection { line_number: usize, section: String },
    DuplicateSectionKey { line_number: usize, section: String, key: String },
}

impl Display for IniParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingEquals { line_number } => f.write_fmt(format_args!("{line_number}: Missing an `=` to separate the key and value")),
            Self::ExpectedSectionTitle { line_number } => f.write_fmt(format_args!("{line_number}: Expected a section title")),
            Self::BrokenSectionTitle { line_number } => f.write_fmt(format_args!("{line_number}: Expected a `]` to close a `[`")),
            Self::DuplicateSection { line_number, section } => f.write_fmt(format_args!("{line_number}: Duplicate section `{section}`")),
            Self::DuplicateSectionKey { line_number, section, key } => f.write_fmt(format_args!("{line_number}: Duplicate key `{key}` in section `{section}`"))
        }
    }
}

#[cfg(test)]
mod test;
