use std::time::Instant;
use tinyrand::{Rand, Seeded, StdRand};

fn main() {
    let start_time = Instant::now();
    const SAMPLES: usize = 1_000;
    const ITERS: usize = 1_000_000;
    let mut samples = (0..SAMPLES).map(|sample| {
        let mut rand = StdRand::seed(sample as u64);
        gen_pi(ITERS, &mut rand)
        
    }).collect::<Vec<_>>();
    samples.sort_unstable_by(|a, b| a.total_cmp(b));
    
    let min = quantile(&samples, 0.0);
    let p025 = quantile(&samples, 0.025);
    let p05 = quantile(&samples, 0.05);
    let median = quantile(&samples, 0.5);
    let p95 = quantile(&samples, 0.95);
    let p975 = quantile(&samples, 0.975);
    let max = quantile(&samples, 1.0);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    
    println!("min: {min:.9}, median: {median:.9}, max: {max:.9}");
    println!("mean: {mean:.9}");
    println!("CI (p=0.1):  [{p05:.9}, {p95:.9}]");
    println!("CI (p=0.05): [{p025:.9}, {p975:.9}]");
    println!("took {:.1} s", start_time.elapsed().as_millis() as f64 / 1000.0);
    println!("true value: {}", std::f64::consts::PI);
}

fn gen_pi(iters: usize, rand: &mut impl Rand) -> f64 {
    let mut count = 0_u64;
    for _ in 0..iters {
        let x = rand.next_u64() as f64 / u64::MAX as f64;
        let y = rand.next_u64() as f64 / u64::MAX as f64;
        if x * x + y * y <= 1.0 {
            count += 1;
        }
    }
    let area = count as f64 / iters as f64;
    area * 4.0
}

pub fn quantile<T>(ordered: &[T], quantile: f64) -> &T {
    let index = (quantile * ordered.len() as f64) as usize;
    let index = if index == 0 { 0 } else { index - 1 };
    &ordered[index]
}
