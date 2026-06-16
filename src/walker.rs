use std::path::{Path, PathBuf};
use std::sync::Mutex;

use anyhow::{Context, Result};
use ignore::{DirEntry, WalkBuilder, WalkState};

const TARGET_EXTENSIONS: &[&str] = &[
    "js", "jsx", "ts", "tsx", "py", "rs", "go", "java", "cs", "swift", "kt", "md", "json", "yaml",
    "yml", "toml",
];

pub fn collect_code_files(root: &Path) -> Result<Vec<PathBuf>> {
    let files = Mutex::new(Vec::new());
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .parents(true)
        .ignore(true)
        .add_custom_ignore_filename(".cursorignore")
        .threads(0);

    builder.build_parallel().run(|| {
        let files = &files;
        Box::new(move |result| {
            let entry = match result {
                Ok(entry) => entry,
                Err(_) => return WalkState::Continue,
            };

            if is_target_file(&entry) {
                if let Some(path) = entry.path().to_str() {
                    if path.contains("/.git/") {
                        return WalkState::Continue;
                    }
                }

                if let Ok(mut guard) = files.lock() {
                    guard.push(entry.into_path());
                }
            }

            WalkState::Continue
        })
    });

    let mut files = files.into_inner().context("file walker lock poisoned")?;
    files.sort_unstable();
    Ok(files)
}

fn is_target_file(entry: &DirEntry) -> bool {
    entry
        .file_type()
        .is_some_and(|file_type| file_type.is_file())
        && entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .is_some_and(|extension| TARGET_EXTENSIONS.contains(&extension.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn collects_supported_extensions() {
        let root = temp_dir();
        write_file(&root, "main.rs", "");
        write_file(&root, "server.go", "");
        write_file(&root, "App.java", "");
        write_file(&root, "Program.cs", "");
        write_file(&root, "View.swift", "");
        write_file(&root, "Service.kt", "");
        write_file(&root, "README.md", "");
        write_file(&root, "package.json", "{}");
        write_file(&root, "config.yaml", "");
        write_file(&root, "config.yml", "");
        write_file(&root, "Cargo.toml", "");
        write_file(&root, "image.png", "");

        let files = collect_code_files(&root).unwrap();
        let names = relative_names(&root, files);

        assert!(names.contains(&"main.rs".to_owned()));
        assert!(names.contains(&"server.go".to_owned()));
        assert!(names.contains(&"App.java".to_owned()));
        assert!(names.contains(&"Program.cs".to_owned()));
        assert!(names.contains(&"View.swift".to_owned()));
        assert!(names.contains(&"Service.kt".to_owned()));
        assert!(names.contains(&"README.md".to_owned()));
        assert!(names.contains(&"package.json".to_owned()));
        assert!(names.contains(&"config.yaml".to_owned()));
        assert!(names.contains(&"config.yml".to_owned()));
        assert!(names.contains(&"Cargo.toml".to_owned()));
        assert!(!names.contains(&"image.png".to_owned()));
    }

    #[test]
    fn respects_gitignore_and_cursorignore() {
        let root = temp_dir();
        fs::create_dir(root.join(".git")).unwrap();
        write_file(&root, ".gitignore", "ignored.rs\n");
        write_file(&root, ".cursorignore", "cursor_ignored.ts\n");
        write_file(&root, "visible.rs", "");
        write_file(&root, "ignored.rs", "");
        write_file(&root, "cursor_ignored.ts", "");

        let files = collect_code_files(&root).unwrap();
        let names = relative_names(&root, files);

        assert!(names.contains(&"visible.rs".to_owned()));
        assert!(!names.contains(&"ignored.rs".to_owned()));
        assert!(!names.contains(&"cursor_ignored.ts".to_owned()));
    }

    fn temp_dir() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = env::temp_dir().join(format!("contextshrink-walker-{unique}"));
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn write_file(root: &Path, name: &str, contents: &str) {
        fs::write(root.join(name), contents).unwrap();
    }

    fn relative_names(root: &Path, files: Vec<PathBuf>) -> Vec<String> {
        files
            .into_iter()
            .map(|path| {
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect()
    }
}
