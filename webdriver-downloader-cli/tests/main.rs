use std::ffi::OsStr;

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::ord::eq;
use predicates::prelude::*;
use predicates::str::ends_with;

use webdriver_downloader::prelude::*;

const CHROMEDRIVER_BIN: &str = os_specific::chromedriver::DRIVER_EXECUTABLE_NAME;

// Tests for chromedriver, which also checks overall functionality.
#[test]
fn test_passes_no_mkdir() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.success();
    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());
}

#[test]
fn test_passes_mkdir() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push("new_dir");
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--mkdir"),
            ]
            .iter(),
        )
        .assert();

    assert.success();
    temp_dir
        .child(format!("new_dir/{}", CHROMEDRIVER_BIN))
        .assert(predicate::path::exists());
}

#[test]
fn test_existing_driver() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.success();
    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.success().stdout(eq("Driver already installed.\n"));

    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());
}

#[test]
fn test_reinstall() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.success();
    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--reinstall"),
            ]
            .iter(),
        )
        .assert();

    assert
        .success()
        .stdout(ends_with("Driver installed successfully.\n"));

    temp_dir
        .child(CHROMEDRIVER_BIN)
        .assert(predicate::path::exists());
}

#[test]
fn test_fails_no_mkdir_and_no_dir() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push("new_dir");
    driver_path.push(CHROMEDRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--driver"), driver_path.as_os_str()].iter())
        .assert();

    assert.failure();
    temp_dir.child("new_dir").assert(predicate::path::missing());
}

#[test]
fn test_fails_no_browser() {
    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args([OsStr::new("--browser"), OsStr::new("no_browser.exe")].iter())
        .assert();

    assert.failure();
}

// Tests for other drivers.

// geckodriver
const GECKODRIVER_BIN: &str = os_specific::geckodriver::DRIVER_EXECUTABLE_NAME;

#[test]
fn test_geckodriver() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut driver_path = temp_dir.to_path_buf();
    driver_path.push(GECKODRIVER_BIN);

    let mut cmd = Command::cargo_bin("webdriver-downloader").unwrap();
    let assert = cmd
        .args(
            [
                OsStr::new("--driver"),
                driver_path.as_os_str(),
                OsStr::new("--type"),
                OsStr::new("gecko"),
            ]
            .iter(),
        )
        .assert();

    assert.success();
    temp_dir
        .child(GECKODRIVER_BIN)
        .assert(predicate::path::exists());
}
