use std::{
    sync::{
        atomic::{AtomicU16, AtomicU64, Ordering::Relaxed},
        Arc,
    },
    thread,
    time::Instant,
};
use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::clock_seed::ClockSeed;

fn main() {
    let max_rolls = 1000000000;
    let num_threads = 32;

    let total_rolls = Arc::new(AtomicU64::new(0));
    let max_0s = Arc::new(AtomicU16::new(0));
    let should_stop = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    let start = Instant::now();

    for _ in 0..num_threads {
        let total_rolls = Arc::clone(&total_rolls);
        let max_0s = Arc::clone(&max_0s);
        let should_stop = Arc::clone(&should_stop);

        let handle = thread::spawn(move || {
            let seed = ClockSeed::default().next_u64();
            let mut rng = StdRand::seed(seed);

            while should_stop.load(Relaxed) == 0 {
                let mut num_0s = 0;
                for _ in 0..231 {
                    let num = rng.next_u16() % 4;
                    if num == 0 {
                        num_0s += 1;
                    }
                }

                total_rolls.fetch_add(1, Relaxed);

                let current_max = max_0s.load(Relaxed);
                if num_0s > current_max {
                    max_0s.store(num_0s, Relaxed);
                    if num_0s > 177 {
                        should_stop.store(1, Relaxed);
                        break;
                    }
                }

                if total_rolls.load(Relaxed) >= max_rolls {
                    should_stop.store(1, Relaxed);
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();

    println!(
        "Highest number of paralysis procs rolled: {}",
        max_0s.load(Relaxed)
    );
    println!("Number of rolls simulated: {}", total_rolls.load(Relaxed));
    println!("Time elapsed: {:?}", duration);
}
