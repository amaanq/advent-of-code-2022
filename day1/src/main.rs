const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (max_cal_prob_a, elf_idx_prob_a) = a();
    println!("Day 1(a): max_cal={max_cal_prob_a}, elf_idx={elf_idx_prob_a}");

    let (max_cal_a, max_cal_b, max_cal_c, elf_idx_a, elf_idx_b, elf_idx_c) = b();
    println!("Day 1(b): max_cal_a={max_cal_a}, max_cal_b={max_cal_b}, max_cal_c={max_cal_c}, sum={}, elf_idx_a={elf_idx_a}, elf_idx_b={elf_idx_b}, elf_idx_c={elf_idx_c}", max_cal_a+max_cal_b+max_cal_c);
}

fn a() -> (i32, usize) {
    let (mut max_calories, mut max_elf_idx, mut curr_calories, mut elf_idx) = (0, 0, 0, 0);

    for line in INPUT.lines() {
        if line.is_empty() {
            elf_idx += 1;
            if curr_calories > max_calories {
                max_calories = curr_calories;
                max_elf_idx = elf_idx;
            }
            curr_calories = 0;
            continue;
        }

        curr_calories += line.parse::<i32>().unwrap();
    }

    (max_calories, max_elf_idx)
}

fn b() -> (i32, i32, i32, usize, usize, usize) {
    let (mut max_cal_a, mut max_cal_b, mut max_cal_c) = (0, 0, 0);
    let (mut elf_idx_a, mut elf_idx_b, mut elf_idx_c) = (0, 0, 0);

    let mut curr_calories = 0;
    let mut elf_idx = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            elf_idx += 1;
            match (
                curr_calories > max_cal_a,
                curr_calories > max_cal_b,
                curr_calories > max_cal_c,
            ) {
                (true, true, true) => {
                    // curr > all
                    max_cal_c = max_cal_b;
                    max_cal_b = max_cal_a;
                    max_cal_a = curr_calories;

                    elf_idx_c = elf_idx_b;
                    elf_idx_b = elf_idx_a;
                    elf_idx_a = elf_idx;
                }
                (false, true, true) => {
                    // curr > b & c
                    max_cal_c = max_cal_b;
                    max_cal_b = curr_calories;

                    elf_idx_c = elf_idx_b;
                    elf_idx_b = elf_idx;
                }
                (false, false, true) => {
                    // curr > c only
                    max_cal_c = curr_calories;

                    elf_idx_c = elf_idx;
                }
                (false, false, false) => {
                    // curr < all 3
                }
                (true | false, _, _) => {
                    // curr > a but not b | c?
                    // &
                    // curr > b but not c?
                    panic!("curr_calories={curr_calories}, max_cal_a={max_cal_a}, max_cal_b={max_cal_b}, max_cal_c={max_cal_c}, elf_idx_a={elf_idx_a}, elf_idx_b={elf_idx_b}, elf_idx_c={elf_idx_c}");
                }
            }

            curr_calories = 0;
            continue;
        }

        curr_calories += line.parse::<i32>().unwrap();
    }

    (
        max_cal_a, max_cal_b, max_cal_c, elf_idx_a, elf_idx_b, elf_idx_c,
    )
}
