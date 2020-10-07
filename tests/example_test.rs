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

#[test]
fn day_seven_part_one() {
    let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let expected_str = "43210\n";

    let mut command = Command::cargo_bin("d07p1").unwrap();
    let assert = command.write_stdin(input).assert();

    assert.success().stdout(expected_str);
}

#[test]
fn day_seven_part_two() {
    let input =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let expected_str = "139629729\n";

    let mut command = Command::cargo_bin("d07p2").unwrap();
    let assert = command.write_stdin(input).assert();

    assert.success().stdout(expected_str);
}
