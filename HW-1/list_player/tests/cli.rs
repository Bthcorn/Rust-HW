use assert_cmd::Command;

#[test]
fn test0players() {
    let mut cmd: Command = Command::cargo_bin("list_player").unwrap();
    cmd.assert().success().stdout("Player 1: N/A\nPlayer 2: N/A\n");
}

#[test]
fn test1players() {
    let mut cmd: Command = Command::cargo_bin("list_player").unwrap();
    cmd.arg("Mike").assert().success().stdout("Player 1: Mike\nPlayer 2: N/A\n");
}

#[test]
fn test2players() {
    let mut cmd: Command = Command::cargo_bin("list_player").unwrap();
    cmd.arg("Mike").arg("John").assert().success().stdout("Player 1: Mike\nPlayer 2: John\n");
}

#[test]
fn test3players() {
    let mut cmd: Command = Command::cargo_bin("list_player").unwrap();
    cmd.arg("Mike").arg("John").arg("Henry").assert().success().stdout("Player 1: Mike\nPlayer 2: John\n");
}

