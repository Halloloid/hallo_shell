use std::collections::{HashMap, HashSet};

pub struct ShellCompleter {
    pub completeion: HashMap<String, String>,
}

impl rustyline::completion::Completer for ShellCompleter {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates = Vec::new();

        let current_word = &line[..pos];

        if !current_word.contains(' ') {
            if "echo".starts_with(current_word) && !current_word.is_empty() {
                candidates.push("echo ".to_string());
            } else if "exit".starts_with(current_word) && !current_word.is_empty() {
                candidates.push("exit ".to_string());
            } else {
                let executables = get_all_executabels();
                for executable in executables {
                    if executable.starts_with(current_word) && !current_word.is_empty() {
                        candidates.push(format!("{} ", executable));
                    }
                }
            }

            candidates.sort();
            Ok((0, candidates))
        } else {
            if let Some(command) = current_word.split_whitespace().next() {
                if current_word.ends_with(' ') || current_word.contains(' ') {
                    if let Some(script_path) = self.completeion.get(command) {
                        let words: Vec<&str> = current_word.split_whitespace().collect();

                        let current_arg = if current_word.ends_with(' ') {
                            ""
                        } else {
                            words.last().unwrap_or(&"")
                        };

                        let previus_arg = if current_word.ends_with(' ') {
                            words.last().unwrap_or(&"")
                        } else {
                            if words.len() >= 2 {
                                words[words.len() - 2]
                            } else {
                                ""
                            }
                        };
                        if let Ok(output) = std::process::Command::new(script_path)
                            .arg(command)
                            .arg(current_arg)
                            .arg(previus_arg)
                            .output()
                        {
                            let stdout_str = String::from_utf8_lossy(&output.stdout);
                            if let Some(line) = stdout_str.lines().next() {
                                candidates.push(format!("{} ", line.trim()));
                                return Ok((current_word.len()-current_arg.len(), candidates));
                            }
                        }
                    }
                }
            }
            let last_space_idx = current_word.rfind(' ').unwrap();
            let prefix = &current_word[last_space_idx + 1..];

            if prefix.contains('/') {
                let idx = prefix.rfind('/').unwrap();
                let path = &prefix[..idx];
                let prefix = &prefix[idx + 1..];
                let all_files_in_path = get_all_root_file_and_dirs(path);

                for file in all_files_in_path {
                    if file.starts_with(prefix) {
                        candidates.push(file);
                    }
                }

                candidates.sort();

                return Ok((last_space_idx + 1 + idx + 1, candidates));
            }

            let all_files = get_all_root_file_and_dirs(".");

            for file in all_files {
                if file.starts_with(prefix) {
                    candidates.push(file);
                }
            }

            candidates.sort();

            Ok((last_space_idx + 1, candidates))
        }
    }
}

use is_executable::is_executable;
use std::env;
use std::fs;

fn get_all_executabels() -> HashSet<String> {
    let mut set = HashSet::new();

    if let Ok(spliter) = env::var("PATH") {
        for path in env::split_paths(&spliter) {
            if let Ok(entires) = fs::read_dir(path) {
                for entry in entires.flatten() {
                    let path = entry.path();
                    if is_executable(&path) {
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            set.insert(file_name.to_string());
                        }
                    }
                }
            }
        }
    }
    set
}

fn get_all_root_file_and_dirs(path: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    if let Ok(entires) = fs::read_dir(path) {
        for entry in entires.flatten() {
            let file = entry.file_type().unwrap();
            let is_dir = file.is_dir();
            if let Ok(name) = entry.file_name().into_string() {
                if is_dir {
                    v.push(format!("{name}/"));
                } else {
                    v.push(format!("{name} "));
                }
            }
        }
    }
    v
}
