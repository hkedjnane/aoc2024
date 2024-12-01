fn part_1(input: &str) -> usize {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    input.split('\n').for_each(|line| {
        line.split(' ')
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i32>().unwrap())
            .enumerate()
            .for_each(|(i, x)| {
                if i == 0 {
                    left_list.push(x);
                } else {
                    right_list.push(x);
                }
            })
    });

    let mut result: usize = 0;

    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len() {
        result += (left_list[i] - right_list[i]).abs() as usize;
        // println!("{}", result);
    }

    result
}

fn part_2(input: &str) -> usize {
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    input.split('\n').for_each(|line| {
        line.split(' ')
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i32>().unwrap())
            .enumerate()
            .for_each(|(i, x)| {
                if i == 0 {
                    left_list.push(x);
                } else {
                    right_list.push(x);
                }
            })
    });

    let mut result: usize = 0;

    for left_element in left_list.iter() {
        let mut occ = 0;
        for i in 0..right_list.len() {
            if right_list[i] == *left_element {
                occ += 1;
            }
        }
        result += (left_element * occ) as usize;
    }

    result
}

pub fn solution(input: &str) -> (Option<usize>, Option<usize>) {
    let part1 = Some(part_1(input));
    let part2 = Some(part_2(input));
    (part1, part2)
}
