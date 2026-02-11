use std::{
    cell::OnceCell,
    path::{Path, PathBuf},
};

use regex::Regex;

pub struct Glob {
    pattern: String,
    regex: OnceCell<Regex>,
}

impl Glob {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            regex: OnceCell::new(),
        }
    }

    pub fn matches<'gl>(&'gl self) -> anyhow::Result<Vec<GlobMatch<'gl>>> {
        let mut matches: Vec<GlobMatch<'gl>> = Vec::new();

        for path in glob::glob(&self.pattern)?.flatten() {
            matches.push(GlobMatch { glob: self, path });
        }

        Ok(matches)
    }

    fn get_regex(&self) -> &Regex {
        self.regex.get_or_init(|| {
            let pattern = self.pattern.replace("**", "(.+?)");
            let pattern = format!("^{}$", pattern);
            Regex::new(&pattern).unwrap()
        })
    }
}

pub struct GlobMatch<'gl> {
    glob: &'gl Glob,
    pub path: PathBuf,
}

impl<'gl> GlobMatch<'gl> {
    pub fn file_name_str(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    pub fn parent(&self) -> Option<&Path> {
        self.path.parent()
    }

    pub fn components(&'gl self) -> Option<Vec<String>> {
        let captures = self.glob.get_regex().captures(self.path.to_str()?)?;
        let components: Vec<String> = captures
            .iter()
            .skip(1)
            .flatten()
            .flat_map(|m| m.as_str().split('/'))
            .map(|s| s.to_string())
            .collect();

        (!components.is_empty()).then_some(components)
    }
}
