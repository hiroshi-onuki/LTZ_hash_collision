use std::fs::{create_dir_all, File};
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

use crate::{
    collision::collision_depth, find_collision_auto, find_non_residue, find_non_residue_fp2,
    LTZ_hash_auto,
};

pub fn handle_hash(p: u64, input: &str) {
    if !input.chars().all(|c| c.is_digit(8)) {
        eprintln!("invalid octal string");
        return;
    }
    match p {
        103 => {
            let res = LTZ_hash_auto!(103, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        1013 => {
            let res = LTZ_hash_auto!(1013, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        1033 => {
            let res = LTZ_hash_auto!(1033, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3253 => {
            let res = LTZ_hash_auto!(3253, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3257 => {
            let res = LTZ_hash_auto!(3257, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3307 => {
            let res = LTZ_hash_auto!(3307, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3313 => {
            let res = LTZ_hash_auto!(3313, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3323 => {
            let res = LTZ_hash_auto!(3323, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3343 => {
            let res = LTZ_hash_auto!(3343, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3347 => {
            let res = LTZ_hash_auto!(3347, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3373 => {
            let res = LTZ_hash_auto!(3373, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3407 => {
            let res = LTZ_hash_auto!(3407, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3413 => {
            let res = LTZ_hash_auto!(3413, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3433 => {
            let res = LTZ_hash_auto!(3433, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3457 => {
            let res = LTZ_hash_auto!(3457, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3463 => {
            let res = LTZ_hash_auto!(3463, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3467 => {
            let res = LTZ_hash_auto!(3467, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3517 => {
            let res = LTZ_hash_auto!(3517, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3527 => {
            let res = LTZ_hash_auto!(3527, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3533 => {
            let res = LTZ_hash_auto!(3533, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3547 => {
            let res = LTZ_hash_auto!(3547, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3557 => {
            let res = LTZ_hash_auto!(3557, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3583 => {
            let res = LTZ_hash_auto!(3583, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3593 => {
            let res = LTZ_hash_auto!(3593, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3607 => {
            let res = LTZ_hash_auto!(3607, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3613 => {
            let res = LTZ_hash_auto!(3613, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3617 => {
            let res = LTZ_hash_auto!(3617, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3623 => {
            let res = LTZ_hash_auto!(3623, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3637 => {
            let res = LTZ_hash_auto!(3637, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3643 => {
            let res = LTZ_hash_auto!(3643, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3673 => {
            let res = LTZ_hash_auto!(3673, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3677 => {
            let res = LTZ_hash_auto!(3677, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3697 => {
            let res = LTZ_hash_auto!(3697, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3727 => {
            let res = LTZ_hash_auto!(3727, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3733 => {
            let res = LTZ_hash_auto!(3733, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3767 => {
            let res = LTZ_hash_auto!(3767, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3793 => {
            let res = LTZ_hash_auto!(3793, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3797 => {
            let res = LTZ_hash_auto!(3797, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3803 => {
            let res = LTZ_hash_auto!(3803, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3823 => {
            let res = LTZ_hash_auto!(3823, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3833 => {
            let res = LTZ_hash_auto!(3833, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3847 => {
            let res = LTZ_hash_auto!(3847, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3853 => {
            let res = LTZ_hash_auto!(3853, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3863 => {
            let res = LTZ_hash_auto!(3863, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3877 => {
            let res = LTZ_hash_auto!(3877, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3907 => {
            let res = LTZ_hash_auto!(3907, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3917 => {
            let res = LTZ_hash_auto!(3917, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3923 => {
            let res = LTZ_hash_auto!(3923, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3943 => {
            let res = LTZ_hash_auto!(3943, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3947 => {
            let res = LTZ_hash_auto!(3947, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        3967 => {
            let res = LTZ_hash_auto!(3967, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4003 => {
            let res = LTZ_hash_auto!(4003, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4007 => {
            let res = LTZ_hash_auto!(4007, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4013 => {
            let res = LTZ_hash_auto!(4013, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4027 => {
            let res = LTZ_hash_auto!(4027, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4057 => {
            let res = LTZ_hash_auto!(4057, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4073 => {
            let res = LTZ_hash_auto!(4073, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4093 => {
            let res = LTZ_hash_auto!(4093, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4127 => {
            let res = LTZ_hash_auto!(4127, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4133 => {
            let res = LTZ_hash_auto!(4133, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4153 => {
            let res = LTZ_hash_auto!(4153, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4157 => {
            let res = LTZ_hash_auto!(4157, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4177 => {
            let res = LTZ_hash_auto!(4177, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4217 => {
            let res = LTZ_hash_auto!(4217, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4243 => {
            let res = LTZ_hash_auto!(4243, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4253 => {
            let res = LTZ_hash_auto!(4253, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4273 => {
            let res = LTZ_hash_auto!(4273, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4283 => {
            let res = LTZ_hash_auto!(4283, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4297 => {
            let res = LTZ_hash_auto!(4297, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4327 => {
            let res = LTZ_hash_auto!(4327, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4337 => {
            let res = LTZ_hash_auto!(4337, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4357 => {
            let res = LTZ_hash_auto!(4357, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4363 => {
            let res = LTZ_hash_auto!(4363, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4373 => {
            let res = LTZ_hash_auto!(4373, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4397 => {
            let res = LTZ_hash_auto!(4397, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4423 => {
            let res = LTZ_hash_auto!(4423, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4447 => {
            let res = LTZ_hash_auto!(4447, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4457 => {
            let res = LTZ_hash_auto!(4457, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4463 => {
            let res = LTZ_hash_auto!(4463, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4483 => {
            let res = LTZ_hash_auto!(4483, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4493 => {
            let res = LTZ_hash_auto!(4493, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4507 => {
            let res = LTZ_hash_auto!(4507, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4513 => {
            let res = LTZ_hash_auto!(4513, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4517 => {
            let res = LTZ_hash_auto!(4517, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4523 => {
            let res = LTZ_hash_auto!(4523, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4547 => {
            let res = LTZ_hash_auto!(4547, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4567 => {
            let res = LTZ_hash_auto!(4567, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4583 => {
            let res = LTZ_hash_auto!(4583, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4597 => {
            let res = LTZ_hash_auto!(4597, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4603 => {
            let res = LTZ_hash_auto!(4603, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4637 => {
            let res = LTZ_hash_auto!(4637, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4643 => {
            let res = LTZ_hash_auto!(4643, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4657 => {
            let res = LTZ_hash_auto!(4657, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4663 => {
            let res = LTZ_hash_auto!(4663, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4673 => {
            let res = LTZ_hash_auto!(4673, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4703 => {
            let res = LTZ_hash_auto!(4703, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4723 => {
            let res = LTZ_hash_auto!(4723, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4733 => {
            let res = LTZ_hash_auto!(4733, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4783 => {
            let res = LTZ_hash_auto!(4783, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4787 => {
            let res = LTZ_hash_auto!(4787, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4793 => {
            let res = LTZ_hash_auto!(4793, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4813 => {
            let res = LTZ_hash_auto!(4813, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4817 => {
            let res = LTZ_hash_auto!(4817, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4877 => {
            let res = LTZ_hash_auto!(4877, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4903 => {
            let res = LTZ_hash_auto!(4903, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4933 => {
            let res = LTZ_hash_auto!(4933, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4937 => {
            let res = LTZ_hash_auto!(4937, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4943 => {
            let res = LTZ_hash_auto!(4943, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4957 => {
            let res = LTZ_hash_auto!(4957, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4967 => {
            let res = LTZ_hash_auto!(4967, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4973 => {
            let res = LTZ_hash_auto!(4973, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4987 => {
            let res = LTZ_hash_auto!(4987, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        4993 => {
            let res = LTZ_hash_auto!(4993, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5003 => {
            let res = LTZ_hash_auto!(5003, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5023 => {
            let res = LTZ_hash_auto!(5023, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5077 => {
            let res = LTZ_hash_auto!(5077, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5087 => {
            let res = LTZ_hash_auto!(5087, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5107 => {
            let res = LTZ_hash_auto!(5107, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5113 => {
            let res = LTZ_hash_auto!(5113, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5147 => {
            let res = LTZ_hash_auto!(5147, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        5153 => {
            let res = LTZ_hash_auto!(5153, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        _ => {
            eprintln!("unsupported prime {}", p);
            return;
        }
    }
}

pub fn handle_collision(p: u64) {
    let start = Instant::now();
    match p {
        103 => {
            const DEPTH: usize = collision_depth(103);
            let (m1, m2) = find_collision_auto!(103, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(103, &m1);
            let h2 = LTZ_hash_auto!(103, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        1013 => {
            const DEPTH: usize = collision_depth(1013);
            let (m1, m2) = find_collision_auto!(1013, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(1013, &m1);
            let h2 = LTZ_hash_auto!(1013, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        1033 => {
            const DEPTH: usize = collision_depth(1033);
            let (m1, m2) = find_collision_auto!(1033, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(1033, &m1);
            let h2 = LTZ_hash_auto!(1033, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3253 => {
            const DEPTH: usize = collision_depth(3253);
            let (m1, m2) = find_collision_auto!(3253, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3253, &m1);
            let h2 = LTZ_hash_auto!(3253, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3257 => {
            const DEPTH: usize = collision_depth(3257);
            let (m1, m2) = find_collision_auto!(3257, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3257, &m1);
            let h2 = LTZ_hash_auto!(3257, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3307 => {
            const DEPTH: usize = collision_depth(3307);
            let (m1, m2) = find_collision_auto!(3307, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3307, &m1);
            let h2 = LTZ_hash_auto!(3307, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3313 => {
            const DEPTH: usize = collision_depth(3313);
            let (m1, m2) = find_collision_auto!(3313, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3313, &m1);
            let h2 = LTZ_hash_auto!(3313, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3323 => {
            const DEPTH: usize = collision_depth(3323);
            let (m1, m2) = find_collision_auto!(3323, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3323, &m1);
            let h2 = LTZ_hash_auto!(3323, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3343 => {
            const DEPTH: usize = collision_depth(3343);
            let (m1, m2) = find_collision_auto!(3343, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3343, &m1);
            let h2 = LTZ_hash_auto!(3343, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3347 => {
            const DEPTH: usize = collision_depth(3347);
            let (m1, m2) = find_collision_auto!(3347, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3347, &m1);
            let h2 = LTZ_hash_auto!(3347, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3373 => {
            const DEPTH: usize = collision_depth(3373);
            let (m1, m2) = find_collision_auto!(3373, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3373, &m1);
            let h2 = LTZ_hash_auto!(3373, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3407 => {
            const DEPTH: usize = collision_depth(3407);
            let (m1, m2) = find_collision_auto!(3407, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3407, &m1);
            let h2 = LTZ_hash_auto!(3407, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3413 => {
            const DEPTH: usize = collision_depth(3413);
            let (m1, m2) = find_collision_auto!(3413, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3413, &m1);
            let h2 = LTZ_hash_auto!(3413, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3433 => {
            const DEPTH: usize = collision_depth(3433);
            let (m1, m2) = find_collision_auto!(3433, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3433, &m1);
            let h2 = LTZ_hash_auto!(3433, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3457 => {
            const DEPTH: usize = collision_depth(3457);
            let (m1, m2) = find_collision_auto!(3457, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3457, &m1);
            let h2 = LTZ_hash_auto!(3457, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3463 => {
            const DEPTH: usize = collision_depth(3463);
            let (m1, m2) = find_collision_auto!(3463, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3463, &m1);
            let h2 = LTZ_hash_auto!(3463, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3467 => {
            const DEPTH: usize = collision_depth(3467);
            let (m1, m2) = find_collision_auto!(3467, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3467, &m1);
            let h2 = LTZ_hash_auto!(3467, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3517 => {
            const DEPTH: usize = collision_depth(3517);
            let (m1, m2) = find_collision_auto!(3517, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3517, &m1);
            let h2 = LTZ_hash_auto!(3517, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3527 => {
            const DEPTH: usize = collision_depth(3527);
            let (m1, m2) = find_collision_auto!(3527, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3527, &m1);
            let h2 = LTZ_hash_auto!(3527, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3533 => {
            const DEPTH: usize = collision_depth(3533);
            let (m1, m2) = find_collision_auto!(3533, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3533, &m1);
            let h2 = LTZ_hash_auto!(3533, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3547 => {
            const DEPTH: usize = collision_depth(3547);
            let (m1, m2) = find_collision_auto!(3547, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3547, &m1);
            let h2 = LTZ_hash_auto!(3547, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3557 => {
            const DEPTH: usize = collision_depth(3557);
            let (m1, m2) = find_collision_auto!(3557, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3557, &m1);
            let h2 = LTZ_hash_auto!(3557, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3583 => {
            const DEPTH: usize = collision_depth(3583);
            let (m1, m2) = find_collision_auto!(3583, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3583, &m1);
            let h2 = LTZ_hash_auto!(3583, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3593 => {
            const DEPTH: usize = collision_depth(3593);
            let (m1, m2) = find_collision_auto!(3593, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3593, &m1);
            let h2 = LTZ_hash_auto!(3593, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3607 => {
            const DEPTH: usize = collision_depth(3607);
            let (m1, m2) = find_collision_auto!(3607, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3607, &m1);
            let h2 = LTZ_hash_auto!(3607, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3613 => {
            const DEPTH: usize = collision_depth(3613);
            let (m1, m2) = find_collision_auto!(3613, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3613, &m1);
            let h2 = LTZ_hash_auto!(3613, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3617 => {
            const DEPTH: usize = collision_depth(3617);
            let (m1, m2) = find_collision_auto!(3617, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3617, &m1);
            let h2 = LTZ_hash_auto!(3617, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3623 => {
            const DEPTH: usize = collision_depth(3623);
            let (m1, m2) = find_collision_auto!(3623, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3623, &m1);
            let h2 = LTZ_hash_auto!(3623, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3637 => {
            const DEPTH: usize = collision_depth(3637);
            let (m1, m2) = find_collision_auto!(3637, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3637, &m1);
            let h2 = LTZ_hash_auto!(3637, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3643 => {
            const DEPTH: usize = collision_depth(3643);
            let (m1, m2) = find_collision_auto!(3643, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3643, &m1);
            let h2 = LTZ_hash_auto!(3643, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3673 => {
            const DEPTH: usize = collision_depth(3673);
            let (m1, m2) = find_collision_auto!(3673, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3673, &m1);
            let h2 = LTZ_hash_auto!(3673, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3677 => {
            const DEPTH: usize = collision_depth(3677);
            let (m1, m2) = find_collision_auto!(3677, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3677, &m1);
            let h2 = LTZ_hash_auto!(3677, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3697 => {
            const DEPTH: usize = collision_depth(3697);
            let (m1, m2) = find_collision_auto!(3697, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3697, &m1);
            let h2 = LTZ_hash_auto!(3697, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3727 => {
            const DEPTH: usize = collision_depth(3727);
            let (m1, m2) = find_collision_auto!(3727, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3727, &m1);
            let h2 = LTZ_hash_auto!(3727, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3733 => {
            const DEPTH: usize = collision_depth(3733);
            let (m1, m2) = find_collision_auto!(3733, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3733, &m1);
            let h2 = LTZ_hash_auto!(3733, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3767 => {
            const DEPTH: usize = collision_depth(3767);
            let (m1, m2) = find_collision_auto!(3767, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3767, &m1);
            let h2 = LTZ_hash_auto!(3767, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3793 => {
            const DEPTH: usize = collision_depth(3793);
            let (m1, m2) = find_collision_auto!(3793, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3793, &m1);
            let h2 = LTZ_hash_auto!(3793, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3797 => {
            const DEPTH: usize = collision_depth(3797);
            let (m1, m2) = find_collision_auto!(3797, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3797, &m1);
            let h2 = LTZ_hash_auto!(3797, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3803 => {
            const DEPTH: usize = collision_depth(3803);
            let (m1, m2) = find_collision_auto!(3803, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3803, &m1);
            let h2 = LTZ_hash_auto!(3803, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3823 => {
            const DEPTH: usize = collision_depth(3823);
            let (m1, m2) = find_collision_auto!(3823, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3823, &m1);
            let h2 = LTZ_hash_auto!(3823, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3833 => {
            const DEPTH: usize = collision_depth(3833);
            let (m1, m2) = find_collision_auto!(3833, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3833, &m1);
            let h2 = LTZ_hash_auto!(3833, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3847 => {
            const DEPTH: usize = collision_depth(3847);
            let (m1, m2) = find_collision_auto!(3847, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3847, &m1);
            let h2 = LTZ_hash_auto!(3847, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3853 => {
            const DEPTH: usize = collision_depth(3853);
            let (m1, m2) = find_collision_auto!(3853, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3853, &m1);
            let h2 = LTZ_hash_auto!(3853, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3863 => {
            const DEPTH: usize = collision_depth(3863);
            let (m1, m2) = find_collision_auto!(3863, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3863, &m1);
            let h2 = LTZ_hash_auto!(3863, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3877 => {
            const DEPTH: usize = collision_depth(3877);
            let (m1, m2) = find_collision_auto!(3877, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3877, &m1);
            let h2 = LTZ_hash_auto!(3877, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3907 => {
            const DEPTH: usize = collision_depth(3907);
            let (m1, m2) = find_collision_auto!(3907, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3907, &m1);
            let h2 = LTZ_hash_auto!(3907, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3917 => {
            const DEPTH: usize = collision_depth(3917);
            let (m1, m2) = find_collision_auto!(3917, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3917, &m1);
            let h2 = LTZ_hash_auto!(3917, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3923 => {
            const DEPTH: usize = collision_depth(3923);
            let (m1, m2) = find_collision_auto!(3923, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3923, &m1);
            let h2 = LTZ_hash_auto!(3923, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3943 => {
            const DEPTH: usize = collision_depth(3943);
            let (m1, m2) = find_collision_auto!(3943, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3943, &m1);
            let h2 = LTZ_hash_auto!(3943, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3947 => {
            const DEPTH: usize = collision_depth(3947);
            let (m1, m2) = find_collision_auto!(3947, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3947, &m1);
            let h2 = LTZ_hash_auto!(3947, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        3967 => {
            const DEPTH: usize = collision_depth(3967);
            let (m1, m2) = find_collision_auto!(3967, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(3967, &m1);
            let h2 = LTZ_hash_auto!(3967, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4003 => {
            const DEPTH: usize = collision_depth(4003);
            let (m1, m2) = find_collision_auto!(4003, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4003, &m1);
            let h2 = LTZ_hash_auto!(4003, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4007 => {
            const DEPTH: usize = collision_depth(4007);
            let (m1, m2) = find_collision_auto!(4007, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4007, &m1);
            let h2 = LTZ_hash_auto!(4007, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4013 => {
            const DEPTH: usize = collision_depth(4013);
            let (m1, m2) = find_collision_auto!(4013, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4013, &m1);
            let h2 = LTZ_hash_auto!(4013, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4027 => {
            const DEPTH: usize = collision_depth(4027);
            let (m1, m2) = find_collision_auto!(4027, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4027, &m1);
            let h2 = LTZ_hash_auto!(4027, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4057 => {
            const DEPTH: usize = collision_depth(4057);
            let (m1, m2) = find_collision_auto!(4057, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4057, &m1);
            let h2 = LTZ_hash_auto!(4057, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4073 => {
            const DEPTH: usize = collision_depth(4073);
            let (m1, m2) = find_collision_auto!(4073, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4073, &m1);
            let h2 = LTZ_hash_auto!(4073, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4093 => {
            const DEPTH: usize = collision_depth(4093);
            let (m1, m2) = find_collision_auto!(4093, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4093, &m1);
            let h2 = LTZ_hash_auto!(4093, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4127 => {
            const DEPTH: usize = collision_depth(4127);
            let (m1, m2) = find_collision_auto!(4127, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4127, &m1);
            let h2 = LTZ_hash_auto!(4127, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4133 => {
            const DEPTH: usize = collision_depth(4133);
            let (m1, m2) = find_collision_auto!(4133, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4133, &m1);
            let h2 = LTZ_hash_auto!(4133, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4153 => {
            const DEPTH: usize = collision_depth(4153);
            let (m1, m2) = find_collision_auto!(4153, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4153, &m1);
            let h2 = LTZ_hash_auto!(4153, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4157 => {
            const DEPTH: usize = collision_depth(4157);
            let (m1, m2) = find_collision_auto!(4157, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4157, &m1);
            let h2 = LTZ_hash_auto!(4157, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4177 => {
            const DEPTH: usize = collision_depth(4177);
            let (m1, m2) = find_collision_auto!(4177, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4177, &m1);
            let h2 = LTZ_hash_auto!(4177, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4217 => {
            const DEPTH: usize = collision_depth(4217);
            let (m1, m2) = find_collision_auto!(4217, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4217, &m1);
            let h2 = LTZ_hash_auto!(4217, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4243 => {
            const DEPTH: usize = collision_depth(4243);
            let (m1, m2) = find_collision_auto!(4243, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4243, &m1);
            let h2 = LTZ_hash_auto!(4243, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4253 => {
            const DEPTH: usize = collision_depth(4253);
            let (m1, m2) = find_collision_auto!(4253, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4253, &m1);
            let h2 = LTZ_hash_auto!(4253, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4273 => {
            const DEPTH: usize = collision_depth(4273);
            let (m1, m2) = find_collision_auto!(4273, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4273, &m1);
            let h2 = LTZ_hash_auto!(4273, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4283 => {
            const DEPTH: usize = collision_depth(4283);
            let (m1, m2) = find_collision_auto!(4283, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4283, &m1);
            let h2 = LTZ_hash_auto!(4283, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4297 => {
            const DEPTH: usize = collision_depth(4297);
            let (m1, m2) = find_collision_auto!(4297, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4297, &m1);
            let h2 = LTZ_hash_auto!(4297, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4327 => {
            const DEPTH: usize = collision_depth(4327);
            let (m1, m2) = find_collision_auto!(4327, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4327, &m1);
            let h2 = LTZ_hash_auto!(4327, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4337 => {
            const DEPTH: usize = collision_depth(4337);
            let (m1, m2) = find_collision_auto!(4337, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4337, &m1);
            let h2 = LTZ_hash_auto!(4337, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4357 => {
            const DEPTH: usize = collision_depth(4357);
            let (m1, m2) = find_collision_auto!(4357, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4357, &m1);
            let h2 = LTZ_hash_auto!(4357, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4363 => {
            const DEPTH: usize = collision_depth(4363);
            let (m1, m2) = find_collision_auto!(4363, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4363, &m1);
            let h2 = LTZ_hash_auto!(4363, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4373 => {
            const DEPTH: usize = collision_depth(4373);
            let (m1, m2) = find_collision_auto!(4373, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4373, &m1);
            let h2 = LTZ_hash_auto!(4373, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4397 => {
            const DEPTH: usize = collision_depth(4397);
            let (m1, m2) = find_collision_auto!(4397, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4397, &m1);
            let h2 = LTZ_hash_auto!(4397, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4423 => {
            const DEPTH: usize = collision_depth(4423);
            let (m1, m2) = find_collision_auto!(4423, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4423, &m1);
            let h2 = LTZ_hash_auto!(4423, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4447 => {
            const DEPTH: usize = collision_depth(4447);
            let (m1, m2) = find_collision_auto!(4447, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4447, &m1);
            let h2 = LTZ_hash_auto!(4447, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4457 => {
            const DEPTH: usize = collision_depth(4457);
            let (m1, m2) = find_collision_auto!(4457, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4457, &m1);
            let h2 = LTZ_hash_auto!(4457, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4463 => {
            const DEPTH: usize = collision_depth(4463);
            let (m1, m2) = find_collision_auto!(4463, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4463, &m1);
            let h2 = LTZ_hash_auto!(4463, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4483 => {
            const DEPTH: usize = collision_depth(4483);
            let (m1, m2) = find_collision_auto!(4483, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4483, &m1);
            let h2 = LTZ_hash_auto!(4483, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4493 => {
            const DEPTH: usize = collision_depth(4493);
            let (m1, m2) = find_collision_auto!(4493, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4493, &m1);
            let h2 = LTZ_hash_auto!(4493, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4507 => {
            const DEPTH: usize = collision_depth(4507);
            let (m1, m2) = find_collision_auto!(4507, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4507, &m1);
            let h2 = LTZ_hash_auto!(4507, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4513 => {
            const DEPTH: usize = collision_depth(4513);
            let (m1, m2) = find_collision_auto!(4513, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4513, &m1);
            let h2 = LTZ_hash_auto!(4513, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4517 => {
            const DEPTH: usize = collision_depth(4517);
            let (m1, m2) = find_collision_auto!(4517, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4517, &m1);
            let h2 = LTZ_hash_auto!(4517, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4523 => {
            const DEPTH: usize = collision_depth(4523);
            let (m1, m2) = find_collision_auto!(4523, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4523, &m1);
            let h2 = LTZ_hash_auto!(4523, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4547 => {
            const DEPTH: usize = collision_depth(4547);
            let (m1, m2) = find_collision_auto!(4547, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4547, &m1);
            let h2 = LTZ_hash_auto!(4547, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4567 => {
            const DEPTH: usize = collision_depth(4567);
            let (m1, m2) = find_collision_auto!(4567, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4567, &m1);
            let h2 = LTZ_hash_auto!(4567, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4583 => {
            const DEPTH: usize = collision_depth(4583);
            let (m1, m2) = find_collision_auto!(4583, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4583, &m1);
            let h2 = LTZ_hash_auto!(4583, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4597 => {
            const DEPTH: usize = collision_depth(4597);
            let (m1, m2) = find_collision_auto!(4597, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4597, &m1);
            let h2 = LTZ_hash_auto!(4597, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4603 => {
            const DEPTH: usize = collision_depth(4603);
            let (m1, m2) = find_collision_auto!(4603, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4603, &m1);
            let h2 = LTZ_hash_auto!(4603, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4637 => {
            const DEPTH: usize = collision_depth(4637);
            let (m1, m2) = find_collision_auto!(4637, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4637, &m1);
            let h2 = LTZ_hash_auto!(4637, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4643 => {
            const DEPTH: usize = collision_depth(4643);
            let (m1, m2) = find_collision_auto!(4643, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4643, &m1);
            let h2 = LTZ_hash_auto!(4643, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4657 => {
            const DEPTH: usize = collision_depth(4657);
            let (m1, m2) = find_collision_auto!(4657, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4657, &m1);
            let h2 = LTZ_hash_auto!(4657, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4663 => {
            const DEPTH: usize = collision_depth(4663);
            let (m1, m2) = find_collision_auto!(4663, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4663, &m1);
            let h2 = LTZ_hash_auto!(4663, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4673 => {
            const DEPTH: usize = collision_depth(4673);
            let (m1, m2) = find_collision_auto!(4673, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4673, &m1);
            let h2 = LTZ_hash_auto!(4673, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4703 => {
            const DEPTH: usize = collision_depth(4703);
            let (m1, m2) = find_collision_auto!(4703, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4703, &m1);
            let h2 = LTZ_hash_auto!(4703, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4723 => {
            const DEPTH: usize = collision_depth(4723);
            let (m1, m2) = find_collision_auto!(4723, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4723, &m1);
            let h2 = LTZ_hash_auto!(4723, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4733 => {
            const DEPTH: usize = collision_depth(4733);
            let (m1, m2) = find_collision_auto!(4733, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4733, &m1);
            let h2 = LTZ_hash_auto!(4733, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4783 => {
            const DEPTH: usize = collision_depth(4783);
            let (m1, m2) = find_collision_auto!(4783, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4783, &m1);
            let h2 = LTZ_hash_auto!(4783, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4787 => {
            const DEPTH: usize = collision_depth(4787);
            let (m1, m2) = find_collision_auto!(4787, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4787, &m1);
            let h2 = LTZ_hash_auto!(4787, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4793 => {
            const DEPTH: usize = collision_depth(4793);
            let (m1, m2) = find_collision_auto!(4793, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4793, &m1);
            let h2 = LTZ_hash_auto!(4793, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4813 => {
            const DEPTH: usize = collision_depth(4813);
            let (m1, m2) = find_collision_auto!(4813, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4813, &m1);
            let h2 = LTZ_hash_auto!(4813, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4817 => {
            const DEPTH: usize = collision_depth(4817);
            let (m1, m2) = find_collision_auto!(4817, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4817, &m1);
            let h2 = LTZ_hash_auto!(4817, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4877 => {
            const DEPTH: usize = collision_depth(4877);
            let (m1, m2) = find_collision_auto!(4877, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4877, &m1);
            let h2 = LTZ_hash_auto!(4877, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4903 => {
            const DEPTH: usize = collision_depth(4903);
            let (m1, m2) = find_collision_auto!(4903, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4903, &m1);
            let h2 = LTZ_hash_auto!(4903, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4933 => {
            const DEPTH: usize = collision_depth(4933);
            let (m1, m2) = find_collision_auto!(4933, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4933, &m1);
            let h2 = LTZ_hash_auto!(4933, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4937 => {
            const DEPTH: usize = collision_depth(4937);
            let (m1, m2) = find_collision_auto!(4937, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4937, &m1);
            let h2 = LTZ_hash_auto!(4937, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4943 => {
            const DEPTH: usize = collision_depth(4943);
            let (m1, m2) = find_collision_auto!(4943, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4943, &m1);
            let h2 = LTZ_hash_auto!(4943, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4957 => {
            const DEPTH: usize = collision_depth(4957);
            let (m1, m2) = find_collision_auto!(4957, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4957, &m1);
            let h2 = LTZ_hash_auto!(4957, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4967 => {
            const DEPTH: usize = collision_depth(4967);
            let (m1, m2) = find_collision_auto!(4967, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4967, &m1);
            let h2 = LTZ_hash_auto!(4967, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4973 => {
            const DEPTH: usize = collision_depth(4973);
            let (m1, m2) = find_collision_auto!(4973, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4973, &m1);
            let h2 = LTZ_hash_auto!(4973, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4987 => {
            const DEPTH: usize = collision_depth(4987);
            let (m1, m2) = find_collision_auto!(4987, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4987, &m1);
            let h2 = LTZ_hash_auto!(4987, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        4993 => {
            const DEPTH: usize = collision_depth(4993);
            let (m1, m2) = find_collision_auto!(4993, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(4993, &m1);
            let h2 = LTZ_hash_auto!(4993, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5003 => {
            const DEPTH: usize = collision_depth(5003);
            let (m1, m2) = find_collision_auto!(5003, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5003, &m1);
            let h2 = LTZ_hash_auto!(5003, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5023 => {
            const DEPTH: usize = collision_depth(5023);
            let (m1, m2) = find_collision_auto!(5023, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5023, &m1);
            let h2 = LTZ_hash_auto!(5023, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5077 => {
            const DEPTH: usize = collision_depth(5077);
            let (m1, m2) = find_collision_auto!(5077, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5077, &m1);
            let h2 = LTZ_hash_auto!(5077, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5087 => {
            const DEPTH: usize = collision_depth(5087);
            let (m1, m2) = find_collision_auto!(5087, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5087, &m1);
            let h2 = LTZ_hash_auto!(5087, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5107 => {
            const DEPTH: usize = collision_depth(5107);
            let (m1, m2) = find_collision_auto!(5107, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5107, &m1);
            let h2 = LTZ_hash_auto!(5107, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5113 => {
            const DEPTH: usize = collision_depth(5113);
            let (m1, m2) = find_collision_auto!(5113, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5113, &m1);
            let h2 = LTZ_hash_auto!(5113, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5147 => {
            const DEPTH: usize = collision_depth(5147);
            let (m1, m2) = find_collision_auto!(5147, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5147, &m1);
            let h2 = LTZ_hash_auto!(5147, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        5153 => {
            const DEPTH: usize = collision_depth(5153);
            let (m1, m2) = find_collision_auto!(5153, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(5153, &m1);
            let h2 = LTZ_hash_auto!(5153, &m2);
            if h1 == h2 {
                println!("collision found: 0o{} and 0o{}", m1, m2);
                println!("The hash values are [{}, {}, {}]", h1.0, h1.1, h1.2);
            } else {
                println!(
                    "hash values do not match: [{}, {}, {}] != [{}, {}, {}]",
                    h1.0, h1.1, h1.2, h2.0, h2.1, h2.2
                );
            }
        }
        _ => {
            eprintln!("unsupported prime {}", p);
            return;
        }
    }
    let dur = start.elapsed();
    println!("time = {:.2}s", dur.as_secs_f64());
}
