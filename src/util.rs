use rayon::prelude::*;

const PARALLEL_THRESHOLD: usize = 256 * 1024;
const CHUNK_SIZE: usize = 64 * 1024;

#[inline]
pub fn count_bytes(bytes: &[u8]) -> [usize; 256] {
    if bytes.len() < PARALLEL_THRESHOLD {
        let mut counts = [0usize; 256];
        for &b in bytes {
            counts[b as usize] += 1;
        }
        counts
    } else {
        bytes
            .par_chunks(CHUNK_SIZE)
            .fold(
                || [0usize; 256],
                |mut acc, chunk| {
                    for &b in chunk {
                        acc[b as usize] += 1;
                    }
                    acc
                },
            )
            .reduce(
                || [0usize; 256],
                |mut a, b| {
                    for i in 0..256 {
                        a[i] += b[i];
                    }
                    a
                },
            )
    }
}
