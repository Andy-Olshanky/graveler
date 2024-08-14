use std::{time::Instant};
use tinyrand::{Rand, StdRand, Seeded};
use tinyrand_std::{clock_seed::ClockSeed};

fn main() {
    let mut num_0s;
    let mut num_rolls = 0;
    let mut max_0s = 0;
    let max_rolls = 1000000000;

    let seed = ClockSeed::default().next_u64();
    let mut rng = StdRand::seed(seed);
    
    let start = Instant::now();
    
    while max_0s < 177 && num_rolls < max_rolls {
        num_0s = 0;
        for _ in 0..231 {
            let num = rng.next_u16() % 4;
            if num == 0 {
                num_0s += 1;
            }
        }
        num_rolls += 1;
        if num_0s > max_0s {
            max_0s = num_0s;
        }
    }

    let duration = start.elapsed();

    println!("Highest number of paralysis procs rolled: {}", max_0s);
    println!("Number of rolls simulated: {}", num_rolls);
    println!("Time elapsed: {:?}", duration);
}
