use std::ffi::OsString;
use std::path::{Path, PathBuf};

use semver::Version;

use crate::os_specific::DefaultPathError;
use crate::traits::version_req_url_info::VersionReqError;

pub const DRIVER_EXECUTABLE_NAME: &str = "geckodriver.exe";

pub fn default_browser_path() -> Result<PathBuf, DefaultPathError> {
    let program_files = std::env::var("ProgramFiles")?;
    Ok(PathBuf::from(format!(
        r"{}\Mozilla Firefox\firefox.exe",
        program_files
    )))
}

pub fn build_url(version_string: &str) -> String {
    format!(
        "https://github.com/mozilla/geckodriver/releases/download/v{ver}/geckodriver-v{ver}-win64.zip",
        ver=version_string
    )
}

pub fn binary_version(browser_path: &Path) -> Result<Version, VersionReqError> {
    let mut child = std::process::Command::new("powershell");

    let mut command = OsString::from("(Get-Item \"");
    command.push(browser_path);
    command.push("\").VersionInfo.FileVersion");

    child.arg("-command").arg(command);

    let output = child.output()?;
    lenient_semver::parse(&String::from_utf8_lossy(&output.stdout)).map_err(|e| e.owned().into())
}
