use assert_cmd::Command;
// test Question 3.1
#[test]
fn test_main() {
    let mut cmd: Command = Command::cargo_bin("arrow").unwrap();
    cmd.arg("").assert().success().stdout("");
}

#[test]
fn test_main1() {
    let mut cmd: Command = Command::cargo_bin("arrow").unwrap();
    cmd.arg("1").assert().success().stdout("*\n");
}

#[test]
fn test_main2() {
    let mut cmd: Command = Command::cargo_bin("arrow").unwrap();
    cmd.arg("2").assert().success().stdout("*\n**\n*\n");
}

#[test]
fn test_main3() {
    let mut cmd: Command = Command::cargo_bin("arrow").unwrap();
    cmd.arg("3").assert().success().stdout("*\n**\n***\n**\n*\n");
}

#[test]
fn test_main4() {
    let mut cmd: Command = Command::cargo_bin("arrow").unwrap();
    cmd.arg("4").assert().success().stdout("*\n**\n***\n****\n***\n**\n*\n");
}

#[test]
fn test_main5() {
    let mut cmd: Command = Command::cargo_bin("arrow").unwrap();
    cmd.arg("5").assert().success().stdout("*\n**\n***\n****\n*****\n****\n***\n**\n*\n");
}
// =====================================================
// test Question 3.2
#[test]
fn test_flip_arrow() {
    let mut cmd: Command = Command::cargo_bin("flip_arrow").unwrap();
    cmd.arg("").assert().success().stdout("");
}

#[test]
fn test_flip_arrow1() {
    let mut cmd: Command = Command::cargo_bin("flip_arrow").unwrap();
    cmd.arg("1").assert().success().stdout("*\n");
}

#[test]
fn test_flip_arrow2() {
    let mut cmd: Command = Command::cargo_bin("flip_arrow").unwrap();
    cmd.arg("2").assert().success().stdout(" *\n**\n *\n");
}

#[test]
fn test_flip_arrow3() {
    let mut cmd: Command = Command::cargo_bin("flip_arrow").unwrap();
    cmd.arg("3").assert().success().stdout("  *\n **\n***\n **\n  *\n");
}

#[test]
fn test_flip_arrow4() {
    let mut cmd: Command = Command::cargo_bin("flip_arrow").unwrap();
    cmd.arg("4").assert().success().stdout("   *\n  **\n ***\n****\n ***\n  **\n   *\n");
}

#[test]
fn test_flip_arrow5() {
    let mut cmd: Command = Command::cargo_bin("flip_arrow").unwrap();
    cmd.arg("5").assert().success().stdout("    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n");
}