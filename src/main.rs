use clap::Parser;
use rust_pi::sample::{quantile, SummaryStatistics};
use rust_pi::{dkw, normal, ConfidenceInterval};
use std::time::Instant;
use tinyrand::{Rand, Seeded, StdRand};

#[derive(Debug, clap::Parser, Clone)]
struct Args {
    /// sample size
    #[clap(long = "sample-size", default_value = "1000")]
    sample_size: usize,

    /// trials
    #[clap(long = "trials", default_value = "1000000")]
    trials: usize,
}

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let mut sample = (0..args.sample_size)
        .map(|sample| {
            let mut rand = StdRand::seed(sample as u64);
            gen_pi(args.trials, &mut rand)
        })
        .collect::<Vec<_>>();
    sample.sort_unstable_by(|a, b| a.total_cmp(b));

    let p025 = *quantile(&sample, 0.025);
    let p05 = *quantile(&sample, 0.05);
    let median = *quantile(&sample, 0.5);
    let p95 = *quantile(&sample, 0.95);
    let p975 = *quantile(&sample, 0.975);

    println!(
        "n: {}, trials: {}, took {:.1} s",
        args.sample_size,
        args.trials,
        start_time.elapsed().as_millis() as f64 / 1000.0
    );
    let sample_stats = SummaryStatistics::from(&*sample);
    println!(
        "min: {:.9}, median: {median:.9}, max: {:.9}",
        sample_stats.min, sample_stats.max
    );
    println!(
        "µ: {:.9}, σ: {:.9}",
        sample_stats.mean, sample_stats.std_dev
    );
    println!("reference π: {}", std::f64::consts::PI);

    println!();
    println!("normal:");
    let norm_ci_10 = normal::ci(
        sample_stats.mean,
        sample_stats.std_dev,
        args.sample_size,
        0.1,
    );
    let norm_ci_05 = normal::ci(
        sample_stats.mean,
        sample_stats.std_dev,
        args.sample_size,
        0.05,
    );
    println!("  CI (α=0.1):  {norm_ci_10:.9}");
    println!("  CI (α=0.05): {norm_ci_05:.9}");

    println!();
    println!("Unadjusted nonparametric:");
    let unadj_ci_10 = ConfidenceInterval {
        lower: p05,
        upper: p95,
    };
    let unadj_ci_05 = ConfidenceInterval {
        lower: p025,
        upper: p975,
    };
    println!("  CI (α=0.1):  {unadj_ci_10:.9}");
    println!("  CI (α=0.05): {unadj_ci_05:.9}");

    println!();
    println!("Dvoretzky–Kiefer–Wolfowitz nonparametric:");
    if sample.len() > 1 {
        let dkw_ci_10 = dkw::ci(&sample, 0.1);
        let dkw_ci_05 = dkw::ci(&sample, 0.05);
        println!("  CI (α=0.1):  {dkw_ci_10:.9}");
        println!("  CI (α=0.05): {dkw_ci_05:.9}");
    } else {
        println!("  insufficient sample size for a confidence interval")
    }
}

fn gen_pi(trials: usize, rand: &mut impl Rand) -> f64 {
    let mut inside_unit_radius = 0_u64;
    for _ in 0..trials {
        let x = rand.next_u64() as f64 / u64::MAX as f64;
        let y = rand.next_u64() as f64 / u64::MAX as f64;
        if x * x + y * y <= 1.0 {
            inside_unit_radius += 1;
        }
    }
    let area = inside_unit_radius as f64 / trials as f64;
    area * 4.0
}
