use quad_rand as qrand;

fn main() {
    // seed random
    qrand::srand(12345);

    // get random number from 0 to u32::MAX
    let x = qrand::rand();

    // get random number from given range
    let x = qrand::gen_range(0., 1.);
    assert!(x >= 0. && x < 1.);
    println!("x={}", x);

    // gen_range works for most of standard number types
    let x: u8 = qrand::gen_range(64, 128);
    assert!(x >= 64 && x < 128);
    println!("x={}", x);
}
