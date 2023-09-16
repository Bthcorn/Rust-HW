use assert_cmd::Command;
#[test]
fn test_ascending_sort() {
    let mut array = vec![];
    let sorted = sort_value::ascending_sort(&mut array);
    assert_eq!(sorted, vec![]);

    let mut array = vec![5, 2, 1, 3, 4];
    let sorted = sort_value::ascending_sort(&mut array);
    assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
} // add more tests

#[test]
fn test_descending_sort() {
    let mut array = vec![];
    let sorted = sort_value::descending_sort(&mut array);
    assert_eq!(sorted, vec![]);

    let mut array = vec![5, 2, 1, 3, 4];
    let sorted = sort_value::descending_sort(&mut array);
    assert_eq!(sorted, vec![5, 4, 3, 2, 1]);
} 

#[test]
fn test_sort_value() {
    let mut cmd: Command = Command::cargo_bin("sort_value").unwrap();
    cmd.arg("4")
        .arg("7")
        .arg("8")
        .arg("9")
        .arg("3")
        .arg("4")
        .assert()
        .success()
        .stdout("Ascending: [3, 4, 4, 7, 8, 9]\nDescending: [9, 8, 7, 4, 4, 3]\n");
}