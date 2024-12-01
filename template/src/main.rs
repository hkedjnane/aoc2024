use dayx::solution;

fn main(int argc: i32, argv: Vec<String>) {
    let input = include_str!("../assets/input.txt")
        .strip_suffix('\n')
        .unwrap();

    let test_input = include_str!("../assets/test.txt")
        .strip_suffix('\n')
        .unwrap();

    let actual_input = if argc > 1 && argv[1] == "test" {
        test_input
    } else {
        input
    };

    let (part1, part2) = solution(actual_input);
    println!("Part 1: {}", part1.unwrap_or_default());
    println!("Part 2: {}", part2.unwrap_or_default());
}
