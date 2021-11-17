use crate::utils;

fn day_1_part_1(report: &[u32]) -> u32 {
    let (a, b) = find_tuple_sum_to_target(2020, report, None).expect("values do not sum");
    a * b
}

fn day_1_part_2(report: &[u32]) -> u32 {
    let (a, b, c) = find_three_tuple_sum_to_target(2020, report).expect("values do not sum");
    a * b * c
}

fn find_three_tuple_sum_to_target(target: u32, numbers: &[u32]) -> Option<(u32, u32, u32)> {
    if numbers.len() < 2 {
        return None;
    }

    let head_v = *numbers.get(0).expect("no number");
    let tail = &numbers[1..];

    match find_tuple_sum_to_target(target, tail, Some(head_v)) {
        Some((a, b)) => Some((head_v, a, b)),
        None => find_three_tuple_sum_to_target(target, &tail),
    }
}

fn find_tuple_sum_to_target(target: u32, list: &[u32], seed: Option<u32>) -> Option<(u32, u32)> {
    if list.len() < 2 {
        return None;
    }

    let seed_v = seed.unwrap_or(0);

    let head_v = *list.get(0).expect("no number");
    let tail = &list[1..];

    match tail
        .iter()
        .find(|&number| seed_v + head_v + number == target)
    {
        Some(&n) => Some((head_v, n)),
        None => find_tuple_sum_to_target(target, tail, seed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_part_1() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(day_1_part_1(&vec), 514579);

        let my_vec: Vec<u32> = utils::read_lines_as_strings(String::from("res/input_1.txt"));

        assert_eq!(day_1_part_1(&my_vec), 1005459)
    }

    #[test]
    fn test_find_sum_to_target_partner() {
        assert_eq!(find_tuple_sum_to_target(2020, &vec![], None), None);

        assert_eq!(find_tuple_sum_to_target(2020, &vec![3], None), None);

        assert_eq!(
            find_tuple_sum_to_target(2020, &vec![1, 2019], None),
            Some((1, 2019)),
        );

        assert_eq!(
            find_tuple_sum_to_target(2020, &(vec![2019, 1]), None),
            Some((2019, 1))
        );

        assert_eq!(find_tuple_sum_to_target(2020, &(vec![1, 2, 3]), None), None);
        assert_eq!(
            find_tuple_sum_to_target(2020, &(vec![1, 2019]), None),
            Some((1, 2019))
        );
        assert_eq!(
            find_tuple_sum_to_target(2020, &(vec![10, 2019, 1]), None),
            Some((2019, 1))
        );
        assert_eq!(
            find_tuple_sum_to_target(2020, &(vec![10, 2019, 3, 1]), None),
            Some((2019, 1))
        );
        assert_eq!(
            find_tuple_sum_to_target(2020, &(vec![10, 2018, 2019, 1, 2]), None),
            Some((2018, 2))
        );
    }

    // Part 2

    #[test]
    fn test_day_1_part_2() {
        let vec = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(day_1_part_2(&vec), 241861950);

        let my_vec: Vec<u32> = utils::read_lines_as_strings(String::from("res/input_1.txt"));

        assert_eq!(day_1_part_1(&my_vec), 1005459)
    }

    #[test]
    fn test_find_three_tuple_sum_to_2020() {
        assert_eq!(
            find_three_tuple_sum_to_target(2020, &vec![1, 2, 3, 4, 2019]),
            None,
        );

        assert_eq!(
            find_three_tuple_sum_to_target(2020, &vec![1, 1, 2018]),
            Some((1, 1, 2018)),
        );

        assert_eq!(
            find_three_tuple_sum_to_target(2020, &vec![1, 2, 3, 1, 4, 2018]),
            Some((1, 1, 2018)),
        );

        assert_eq!(
            find_three_tuple_sum_to_target(2020, &vec![8, 2, 1, 2018, 4, 1]),
            Some((1, 2018, 1)),
        );
    }
}
