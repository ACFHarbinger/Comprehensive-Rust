pub fn immutable_tuples() {
    let t: (i8, bool, f32) = (1, true, 3.0);
    dbg!(t.0);
    dbg!(t.1);
    dbg!(t.2);
}

pub fn mutable_tuples() {
    let mut t = (1, true);
    t.0 = 2;
    t.1 = false;
    dbg!(t);
}
