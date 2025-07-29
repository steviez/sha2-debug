#![feature(test)]
extern crate test;

use test::Bencher;
use sha2::{Digest, Sha256};

const NUM_HASHES: u64 = 32_768;

#[bench]
fn bench_sha256_hash(bencher: &mut Bencher) {
    let mut data = [0u8; 32];
    bencher.iter(|| {
        for _ in 0..NUM_HASHES {
            let mut hasher = Sha256::default();
            hasher.update(&data);
            data = hasher.finalize().into();
        }
    })
}