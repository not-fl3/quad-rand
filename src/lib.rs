use std::sync::atomic::{AtomicU64, Ordering};

mod fy;

const DEFAULT_INC: u64 = 1442695040888963407;
const MULTIPLIER: u64 = 6364136223846793005;

static STATE: AtomicU64 = AtomicU64::new(0);

/// Seeds the pseudo-random number generator used by rand()
/// with the value seed.
pub fn srand(seed: u64) {
    STATE.store(0, Ordering::Relaxed);
    rand();
    let oldstate = STATE.load(Ordering::Relaxed);
    STATE.store(oldstate.wrapping_add(seed), Ordering::Relaxed);
    rand();
}

/// returns a pseudo-random number in the range of 0 to u32::MAX.
pub fn rand() -> u32 {
    let oldstate: u64 = STATE.load(Ordering::Relaxed);
    STATE.store(
        oldstate.wrapping_mul(MULTIPLIER).wrapping_add(DEFAULT_INC),
        Ordering::Relaxed,
    );
    let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
    let rot: u32 = (oldstate >> 59) as u32;
    xorshifted.rotate_right(rot)
}

pub trait RandomRange {
    fn gen_range(low: Self, high: Self) -> Self;
}

impl RandomRange for u8 {
    fn gen_range(low: Self, high: Self) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as u8
    }
}

impl RandomRange for f32 {
    fn gen_range(low: Self, high: Self) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        low + (high - low) * r
    }
}
impl RandomRange for f64 {
    fn gen_range(low: Self, high: Self) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        low + (high - low) * r as f64
    }
}
impl RandomRange for i32 {
    fn gen_range(low: i32, high: i32) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as i32
    }
}
impl RandomRange for i64 {
    fn gen_range(low: Self, high: Self) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as i64
    }
}
impl RandomRange for u32 {
    fn gen_range(low: u32, high: u32) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as u32
    }
}
impl RandomRange for u64 {
    fn gen_range(low: u64, high: u64) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as u64
    }
}
impl RandomRange for i16 {
    fn gen_range(low: i16, high: i16) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as i16
    }
}

impl RandomRange for usize {
    fn gen_range(low: usize, high: usize) -> Self {
        let r = rand() as f32 / std::u32::MAX as f32;
        let r = low as f32 + (high as f32 - low as f32) * r;
        r as usize
    }
}

pub fn gen_range<T>(low: T, high: T) -> T
where
    T: RandomRange,
{
    T::gen_range(low, high)
}

pub struct VecChooseIter<'a, T> {
    source: &'a Vec<T>,
    indices: std::vec::IntoIter<usize>,
}

impl<'a, T> Iterator for VecChooseIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.indices.next().map(|ix| &self.source[ix])
    }
}

pub trait ChooseRandom<T> {
    fn shuffle(&mut self);
    fn choose(&self) -> Option<&T>;
    fn choose_mut(&mut self) -> Option<&mut T>;
    fn choose_multiple(&self, _amount: usize) -> VecChooseIter<T>;
}

impl<T> ChooseRandom<T> for Vec<T> {
    fn shuffle(&mut self) {
        let mut fy = fy::FisherYates::default();

        fy.shuffle(self);
    }

    fn choose(&self) -> Option<&T> {
        let ix = gen_range(0, self.len());
        self.get(ix)
    }

    fn choose_mut(&mut self) -> Option<&mut T> {
        let ix = gen_range(0, self.len());
        self.get_mut(ix)
    }

    fn choose_multiple(&self, amount: usize) -> VecChooseIter<T> {
        let mut indices = (0..self.len())
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        indices.resize(amount, 0);

        VecChooseIter {
            source: self,
            indices: indices.into_iter(),
        }
    }
}

#[cfg(feature = "rand")]
pub mod compat {
    pub struct QuadRand;

    impl rand::RngCore for QuadRand {
        fn next_u32(&mut self) -> u32 {
            crate::gen_range(0, std::u32::MAX)
        }
        
        fn next_u64(&mut self) -> u64 {
            crate::gen_range(0, std::u64::MAX)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for i in 0 .. dest.len() {
                dest[i] = crate::gen_range(0, 255)
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            Ok(self.fill_bytes(dest))
        }
    }
}