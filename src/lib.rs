#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Browser {
    pub name: String,
    pub exec: PathBuf,
}

pub fn get_browsers() -> Result<Vec<Browser>, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    return windows::get_browsers();
    #[cfg(target_os = "linux")]
    return linux::get_browsers();
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    return Err("Unsupported OS".into());
}

#[cfg(target_os = "linux")]
pub fn get_executable_browsers() -> Result<Vec<Browser>, Box<dyn std::error::Error>> {
    let browsers = get_browsers()?;
    Ok(browsers.into_iter().filter(|b| b.exec.exists()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_browsers() {
        let browsers = get_browsers().unwrap();
        for browser in &browsers {
            println!("Browser: {}\n Exec: {:?}", browser.name, browser.exec);
        }
        assert!(!browsers.is_empty(), "No browsers found");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn print_executable_browsers() {
        let browsers = get_executable_browsers().unwrap();
        for browser in &browsers {
            println!("Executable Browser: {}\n Exec: {:?}", browser.name, browser.exec);
            assert!(browser.exec.exists(), "Browser should exist");
        }
    }
}
