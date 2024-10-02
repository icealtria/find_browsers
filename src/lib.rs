mod linux;
mod windows;

use std::path::PathBuf;

#[derive(Debug)]
pub struct Browser {
    pub name: String,
    pub exec: PathBuf,
}

pub fn get_browsers() -> Result<Vec<Browser>, Box<dyn std::error::Error>> {
    if cfg!(target_os = "linux") {
        linux::get_browsers()
    } else if cfg!(target_os = "windows") {
        windows::get_browsers()
    } else {
        Err("Unsupported operating system".into())
    }
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
