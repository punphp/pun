//! PHPStan's `relativePathHelper` (FuzzyRelativePathHelper over ParentDirectoryRelativePathHelper): how the names
//! of anonymous classes (`class@anonymous/<path>:<line>`) print a file.

use std::path::Path;

/// FuzzyRelativePathHelper: paths below the analysed paths' common directory are printed relative to it, others as
/// ParentDirectoryRelativePathHelper prints them relative to the working directory.
#[derive(Clone, Debug, Default)]
pub struct RelativePath {
    cwd: String,
    trim: Option<String>,
}

/// `$trimBeginning`: the leading `/` and the rest.
fn trim_beginning(path: &str) -> (&str, &str) {
    match path.strip_prefix('/') {
        Some(rest) => ("/", rest),
        None => ("", path),
    }
}

impl RelativePath {
    /// FuzzyRelativePathHelper::__construct() with `currentWorkingDirectory` and `analysedPaths`.
    pub fn new(cwd: &str, analysed_paths: &[String]) -> RelativePath {
        let mut beginning = "";
        let mut to_trim: Option<Vec<&str>> = None;
        if !matches!(cwd, "" | "/") {
            let (b, rest) = trim_beginning(cwd);
            beginning = b;
            to_trim = Some(rest.split('/').collect());
        }
        'paths: for (number, path) in analysed_paths.iter().enumerate() {
            let (path_beginning, path) = trim_beginning(path);
            let parts: Vec<&str> = path.split('/').collect();
            let mut temp = Vec::new();
            for (i, part) in parts.iter().enumerate() {
                if i == parts.len() - 1 && part.ends_with(".php") {
                    continue;
                }
                match to_trim.as_ref().and_then(|t| t.get(i)) {
                    None if number != 0 => {
                        to_trim = Some(temp);
                        continue 'paths;
                    }
                    Some(p) if p != part => {
                        to_trim = Some(temp);
                        continue 'paths;
                    }
                    _ => {}
                }
                temp.push(*part);
            }
            beginning = path_beginning;
            to_trim = Some(temp);
        }
        let trim = to_trim.filter(|t| !t.is_empty()).map(|t| {
            let trim = format!("{beginning}{}", t.join("/"));
            std::fs::canonicalize(&trim).map_or(trim, |p| p.to_string_lossy().into_owned())
        });
        RelativePath {
            cwd: cwd.to_string(),
            trim,
        }
    }

    /// The working directory and the analysed paths of a run.
    pub fn of(analysed_paths: &[String]) -> RelativePath {
        let cwd = std::env::current_dir()
            .map(|d| d.to_string_lossy().into_owned())
            .unwrap_or_default();
        RelativePath::new(&cwd, analysed_paths)
    }

    /// `getRelativePath()`.
    pub fn get(&self, filename: &str) -> String {
        if let Some(rest) = self.trim.as_deref().and_then(|t| filename.strip_prefix(t)) {
            return rest.trim_start_matches('/').to_string();
        }
        self.parent_directory(filename)
    }

    /// ParentDirectoryRelativePathHelper::getRelativePath() from the working directory.
    fn parent_directory(&self, filename: &str) -> String {
        let filename = filename
            .find("://")
            .map_or(filename, |at| &filename[at + 3..]);
        let parent: Vec<&str> = self.cwd.trim_matches('/').split('/').collect();
        let parts: Vec<&str> = filename.trim_matches('/').split('/').collect();
        let mut i = 0;
        while i < parts.len() && i < parent.len() && parent[..=i] == parts[..=i] {
            i += 1;
        }
        if i == 0 {
            return filename.to_string();
        }
        let mut out = vec![".."; parent.len() - i];
        out.extend(&parts[i..]);
        out.join("/")
    }
}

/// An absolute path as PHPStan's FileHelper prints it.
pub(crate) fn absolute(path: &Path) -> String {
    std::path::absolute(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned()
}
