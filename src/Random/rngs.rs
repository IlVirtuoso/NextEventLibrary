/* porting of
 * Name            : rngs.c  (Random Number Generation - Multiple Streams)
 * Authors         : Steve Park & Dave Geyer
 * Language        : ANSI C
 * Latest Revision : 09-22-98
 * -------------------------------------------------------------------------
 * in Rust made by Matteo Ielacqua
 */

use once_cell::unsync::Lazy;
use std::{alloc::Layout, time};
const MODULUS: i32 = 2147483647;
const MULTIPLIER: i32 = 48271; /* DON'T CHANGE THIS VALUE                  */
const CHECK: i32 = 399268537; /* DON'T CHANGE THIS VALUE                  */
const STREAMS: i32 = 256; /* # of streams, DON'T CHANGE THIS VALUE    */
const A256: i32 = 22925; /* jump multiplier, DON'T CHANGE THIS VALUE */
const DEFAULT: i32 = 123456789; /* initial seed, use 0 < DEFAULT < MODULUS  */

pub struct RandomGenerator {
    seed: Box<[i32]>,
    stream: usize,
    initialized: i32,
}

impl RandomGenerator {
    pub fn new() -> Self {
        RandomGenerator {
            seed: vec![DEFAULT;DEFAULT as usize].into_boxed_slice(),
            stream: 0,
            initialized: 0,
        }
    }

    pub fn Global() -> &'static mut Self {
        static mut instance: Lazy<RandomGenerator> = Lazy::new(|| RandomGenerator::new());
        unsafe { &mut instance }
    }

    pub fn Random(&mut self) -> f64 {
        const Q: f64 = MODULUS / MULTIPLIER;
        const R: f64 = MODULUS % MULTIPLIER;
        unsafe {
            //this is not safe in multithread
            let mut t: i32 =
                MULTIPLIER * (self.seed[self.stream] as f64 % Q) - R * (self.seed[self.stream] as f64 / Q);
            if t > 0 {
                self.seed[self.stream] = t;
            } else {
                self.seed[self.stream] = t + MODULUS;
            }
            return (self.seed[self.stream] / MODULUS) as f64;
        }
    }

    pub fn PlantSeeds(&mut self, n: i32) {
        const Q: i32 = MODULUS / MULTIPLIER;
        const R: i32 = MODULUS % MULTIPLIER;
        let mut x = n;
        unsafe {
            self.initialized = 1;
            let mut s = self.stream;
            self.SelectStream(0);
            self.PutSeed(x);
            self.stream = s;
            for j in (1..STREAMS as usize) {
                x = A256 * (self.seed[j - 1] % Q) - R * (self.seed[j - 1] / Q);
                self.seed[j] = if x > 0 { x } else { x + MODULUS }
            }
        }
    }

    pub fn PutSeed(&mut self, n: i32) {
        let mut ok: bool = false;
        let mut x: i32 = {
            if n > 0 {
                n % MODULUS
            } else {
                time::SystemTime::now()
                    .duration_since(time::SystemTime::UNIX_EPOCH)
                    .expect("Error in catching system time")
                    .as_millis() as i32
            }
        };

        unsafe {
            self.seed[self.stream] = x;
        }
    }

    pub fn GetSeed(&self) -> i32 {
        unsafe { self.seed[self.stream] }
    }

    pub fn SelectStream(&mut self, index: usize) {
        unsafe {
            self.stream = index % STREAMS as usize;

            if self.initialized == 0 && self.stream != 0 {
                self.PlantSeeds(DEFAULT);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Test_Random() {
        let mut gen = RandomGenerator::new();
        gen.SelectStream(0);
        gen.PutSeed(1);
        let mut u: f64 = 0.0;
        for _ in 0..10000 {
            u = gen.Random();
        }
        let mut x = gen.GetSeed();
        let mut ok = x == CHECK;
        gen.SelectStream(1);
        gen.PlantSeeds(1);
        x = gen.GetSeed();
        assert!(ok && (x == A256));
    }
}
