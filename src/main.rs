use rust_pi::sample::quantile;
use rust_pi::{dkw, ConfidenceInterval};
use std::time::Instant;
use tinyrand::{Rand, Seeded, StdRand};

fn main() {
    let start_time = Instant::now();
    const SAMPLE_SIZE: usize = 1_000;
    const ITERS: usize = 1_000_000;
    let mut sample = (0..SAMPLE_SIZE).map(|sample| {
        let mut rand = StdRand::seed(sample as u64);
        gen_pi(ITERS, &mut rand)
        
    }).collect::<Vec<_>>();
    sample.sort_unstable_by(|a, b| a.total_cmp(b));
    
    let min = *quantile(&sample, 0.0);
    let p025 = *quantile(&sample, 0.025);
    let p05 = *quantile(&sample, 0.05);
    let median = *quantile(&sample, 0.5);
    let p95 = *quantile(&sample, 0.95);
    let p975 = *quantile(&sample, 0.975);
    let max = *quantile(&sample, 1.0);
    let mean = sample.iter().sum::<f64>() / sample.len() as f64;
    
    println!("min: {min:.9}, median: {median:.9}, max: {max:.9}");
    println!("mean: {mean:.9}");

    println!("unadjusted nonparametric:");
    let unadj_ci_10 = ConfidenceInterval { lower: p05, upper: p95 };
    let unadj_ci_05 = ConfidenceInterval { lower: p025, upper: p975 };
    println!("  CI (α=0.1):  {unadj_ci_10}");
    println!("  CI (α=0.05): {unadj_ci_05}");

    println!("Dvoretzky–Kiefer–Wolfowitz nonparametric:");
    let dkw_ci_10 = dkw::ci(&sample, 0.1);
    let dkw_ci_05 = dkw::ci(&sample, 0.05);
    println!("  CI (α=0.1):  {dkw_ci_10}");
    println!("  CI (α=0.05): {dkw_ci_05}");
    
    println!("took {:.1} s", start_time.elapsed().as_millis() as f64 / 1000.0);
    println!("reference Pi: {}", std::f64::consts::PI);
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