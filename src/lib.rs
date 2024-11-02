#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

mod fy;

const DEFAULT_INC: u64 = 1442695040888963407;
const MULTIPLIER: u64 = 6364136223846793005;

pub struct RandGenerator {
    state: AtomicU64,
}

impl RandGenerator {
    pub const fn new() -> Self {
        Self {
            state: AtomicU64::new(0),
        }
    }
    pub fn srand(&self, seed: u64) {
        self.state.store(0, Ordering::Relaxed);
        self.rand();
        let oldstate = self.state.load(Ordering::Relaxed);
        self.state
            .store(oldstate.wrapping_add(seed), Ordering::Relaxed);
        self.rand();
    }
    pub fn rand(&self) -> u32 {
        let oldstate: u64 = self.state.load(Ordering::Relaxed);
        self.state.store(
            oldstate.wrapping_mul(MULTIPLIER).wrapping_add(DEFAULT_INC),
            Ordering::Relaxed,
        );
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot: u32 = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
    #[inline]
    pub fn gen_range<T>(&self, low: T, high: T) -> T
    where
        T: RandomRange,
    {
        T::gen_range_with_state(self, low, high)
    }
}

static GLOBAL_STATE: RandGenerator = RandGenerator::new();

/// Seeds the pseudo-random number generator used by rand()
/// with the value seed.
#[inline]
pub fn srand(seed: u64) {
    GLOBAL_STATE.srand(seed);
}

/// Returns a pseudo-random number in the range of 0 to u32::MAX.
#[inline]
pub fn rand() -> u32 {
    GLOBAL_STATE.rand()
}

pub trait RandomRange {
    fn gen_range(low: Self, high: Self) -> Self;
    fn gen_range_with_state(state: &RandGenerator, low: Self, high: Self) -> Self;
}

macro_rules! impl_random_range{
    ($($ty:ty),*,)=>{
        $(
            impl RandomRange for $ty{
                #[inline]
                fn gen_range(low: Self, high: Self) -> Self{
                    Self::gen_range_with_state(&GLOBAL_STATE, low, high)
                }
                #[inline]
                fn gen_range_with_state(gen: &RandGenerator, low: Self, high: Self) -> Self {
                    let r = gen.rand() as f64 / (u32::MAX as f64 + 1.0);
                    let r = low as f64 + (high as f64 - low as f64) * r;
                    r as Self
                }
            }
        )*
    }
}
impl_random_range!(f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize,);

#[inline]
pub fn gen_range<T>(low: T, high: T) -> T
where
    T: RandomRange,
{
    GLOBAL_STATE.gen_range(low, high)
}

pub struct SliceChooseIter<'a, T> {
    source: &'a [T],
    indices: alloc::vec::IntoIter<usize>,
}

impl<'a, T> Iterator for SliceChooseIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.indices.next().map(|ix| &self.source[ix])
    }
}

pub trait ChooseRandom<T> {
    #[inline]
    fn shuffle(&mut self) {
        self.shuffle_with_state(&GLOBAL_STATE)
    }
    #[inline]
    fn choose(&self) -> Option<&T> {
        self.choose_with_state(&GLOBAL_STATE)
    }
    #[inline]
    fn choose_mut(&mut self) -> Option<&mut T> {
        self.choose_mut_with_state(&GLOBAL_STATE)
    }
    #[inline]
    fn choose_multiple(&self, amount: usize) -> SliceChooseIter<T> {
        self.choose_multiple_with_state(&GLOBAL_STATE, amount)
    }

    fn shuffle_with_state(&mut self, state: &RandGenerator);
    fn choose_with_state(&self, state: &RandGenerator) -> Option<&T>;
    fn choose_mut_with_state(&mut self, state: &RandGenerator) -> Option<&mut T>;
    fn choose_multiple_with_state(
        &self,
        state: &RandGenerator,
        _amount: usize,
    ) -> SliceChooseIter<T>;
}

impl<T> ChooseRandom<T> for [T] {
    #[inline]
    fn shuffle_with_state(&mut self, state: &RandGenerator) {
        let mut fy = fy::FisherYates::default();

        fy.shuffle_with_state(state, self);
    }

    #[inline]
    fn choose_with_state(&self, state: &RandGenerator) -> Option<&T> {
        let ix = state.gen_range(0, self.len());
        self.get(ix)
    }

    #[inline]
    fn choose_mut_with_state(&mut self, state: &RandGenerator) -> Option<&mut T> {
        let ix = state.gen_range(0, self.len());
        self.get_mut(ix)
    }

    #[inline]
    fn choose_multiple_with_state(
        &self,
        state: &RandGenerator,
        amount: usize,
    ) -> SliceChooseIter<T> {
        let mut indices = (0..self.len())
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        indices.shuffle_with_state(state);
        indices.resize(amount, 0);

        SliceChooseIter {
            source: self,
            indices: indices.into_iter(),
        }
    }
}

#[cfg(feature = "rand")]
pub mod compat {
    pub struct QuadRandWithState<'a>(pub &'a crate::RandGenerator);

    impl<'a> rand::RngCore for QuadRandWithState<'a> {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.0.gen_range(0, u32::MAX)
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            self.0.gen_range(0, u64::MAX)
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for i in 0..dest.len() {
                dest[i] = self.0.gen_range(0, 255)
            }
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            Ok(self.fill_bytes(dest))
        }
    }

    pub struct QuadRand;

    impl rand::RngCore for QuadRand {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            QuadRandWithState(&crate::GLOBAL_STATE).next_u32()
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            QuadRandWithState(&crate::GLOBAL_STATE).next_u64()
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            QuadRandWithState(&crate::GLOBAL_STATE).fill_bytes(dest)
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            QuadRandWithState(&crate::GLOBAL_STATE).try_fill_bytes(dest)
        }
    }
}
