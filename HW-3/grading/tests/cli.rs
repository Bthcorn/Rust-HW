use assert_cmd::Command;

#[test]
fn test_grading () {
    let mut cmd : Command = Command::cargo_bin("grading").unwrap();
    cmd.arg("-1").assert().success().stdout("Your grade is: Invalid score.\n");
}

#[test]
fn test_grading_75 () {
    let mut cmd : Command = Command::cargo_bin("grading").unwrap();
    cmd.arg("75").assert().success().stdout("Your grade is: B.\n");
}

#[test]
fn test_grading_101 () {
    let mut cmd : Command = Command::cargo_bin("grading").unwrap();
    cmd.arg("101").assert().success().stdout("Your grade is: Invalid score.\n");
}

