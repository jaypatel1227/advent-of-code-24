use crate::puzzle_inputs;

fn day1_p1(list1: &mut [u32], list2: &mut [u32]) -> u32 {
    // assume list1 and list2 are the same length
    assert_eq!(list1.len(), list2.len());
    quick_sort(list1);
    quick_sort(list2);

    let mut distance = 0;

    for i in 0..list1.len() {
        distance += list1[i].abs_diff(list2[i]);
    }

    distance
}

fn day1_p2(list1: &[u32], list2: &[u32]) -> u32 {
    let mut ptr1: usize = 0;
    let mut ptr2: usize = 0;

    let mut distance = 0;

    while ptr1 < list1.len() {
        while ptr2 + 1 < list2.len() && list2[ptr2] < list1[ptr1] {
            ptr2 += 1;
        }

        let mut count = 0;

        if list2[ptr2] == list1[ptr1] {
            while list2[ptr2] == list1[ptr1] {
                count += 1;
                ptr2 += 1;
            }
        }

        distance += list1[ptr1] * count;

        while ptr1 + 1 < list1.len() && list1[ptr1] == list1[ptr1 + 1] {
            ptr1 += 1;
        }

        ptr1 += 1;
        if ptr1 >= list1.len() || ptr2 >= list2.len() {
            break;
        }
    }

    distance
}

pub fn solve_puzzle() {
    println!("=== Day 1 ===");
    let (mut list1, mut list2) = puzzle_inputs::get_day1_input();
    println!("Part 1: {}", day1_p1(&mut list1, &mut list2));
    println!("Part 2: {}", day1_p2(&mut list1, &mut list2));
}

fn quick_sort(list: &mut [u32]) {
    if list.len() <= 1 {
        return;
    }
    quick_sort_inner(list, 0, list.len() - 1);
}

fn quick_sort_inner(list: &mut [u32], start: usize, end: usize) {
    if start < end {
        let pivot = partition(list, start, end);
        if pivot != start {
            quick_sort_inner(list, start, pivot - 1);
        }
        quick_sort_inner(list, pivot + 1, end);
    }
}

fn partition(list: &mut [u32], start: usize, end: usize) -> usize {
    let pivot = list[end];

    let mut i = start;

    for j in start..end {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
    }

    list.swap(i, end);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_empty() {
        let mut list: [u32; 0] = [];
        quick_sort(&mut list);
        assert_eq!(list, []);
    }

    #[test]
    fn test_quick_sort_single_element() {
        let mut list = [42];
        quick_sort(&mut list);
        assert_eq!(list, [42]);
    }

    #[test]
    fn test_quick_sort_sorted() {
        let mut list = [1, 2, 3, 4, 5];
        quick_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse_sorted() {
        let mut list = [5, 4, 3, 2, 1];
        dbg!(list.clone());
        quick_sort(&mut list);
        dbg!(list.clone());
        assert_eq!(list, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_unsorted() {
        let mut list = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        dbg!(list.clone());
        quick_sort(&mut list);
        dbg!(list.clone());
        assert_eq!(list, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_quick_sort_duplicates() {
        let mut list = [5, 3, 8, 3, 1, 3, 9, 3, 5];
        quick_sort(&mut list);
        assert_eq!(list, [1, 3, 3, 3, 3, 5, 5, 8, 9]);
    }

    #[test]
    fn test_quick_sort_large_numbers() {
        let mut list = [1_000_000, 999_999, 1, 50, 10_000, 500];
        quick_sort(&mut list);
        assert_eq!(list, [1, 50, 500, 10_000, 999_999, 1_000_000]);
    }
}
