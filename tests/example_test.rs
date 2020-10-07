use assert_cmd::Command;

#[test]
fn day_one_part_one() {
    let input = "12\n14\n1969\n100756";
    let expected = 2 + 2 + 654 + 33583;
    let expected_str = format!("{}\n", expected);

    let mut command = Command::cargo_bin("d01p1").unwrap();
    let assert = command.write_stdin(input).assert();

    assert.success().stdout(expected_str);
}

#[test]
fn day_one_part_two() {
    let input = "12\n14\n1969\n100756";
    let expected =
        2 + 2 + (654 + 216 + 70 + 21 + 5) + (33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2);
    let expected_str = format!("{}\n", expected);

    let mut command = Command::cargo_bin("d01p2").unwrap();
    let assert = command.write_stdin(input).assert();

    assert.success().stdout(expected_str);
}

#[test]
fn day_two_part_one() {
    let input = "1,1,1,4,99,5,6,0,99";
    let expected_str = "30\n";

    let mut command = Command::cargo_bin("d02p1").unwrap();
    let assert = command.write_stdin(input).assert();

    assert.success().stdout(expected_str);
}
