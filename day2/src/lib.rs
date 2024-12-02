use itertools::Itertools;

fn count_safe(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let mut count = 0;

    for report in reports {
        let mut direction = -1;
        let mut safe = true;
        for (a, b) in report.iter().tuple_windows() {
            let local_direction = if a < b { 1 } else { 0 };
            if direction < 0 {
                direction = local_direction;
            } else if direction != local_direction {
                safe = false;
                break;
            }
            let abs = (a - b).abs();
            if abs < 1 || abs > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            count += 1;
        }
    }

    count
}

fn is_safe(report: &Vec<i32>, can_adjust: bool) -> bool {
    let mut direction = -1;

    for (x, y) in (0..report.len()).tuple_windows() {
        let (a, b) = (report[x], report[y]);
        let local_direction = if a < b { 1 } else { 0 };
        let mut local_safe = true;
        if direction < 0 {
            direction = local_direction;
        } else if direction != local_direction {
            local_safe = false;
        }
        if a == b {
            local_safe = false;
        }
        let abs = (a - b).abs();
        if abs < 1 || abs > 3 {
            local_safe = false;
        }
        if local_safe {
            continue;
        }
        if !can_adjust {
            return false;
        } else {
            for x in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(x);
                if is_safe(&new_report, false) {
                    return true;
                }
            }
            return false;
        };
    }

    true
}

fn count_safe_2(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let mut count = 0;

    for report in reports {
        if is_safe(&report, true) {
            count += 1;
        }
    }

    count
}

pub fn solution(input: &str) -> (Option<i32>, Option<i32>) {
    let part1 = Some(count_safe(input));
    let part2 = Some(count_safe_2(input));
    (part1, part2)
}
