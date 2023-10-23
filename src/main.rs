use std::time::Instant;
use tinyrand::{Rand, StdRand};

fn main() {
    let start_time = Instant::now();
    let pi = gen_pi();
    let elapsed = start_time.elapsed();
    println!("pi = {pi}, took {:.1} s", elapsed.as_millis() as f64 / 1000.0);
}

// fn gen_pi() -> f64 {
//     let dim = 10000_u64;
//     let mut count = 0_u64;
//     for i in 0..=dim {
//         let x = i as f64 / dim as f64;
//         for j in 0..=dim {
//             let y = j as f64 / dim as f64;
//             if (x.powi(2) + y.powi(2)).sqrt() <= 1.0 {
//                 count += 1;
//             }
//         }
//     }
//     let area = count as f64 / dim.pow(2) as f64;
//     area * 4.0
// }

// fn gen_pi() -> f64 {
//     let dim = 10000_u64;
//     let mut count = 0_u64;
//     for i in 0..=dim {
//         for j in 0..=dim {
//             if ((i.pow(2) + j.pow(2)) as f64).sqrt() <= dim as f64 {
//                 count += 1;
//             }
//         }
//     }
//     let area = count as f64 / dim.pow(2) as f64;
//     area * 4.0
// }

fn gen_pi() -> f64 {
    let mut rand = StdRand::default();

    let mut count = 0_u64;
    const ITERS: u64 = 1_000_000_000;
    for _ in 0..ITERS {
        let x = rand.next_u64() as f64 / u64::MAX as f64;
        let y = rand.next_u64() as f64 / u64::MAX as f64;
        if x * x + y * y <= 1.0 {
            count += 1;
        }
    }
    let area = count as f64 / ITERS as f64;
    area * 4.0
}
