#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;

#[derive(Debug)]
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
}
