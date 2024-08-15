# Graveler Softlock Picking coding challenge

In this ShoddyCast video (https://www.youtube.com/watch?v=M8C8dHQE2Ro&list=LL&index=1) some... well made and very optimized python code was written (https://github.com/arhourigan/graveler/blob/main/graveler.py) to run statistical simulations to see how likely a certain challenge in Pokemon from the YouTuber Pikasprey is to happen

At the end of the video, a challenge was given to see how efficient people in the community can make the code and see how fast it can run, here's my attempt

I am using Rust and my CPU is a Ryzen 9 7950X3D with 32GB DDR5 RAM, let's see what we can do with this

My CPU has 32 threads so my efficiency peaked at that number of parallel processes, so that's how many threads I created in this program

## Result
My fastest simulation took 18.736 seconds to execute 1 billion simulations

To run for yourself, install Rust and run the following commands:
```
git clone https://github.com/Andy-Olshanky/graveler.git
cd graveler
cargo build --release
./target/release/graveler
```
