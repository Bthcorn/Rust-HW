// test for stdout assert_cmd
use assert_cmd::Command;

#[test]
fn test_sort_point() {
    let mut cmd: Command = Command::cargo_bin("sort_point2").unwrap();
    cmd.arg("2")
        .arg("3")
        .arg("1")
        .arg("4")
        .arg("3")
        .arg("2")
        .arg("2")
        .arg("1")
        .assert()
        .success()
        .stdout("Ascending: [(1.0, 4.0), (2.0, 1.0), (2.0, 3.0), (3.0, 2.0)]\nDescending: [(3.0, 2.0), (2.0, 3.0), (2.0, 1.0), (1.0, 4.0)]\n");
}
