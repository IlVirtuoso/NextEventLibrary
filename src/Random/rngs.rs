/* porting of
 * Name            : rngs.c  (Random Number Generation - Multiple Streams)
 * Authors         : Steve Park & Dave Geyer
 * Language        : ANSI C
 * Latest Revision : 09-22-98
 * -------------------------------------------------------------------------
 * in Rust made by Matteo Ielacqua
 */

use std::time;

const MODULUS: i32 = 2147483647;
const MULTIPLIER: i32 = 48271; /* DON'T CHANGE THIS VALUE                  */
const CHECK: i32 = 399268537; /* DON'T CHANGE THIS VALUE                  */
const STREAMS: i32 = 256; /* # of streams, DON'T CHANGE THIS VALUE    */
const A256: i32 = 22925; /* jump multiplier, DON'T CHANGE THIS VALUE */
const DEFAULT: i32 = 123456789; /* initial seed, use 0 < DEFAULT < MODULUS  */

static mut seed: [i32; DEFAULT as usize] = [DEFAULT; DEFAULT as usize];
static mut stream: usize = 0;
static initialized: i32 = 0;

pub fn Random() -> f64 {
    const Q: i32 = MODULUS / MULTIPLIER;
    const R: i32 = MODULUS % MULTIPLIER;
    unsafe {
        //this is not safe in multithread
        let mut t: i32 = MULTIPLIER * (seed[stream] % Q) - R * (seed[stream] / Q);
        if t > 0 {
            seed[stream] = t;
        } else {
            seed[stream] = t + MODULUS;
        }
        return (seed[stream] / MODULUS) as f64;
    }
}

pub fn PlantSeeds(n: i32){
    const Q: i32 = MODULUS / MULTIPLIER;
    const R: i32 = MODULUS % MULTIPLIER;
    let mut x = n;
    unsafe {
        let mut s = stream;
        SelectStream(0);
        PutSeed(x);
        stream = s;
        for j in (1..STREAMS as usize) {
            x = A256*(seed[j-1]%Q) -R*(seed[j-1]/Q);
            seed[j]= if x>0{x} else {x+MODULUS}
        }
    }
}

pub fn PutSeed(n: i32) {
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
        seed[stream] = x;
    }
}

pub fn GetSeed() -> i32 {
    unsafe { seed[stream] }
}

pub fn SelectStream(index: usize) {
    unsafe {
        stream = index % STREAMS as usize;

        if initialized == 0 && stream != 0 {PlantSeeds(DEFAULT);}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Test_Random() {
        SelectStream(0);
        PutSeed(1);
        let mut u : f64 = 0.0;
        for _ in 0..10000{
            u = Random();
        }
        let mut x = GetSeed();
        let mut ok = x == CHECK;
        SelectStream(1);
        PlantSeeds(1);
        x = GetSeed();
        assert!( ok && (x == A256));

    }
}