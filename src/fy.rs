//! Implementation of Fisher-Yates algorithm.
//! This is modified version of https://github.com/adambudziak/shuffle/blob/master/src/fy.rs

use crate::RandGenerator;
use core::mem::size_of;

/// Implementation of Fisher-Yates algorithm.
#[derive(Debug, Default)]
pub struct FisherYates {
    buffer: [u8; size_of::<usize>()],
}

impl FisherYates {
    pub fn shuffle_with_state<T>(&mut self, state: &RandGenerator, data: &mut [T]) {
        for i in 1..data.len() {
            let j = self.gen_range(state, i);
            data.swap(i, j);
        }
    }
}

impl FisherYates {
    fn gen_range(&mut self, state: &RandGenerator, top: usize) -> usize {
        const USIZE_BYTES: usize = size_of::<usize>();
        let bit_width = USIZE_BYTES * 8 - top.leading_zeros() as usize;
        let byte_count = (bit_width - 1) / 8 + 1;
        loop {
            for i in 0..byte_count {
                self.buffer[i] = state.gen_range(0, 255);
            }
            let result = usize::from_le_bytes(self.buffer);
            let result = result & ((1 << bit_width) - 1);
            if result < top {
                break result;
            }
        }
    }
}
