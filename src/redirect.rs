use std::{fs, path::Path};

pub fn store_in_file(output: String, file_path: &str, append: bool) {
    let clen_path = file_path.trim();

    if let Some(parent) = Path::new(clen_path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }

    if append {
        let mut content = fs::read_to_string(file_path).unwrap_or_default();
        if content.ends_with('\n') || content.is_empty() {
            content.push_str(&output);
        } else {
            content.push('\n');
            content.push_str(&output);
        }

        fs::write(file_path, content).expect("unabel to write");
    } else {
        fs::write(file_path, output).expect("Unabel to write");
    }
}

pub fn std_store(
    store_err: bool,
    append: bool,
    append_err: bool,
    stdout_str: String,
    stderr_str: String,
    file: &str,
) {
    let store = if store_err || append_err {
        stderr_str.clone()
    } else {
        stdout_str.clone()
    };

    store_in_file(store, file.trim(), append || append_err);

    if store_err || append_err {
        if !stdout_str.is_empty() {
            if stdout_str.ends_with('\n') {
                eprint!("{}", stdout_str);
            } else {
                eprintln!("{}", stdout_str);
            }
        }
    } else {
        if !stderr_str.is_empty() {
            if stderr_str.ends_with('\n') {
                eprint!("{}", stderr_str);
            } else {
                eprintln!("{}", stderr_str);
            }
        }
    }
}
