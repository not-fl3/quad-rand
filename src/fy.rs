//! Implementation of Fisher-Yates algorithm.
//! This is modified version of https://github.com/adambudziak/shuffle/blob/master/src/fy.rs

/// Implementation of Fisher-Yates algorithm.
#[derive(Debug, Default)]
pub struct FisherYates {
    buffer: [u8; std::mem::size_of::<usize>()],
}

impl FisherYates {
    pub fn shuffle<T>(&mut self, data: &mut Vec<T>) {
        for i in 1..data.len() {
            let j = self.gen_range(i);
            data.swap(i, j);
        }
    }
}

impl FisherYates {
    fn gen_range(&mut self, top: usize) -> usize {
        const USIZE_BYTES: usize = std::mem::size_of::<usize>();
        let bit_width = USIZE_BYTES * 8 - top.leading_zeros() as usize;
        let byte_count = (bit_width - 1) / 8 + 1;
        loop {
            for i in 0..byte_count {
                self.buffer[i] = crate::gen_range(0, 255);
            }
            let result = usize::from_le_bytes(self.buffer);
            let result = result & ((1 << bit_width) - 1);
            if result < top {
                break result;
            }
        }
    }
}
