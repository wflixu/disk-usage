use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize)]
pub struct CRes<T: Serialize> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> CRes<T> {
    pub fn success(data: Option<T>) -> Self {
        CRes {
            code: 200,
            msg: String::from(""),
            data,
        }
    }

    pub fn fail(code: u16, msg: String) -> Self {
        CRes {
            code,
            msg,
            data: None,
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> CRes<String> {
    CRes::success(Some(format!(
        "Hello, {}! You've been greeted from Rust!",
        name
    )))
}

#[tauri::command]
fn open_dir(path: &str) -> CRes<()> {
    let result = if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(path)
            .spawn()
            .and_then(|mut child| child.wait())
    } else if cfg!(target_os = "windows") {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .and_then(|mut child| child.wait())
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .and_then(|mut child| child.wait())
    } else {
        return CRes::fail(1, String::from("Unsupported OS"));
    };

    match result {
        Ok(_) => CRes::success(None),
        Err(e) => CRes::fail(1, e.to_string()),
    }
}

#[derive(Serialize, Deserialize)]
struct DirOrFile {
    name: String,
    is_dir: bool,
    count: u64,
}

#[tauri::command]
fn dir_stat(path: &str) -> CRes<Vec<DirOrFile>> {
    use std::fs;
    use std::path::Path;

    fn get_dir_stat(path: &Path) -> Vec<DirOrFile> {
        let mut entries = vec![];
        if let Ok(read_dir) = fs::read_dir(path) {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = fs::metadata(&path).unwrap();
                    let is_dir = metadata.is_dir();
                    let count = if is_dir {
                        get_dir_stat(&path).iter().map(|e| e.count).sum()
                    } else {
                        metadata.len()
                    };
                    entries.push(DirOrFile {
                        name: entry.file_name().into_string().unwrap(),
                        is_dir,
                        count,
                    });
                }
            }
        }
        entries
    }

    let path = Path::new(path);
    if !path.exists() {
        return CRes::fail(1, String::from("Path does not exist"));
    }

    let stats = get_dir_stat(path);
    CRes::success(Some(stats))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_dir, dir_stat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
