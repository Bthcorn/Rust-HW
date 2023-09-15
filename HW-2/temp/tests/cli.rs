use assert_cmd::Command;

#[test] 
fn test_temp_ftoc() {
    let mut cmd: Command = Command::cargo_bin("temp").unwrap();
    cmd.arg("100").assert().success().stdout("The temperature (Celsius): 37.7778");
}

#[test]
fn test_temp_FtoC() {
    let mut cmd: Command = Command::cargo_bin("F_to_C").unwrap();
    cmd.arg("100").assert().success().stdout("The temperature (Celsius): 37.7778");
}

#[test]
fn test_temp_ctof() {
    let mut cmd: Command = Command::cargo_bin("C_to_F").unwrap();
    cmd.arg("0").assert().success().stdout("The temperature (Fahrenheit): 32");
}