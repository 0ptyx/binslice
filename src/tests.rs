use crate::common::BinSlice;
use assert_cmd::prelude::*;
use predicates::{
    self,
    ord::eq,
    str::{self as pstr, contains, is_empty, PredicateStrExt},
};
use std::process::Command;
use tempfile::TempDir;
use walkdir::WalkDir;

// `bs` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("bs").unwrap().assert().failure();
}

// `bs -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("bs")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_takes_path_and_pattern() {
    Command::cargo_bin("bs")
        .unwrap()
        .args(&[
            "--file='tests/datafiles/bin-01.dat'",
            "--pattern='{bit}{u8}{u8}{cstr}{ptr32}'",
            "--raw-binary",
            "--auto-padding=yes",
            "--auto-align=yes",
        ])
        .assert()
        .stdout(contains("unimplemented"));
}
