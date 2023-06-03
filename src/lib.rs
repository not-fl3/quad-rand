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

/// Returns a pseudo-random number in the range of 0 to u32::MAX.
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

macro_rules! impl_random_range{
  ($($ty:ty),*,)=>{
    $(
      impl RandomRange for $ty{
        #[inline]
        fn gen_range(low: Self, high: Self) -> Self {
          let r = rand() as f64 / (u32::MAX as f64 + 1.0);
          let r = low as f64 + (high as f64 - low as f64) * r;
          r as Self
        }
      }
    )*
  }
}
impl_random_range!(f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize,);

pub fn gen_range<T>(low: T, high: T) -> T
where
    T: RandomRange,
{
    T::gen_range(low, high)
}

pub struct SliceChooseIter<'a, T> {
    source: &'a [T],
    indices: std::vec::IntoIter<usize>,
}

impl<'a, T> Iterator for SliceChooseIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.indices.next().map(|ix| &self.source[ix])
    }
}

pub trait ChooseRandom<T> {
    fn shuffle(&mut self);
    fn choose(&self) -> Option<&T>;
    fn choose_mut(&mut self) -> Option<&mut T>;
    fn choose_multiple(&self, _amount: usize) -> SliceChooseIter<T>;
}

impl<T> ChooseRandom<T> for [T] {
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

    fn choose_multiple(&self, amount: usize) -> SliceChooseIter<T> {
        let mut indices = (0..self.len())
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        indices.resize(amount, 0);

        SliceChooseIter {
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
            for i in 0..dest.len() {
                dest[i] = crate::gen_range(0, 255)
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            Ok(self.fill_bytes(dest))
        }
    }
}
