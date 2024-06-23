use assert_cmd::Command;

#[test]
fn test_1() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/1/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/1/output.txt"));
}

#[test]
fn test_2() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/2/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/2/output.txt"));
}

#[test]
fn test_3() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/3/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/3/output.txt"));
}

#[test]
fn test_10() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/10/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/10/output.txt"));
}

#[test]
fn test_20() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/20/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/20/output.txt"));
}

#[test]
fn test_10000_unique_keys() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/10000_unique_keys/input.txt"]);
    command.assert().success().stdout(include_str!(
        "../tests/fixtures/10000_unique_keys/output.txt"
    ));
}

#[test]
fn test_boundaries() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/boundaries/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/boundaries/output.txt"));
}

#[test]
fn test_complex_utf8() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/complex_utf8/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/complex_utf8/output.txt"));
}

#[test]
fn test_dot() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/dot/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/dot/output.txt"));
}

#[test]
fn test_rounding() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/rounding/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/rounding/output.txt"));
}

#[test]
fn test_short() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/short/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/short/output.txt"));
}

#[test]
fn test_shortest() {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.args(["tests/fixtures/shortest/input.txt"]);
    command
        .assert()
        .success()
        .stdout(include_str!("../tests/fixtures/shortest/output.txt"));
}
