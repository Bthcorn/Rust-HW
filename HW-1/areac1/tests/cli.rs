use assert_cmd::Command;

#[test]
fn test_area(){
    let mut cmd: Command = Command::cargo_bin("areac").unwrap();
    cmd.arg("4").assert().success().stdout( "area of the circle: 50.2656");
}