use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn test_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

// #[test]
// fn runs() {
//     let mut cmd = Command::cargo_bin("hello").unwrap();
//     cmd.assert().success();
// }
//
// use std::process::Command;
//
// #[test]
// fn runs() {
//     let mut cmd = Command::new("hello");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }