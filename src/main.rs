use std::env;
use std::io::{self, Read};

fn main() {
    // --- CLI options: --bins N, --width W ---
    let mut bins: usize = 10;
    let mut width: usize = 50;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bins" => {
                if let Some(v) = args.next() {
                    bins = v.parse().unwrap_or(bins);
                }
            }
            "--width" => {
                if let Some(v) = args.next() {
                    width = v.parse().unwrap_or(width);
                }
            }
            "--help" | "-h" => {
                eprintln!("Usage: cat data.txt | histoterm [--bins N] [--width W]");
                eprintln!(
                    "Reads whitespace-separated numbers from stdin and prints an ASCII histogram."
                );
                return;
            }
            _ => {}
        }
    }
    if bins == 0 {
        eprintln!("--bins must be > 0");
        return;
    }
    if width == 0 {
        eprintln!("--width must be > 0");
        return;
    }

    // --- Read stdin ---
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("read stdin");
    let mut data: Vec<f64> = Vec::new();
    for tok in buf.split_whitespace() {
        if let Ok(x) = tok.parse::<f64>() {
            if x.is_finite() {
                data.push(x);
            }
        }
    }
    if data.is_empty() {
        eprintln!("No numeric data found on stdin.");
        return;
    }

    // --- Basic stats ---
    let n = data.len() as f64;
    let (mut min, mut max) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut sum, mut sumsq) = (0.0, 0.0);
    for &x in &data {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
        sum += x;
        sumsq += x * x;
    }
    let mean = sum / n;
    let var = (sumsq / n) - (mean * mean);
    let std = if var > 0.0 { var.sqrt() } else { 0.0 };

    // --- Edge case: all values identical ---
    if (max - min).abs() < f64::EPSILON {
        println!("All values are the same: {:.6}", min);
        println!(
            "count = {}, min = max = {:.6}, mean = {:.6}, std = {:.6}",
            data.len(),
            min,
            mean,
            std
        );
        println!("\n[{}] {}", "#".repeat(width), data.len());
        return;
    }

    // --- Build histogram ---
    let range = max - min;
    let bin_w = range / bins as f64;
    let mut counts = vec![0usize; bins];

    for &x in &data {
        let mut idx = ((x - min) / bin_w).floor() as usize;
        if idx >= bins {
            idx = bins - 1;
        } // include max in last bin
        counts[idx] += 1;
    }
    let max_count = *counts.iter().max().unwrap().max(&1);

    // --- Pretty print ---
    println!("ASCII Histogram  (bins={}, width={})", bins, width);
    println!(
        "count = {}, min = {:.6}, max = {:.6}, mean = {:.6}, std = {:.6}\n",
        data.len(),
        min,
        max,
        mean,
        std
    );

    for (i, &c) in counts.iter().enumerate() {
        let lo = min + i as f64 * bin_w;
        let hi = if i + 1 == bins {
            max
        } else {
            min + (i + 1) as f64 * bin_w
        };
        let bar_len = (c * width) / max_count;
        let pct = (c as f64) * 100.0 / (data.len() as f64);
        println!(
            "[{:<10.4} , {:>10.4}) | {:>6} ({:>5.1}%) | {}",
            lo,
            hi,
            c,
            pct,
            "#".repeat(bar_len)
        );
    }
}
