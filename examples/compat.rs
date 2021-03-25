use quad_rand::compat::QuadRand;
use rand::seq::SliceRandom;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    println!("ordered: {:?}", vec);

    // QuadRand is rand::RngCore implementation, allowing to use all the cool stuff from rand
    vec.shuffle(&mut QuadRand);
    println!("shuffled: {:?}", vec);
}
