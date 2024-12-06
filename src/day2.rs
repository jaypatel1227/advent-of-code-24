use crate::puzzle_inputs::get_day2_input;

fn day2() -> (i32, i32) {
    let mut safe_count = 0;
    let mut one_off_safe = 0;

    let rows = get_day2_input();

    for row in rows {
        safe_count += is_safe(row.clone()) as i32;
        one_off_safe += is_safe_up_to_one(row) as i32;
    }

    return (safe_count, one_off_safe);
}

pub fn solve_puzzle() {
    println!("=== Day 2 ===");
    let (p1, p2) = day2();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn is_safe(list: Vec<i32>) -> bool {
    if list.len() < 2 {
        return true;
    }

    if list.len() == 2 {
        return true;
    }
    let direction = list[0] < list[1];
    for i in 0..list.len() - 1 {
        if ((list[i] < list[i + 1]) != direction)
            || (list[i] == list[i + 1] || list[i].abs_diff(list[i + 1]) > 3)
        {
            return false;
        }
    }

    true
}

fn is_safe_up_to_one(list: Vec<i32>) -> bool {
    for i in 0..list.len() {
        let mut modified_list = list.clone();
        modified_list.remove(i);
        if is_safe(modified_list) {
            return true;
        }
    }
    return false;
}
