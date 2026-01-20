pub fn block_and_scope() {
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y
    };
    dbg!(x);
    // dbg!(y);
}

pub fn if_expression(x: i32) -> String {
    let size = if x < 20 { "small" } else { "large" };
    format!("Number size: {size}")
}

pub fn match_expression(x: i32) -> &'static str {
    match x {
        1 => "One.",
        10 => "Ten.",
        100 => "One Hundred.",
        _ => "Something else.",
    }
}

pub fn break_label() {
    #[allow(unreachable_code)]
    'label: {
        break 'label;
        println!("This will never be printed.");
    }
}
