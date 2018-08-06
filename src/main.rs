extern crate rand;

use rand::prelude::*;

fn reservoir_sampling(data: i32, sample: &mut Vec<i32>, sample_size: usize, ith: usize) {
    let mut rng = thread_rng();
    let k = rng.gen_range(0, ith);
    if sample.len() < sample_size {
        sample.push(data);
    } else if k < sample_size {
        sample[k] = data;
    }
}

fn main() {
    let sample_size = 10;
    let mut sample = Vec::with_capacity(sample_size);
    let mut ith = 1;
    for i in 0..1000 {
        reservoir_sampling(i as i32, &mut sample, sample_size, ith);
        ith += 1;
    }
    println!("{:?}", sample);
}
