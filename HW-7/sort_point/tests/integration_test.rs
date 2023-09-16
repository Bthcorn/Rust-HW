use assert_cmd::Command;

#[test] 
fn test_ascending_sort_point() {
    let test = sort_point::ascending_point(&mut vec![(2.0, 3.0), (1.0, 4.0), (3.0, 2.0), (2.0, 1.0)]);
    assert_eq!(test, [(1.0, 4.0), (2.0, 1.0), (2.0, 3.0), (3.0, 2.0)]);
}

#[test] 
fn test_descending_sort_point() {
    let test = sort_point::descending_point(&mut vec![(2.0, 3.0), (1.0, 4.0), (3.0, 2.0), (2.0, 1.0)]);
    assert_eq!(test, [(3.0, 2.0), (2.0, 3.0), (2.0, 1.0), (1.0, 4.0)]);
}

#[test]
fn test_sort_point() {
    let mut cmd: Command = Command::cargo_bin("sort_point").unwrap();
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

