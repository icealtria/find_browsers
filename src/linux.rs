use crate::Browser;
use dirs::home_dir;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const HTTP_HANDLER: &str = "x-scheme-handler/http";

const SYSTEM_DESKTOP_PATH: &str = "/usr/share/applications/";
const SYSTEM_MIMEINFO_PATH: &str = "/usr/share/applications/mimeinfo.cache";
const LOCAL_DESKTOP_PATH: &str = ".local/share/applications/";
const LOCAL_MIMEINFO_PATH: &str = ".local/share/applications/mimeinfo.cache";
const FLATPAK_DESKTOP_PATH: &str = "/var/lib/flatpak/exports/share/applications";
const FLATPAK_MIMEINFO_PATH: &str = "/var/lib/flatpak/exports/share/applications/mimeinfo.cache";

pub fn get_browsers() -> Result<Vec<Browser>, Box<dyn std::error::Error>> {
    let browser_names = find_installed_browsers()?;
    Ok(resolve_browser_exec_paths(&browser_names))
}

fn sanitize_exec_path(exec_path: String) -> String {
    exec_path.replace("%u", "").replace("%U", "")
}

fn find_installed_browsers() -> Result<Vec<String>, std::io::Error> {
    let mut installed_browsers = Vec::new();
    for mimeinfo_path in get_mimeinfo_paths() {
        if mimeinfo_path.exists() {
            let file_content = fs::read_to_string(&mimeinfo_path)?;
            extract_browsers_from_mimeinfo(&file_content, &mut installed_browsers);
        }
    }
    Ok(installed_browsers)
}

fn get_mimeinfo_paths() -> Vec<PathBuf> {
    let mut paths = vec![
        PathBuf::from(SYSTEM_MIMEINFO_PATH),
        PathBuf::from(FLATPAK_MIMEINFO_PATH),
    ];
    if let Some(home) = home_dir() {
        let local_mimeinfo = home.join(LOCAL_MIMEINFO_PATH);
        paths.push(local_mimeinfo);
    }
    paths
}

fn extract_browsers_from_mimeinfo(content: &str, installed_browsers: &mut Vec<String>) {
    content
        .lines()
        .find(|line| line.starts_with(HTTP_HANDLER))
        .and_then(|line| {
            line.split('=').nth(1).map(|browsers| {
                installed_browsers.extend(browsers.split(';').map(str::to_string));
            })
        });
}

fn resolve_browser_exec_paths(browser_names: &[String]) -> Vec<Browser> {
    let mut browsers_map = HashMap::new();
    for desktop_path in get_desktop_paths() {
        if desktop_path.exists() {
            let entries = fs::read_dir(desktop_path).unwrap_or_else(|_| fs::read_dir("/").unwrap());
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if let Some(file_name) = entry_path.file_name().and_then(|s| s.to_str()) {
                    if browser_names.contains(&file_name.to_string()) {
                        if let Some(browser) = parse_desktop_entry(&entry_path) {
                            browsers_map.insert(browser.name.clone(), browser);
                        }
                    }
                }
            }
        }
    }
    browsers_map.into_values().collect()
}

fn get_desktop_paths() -> Vec<PathBuf> {
    let mut paths = vec![
        PathBuf::from(SYSTEM_DESKTOP_PATH),
        PathBuf::from(FLATPAK_DESKTOP_PATH),
    ];
    if let Some(home) = home_dir() {
        paths.push(home.join(LOCAL_DESKTOP_PATH));
    }
    paths
}

fn resolve_executable_path(exec: String) -> PathBuf {
    let exec = sanitize_exec_path(exec.clone());
    if exec.contains("flatpak") {
        let parts: Vec<&str> = exec.split_whitespace().collect();
        if parts.len() > 2 && parts[0] == "/usr/bin/flatpak" && parts[1] == "run" {
            let sanitized_exec = sanitize_flatpak_exec(exec);
            return PathBuf::from(sanitized_exec);
        }
    }

    let path = PathBuf::from(exec.trim());
    if path.is_absolute() {
        path
    } else {
        if let Ok(output) = Command::new("which")
            .arg(path.to_string_lossy().as_ref())
            .output()
        {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .into()
            } else {
                path
            }
        } else {
            path
        }
    }
}

fn sanitize_flatpak_exec(exec: String) -> String {
    exec.split("@@").next().unwrap_or(&exec).trim().to_string()
}

fn parse_desktop_entry(path: &Path) -> Option<Browser> {
    let content = fs::read_to_string(path).ok()?;
    let name = extract_field_from_desktop_file("Name=", &content)?;
    let exec = extract_field_from_desktop_file("Exec=", &content)?;
    Some(Browser {
        name,
        exec: resolve_executable_path(exec),
    })
}

fn extract_field_from_desktop_file(prefix: &str, content: &str) -> Option<String> {
    content
        .lines()
        .find(|line| line.starts_with(prefix))
        .map(|line| line.trim_start_matches(prefix).to_string())
}
