fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (first, second, third) = tuple;
    first < second && second < third
}

pub fn pattern_destructuring() {
    let tuple = (1, 5, 3);
    let tuple2: (i32, i32, i32) = (1, 3, 5);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) {
            "Ordered"
        } else {
            "Unordered"
        }
    );
    println!(
        "{tuple2:?}: {}",
        if check_order(tuple2) {
            "Ordered"
        } else {
            "Unordered"
        }
    );
}

pub fn refutable_pattern() -> String {
    let some_option: Option<i32> = None;
    if let Some(val) = some_option {
        return format!("Value is: {val}");
    } else {
        return String::from("No value found.");
    }
}
