use quad_rand as qrand;

// all kinds of range types can be used
fn main() {
    // seed random
    qrand::srand(12345);

    let r = qrand::gen_range(4..7);
    assert!(r >= 4 && r < 7);

    let r = qrand::gen_range(4..=7);
    assert!(r >= 4 && r <= 7);

    let r = qrand::gen_range(252u8..);
    assert!(r >= 252);

    let r: i16 = quad_rand::gen_range(..);
    println!("r={}", r);

    let r = qrand::gen_range(..3);
    assert!(r < 3);

    let r = qrand::gen_range(1.0..=2.0);
    assert!(r >= 1.0 && r <= 2.0);
}
