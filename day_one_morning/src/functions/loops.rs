pub fn while_loop(mut x: i32) {
    while x >= 10 {
        x = x / 2;
    }
    dbg!(x);
}

pub fn for_loop() {
    for x in 1..5 {
        dbg!(x);
    }

    for y in 1..=5 {
        dbg!(y);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }

    for (index, elem) in [10, 20, 30, 40, 50].iter().enumerate() {
        dbg!(index, elem);
    }
}

pub fn loop_loop() {
    let mut i = 0;
    loop {
        i += 1;
        if i == 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        dbg!(i);
    }
}

pub fn nested_for_loop() {
    let s = [[5, 6, 7], [8, 9, 10], [11, 12, 13]];
    let mut elems_searched = 0;
    let target_val = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elems_searched += 1;
            if s[i][j] == target_val {
                break 'outer;
            }
        }
    }
    dbg!(elems_searched);
}
