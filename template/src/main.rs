use template::solution;

fn main() {
    // if we pass an argument, that file contains the input
    // otherwise we default to ../assets/input.txt
    let input = if std::env::args().len() > 1 {
        let file = std::env::args().nth(1).unwrap();
        let file = std::fs::read_to_string(file).unwrap();
        let file = file.strip_suffix('\n').unwrap().to_string();
        file
    } else {
        include_str!("../assets/input.txt")
            .strip_suffix('\n')
            .unwrap()
            .to_string()
    };

    let (part1, part2) = solution(&input);

    println!("Part 1: {}", part1.unwrap_or_default());
    println!("Part 2: {}", part2.unwrap_or_default());
}
