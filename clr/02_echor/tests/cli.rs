use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

// 実行で Err が発生してもパニックさせないために、テスト結果を表す型を用意する
type TestResult = Result<(), Box<dyn std::error::Error>>;

// コマンド実行でエラーが発生すると以下の表示になる
// Error: CargoError { cause: Some(NotFoundError { path: ".../target/debug/echora" }) }
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1n() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2n() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

// 以下は Err があるとパニックが発生するテストコード
// thread 'dies_no_args' panicked at tests/cli.rs:17:48:
// called `Result::unwrap()` on an `Err` value: CargoError { cause: Some(NotFoundError { path: ".../target/debug/echora" }) }
// #[test]
// fn dies_no_args() {
//     let mut cmd = Command::cargo_bin("echora").unwrap();
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("Usage"));
// }
//
// #[test]
// fn runs() {
//     let outfile = "tests/expected/hello1.txt";
//     let expected = fs::read_to_string(outfile).unwrap();
//     let mut cmd = Command::cargo_bin("echor").unwrap();
//     cmd.arg("Hello there").assert().success().stdout(expected);
//
//     // let mut cmd = Command::cargo_bin("echor").unwrap();
//     // cmd.arg("hello").assert()
//     //     .success();
// }