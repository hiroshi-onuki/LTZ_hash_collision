use std::fs::{create_dir_all, File};
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

use crate::{
    enumerate_igusa_auto,
    find_non_residue,
    find_non_residue_fp2,
};

const PRIMES: [u64; 22] = [
    7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
    83, 89, 97,
];

fn enumerate_prime(p: u64) -> Option<(Vec<String>, usize, Duration)> {
    match p {
        7 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(7);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        11 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(11);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        13 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(13);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        17 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(17);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        19 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(19);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        23 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(23);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        29 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(29);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        31 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(31);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        37 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(37);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        41 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(41);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        43 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(43);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        47 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(47);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        53 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(53);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        59 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(59);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        61 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(61);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        67 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(67);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        71 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(71);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        73 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(73);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        79 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(79);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        83 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(83);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        89 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(89);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        97 => {
            let start = Instant::now();
            let (vals, count) = enumerate_igusa_auto!(97);
            let dur = start.elapsed();
            let strings = vals.iter().map(|v| format!("[{}, {}, {}]", v.0, v.1, v.2)).collect();
            Some((strings, count, dur))
        },
        _ => None,
    }
}

pub fn handle_count_single(p: u64, file_name: &str) -> io::Result<()> {
    if !PRIMES.contains(&p) {
        eprintln!("unsupported prime {}", p);
        return Ok(());
    }
    if let Some((invs, fp2_count, dur)) = enumerate_prime(p) {
        let d1 = find_non_residue(p);
        let d2 = find_non_residue_fp2(p, d1);
        let mut file = File::create(file_name)?;
        writeln!(file, "p := {};", p);
        writeln!(file, "K := GF(p^4);");
        writeln!(file, "R<z> := PolynomialRing(K);");
        writeln!(file, "x := Roots(z^2 - {})[1][1];", d1);
        writeln!(file, "y := Roots(z^2 - {} - {}*x)[1][1];", d2.c0, d2.c1);
        println!("Fp2 invariants = {}, time = {:.2}s", fp2_count, dur.as_secs_f64());
        writeln!(file, "\ninv_list := [");
        for (idx, s) in invs.iter().enumerate() {
            if idx < fp2_count - 1 {
                writeln!(file, "\t{},", s)?;
            } else {
                writeln!(file, "\t{}", s)?;
            }
        }
        writeln!(file, "];");
    }
    Ok(())
}

pub fn handle_count_all(dir: &str) -> io::Result<()> {
    create_dir_all(dir)?;
    for p in PRIMES.iter() {
        let path = format!("{}/fp2_invariants_p{}.m", dir, p);
        handle_count_single(*p, &path)?;
    }
    Ok(())
}