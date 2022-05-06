use std::fs;

use assert_cmd::Command;
use futures::io;
use tempdir::TempDir;

#[test]
fn swoon_init_minimum_happy_test() -> io::Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let work_dir = TempDir::new("swoon_init_test")?;

    cmd.current_dir(work_dir.path())
        .arg("init")
        .arg("--non-interactive")
        .arg("--org-name=acme")
        .assert()
        .success();

    assert_eq!(fs::read_to_string(work_dir.path().join("swoon.yml"))?, r"---
org_name: acme
default_platform: gcp
default_os: debian:11
");

    Ok(())
}

#[test]
fn swoon_init_with_default_os_test() -> io::Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let work_dir = TempDir::new("swoon_init_test")?;

    cmd.current_dir(work_dir.path())
        .arg("init")
        .arg("--non-interactive")
        .arg("--org-name=acme")
        .arg("--operating-system=debian:10")
        .assert()
        .success();

    assert_eq!(fs::read_to_string(work_dir.path().join("swoon.yml"))?, r"---
org_name: acme
default_platform: gcp
default_os: debian:10
");

    Ok(())
}

#[test]
fn swoon_init_unhappy_when_no_org_name() -> io::Result<()> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let work_dir = TempDir::new("swoon_init_test")?;

    cmd.current_dir(work_dir.path())
        .arg("init")
        .arg("--non-interactive")
        .assert()
        .failure();

    assert_eq!(false, work_dir.path().join("swoon.yml").exists());

    Ok(())
}
