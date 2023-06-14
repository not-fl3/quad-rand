use quad_rand as qrand;

fn main() {
    // seed random
    let randomness = qrand::RandGenerator::new();
    randomness.srand(12345);

    // get random number from 0 to u32::MAX
    let x = randomness.rand();

    // get random number from given range
    let x = randomness.gen_range(0., 1.);
    assert!(x >= 0. && x < 1.);
    println!("x={}", x);

    // gen_range works for most of standard number types
    let x: u8 = randomness.gen_range(64, 128);
    assert!(x >= 64 && x < 128);
    println!("x={}", x);
}
