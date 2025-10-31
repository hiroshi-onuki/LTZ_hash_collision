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
        7 => {
            let res = LTZ_hash_auto!(7, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        13 => {
            let res = LTZ_hash_auto!(13, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        17 => {
            let res = LTZ_hash_auto!(17, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        23 => {
            let res = LTZ_hash_auto!(23, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        37 => {
            let res = LTZ_hash_auto!(37, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        43 => {
            let res = LTZ_hash_auto!(43, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        47 => {
            let res = LTZ_hash_auto!(47, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        53 => {
            let res = LTZ_hash_auto!(53, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        67 => {
            let res = LTZ_hash_auto!(67, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        73 => {
            let res = LTZ_hash_auto!(73, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        83 => {
            let res = LTZ_hash_auto!(83, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        97 => {
            let res = LTZ_hash_auto!(97, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
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
        107 => {
            let res = LTZ_hash_auto!(107, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        113 => {
            let res = LTZ_hash_auto!(113, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        127 => {
            let res = LTZ_hash_auto!(127, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        137 => {
            let res = LTZ_hash_auto!(137, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        157 => {
            let res = LTZ_hash_auto!(157, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        163 => {
            let res = LTZ_hash_auto!(163, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        167 => {
            let res = LTZ_hash_auto!(167, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        173 => {
            let res = LTZ_hash_auto!(173, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        193 => {
            let res = LTZ_hash_auto!(193, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        197 => {
            let res = LTZ_hash_auto!(197, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        223 => {
            let res = LTZ_hash_auto!(223, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        227 => {
            let res = LTZ_hash_auto!(227, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        233 => {
            let res = LTZ_hash_auto!(233, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        257 => {
            let res = LTZ_hash_auto!(257, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        263 => {
            let res = LTZ_hash_auto!(263, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        277 => {
            let res = LTZ_hash_auto!(277, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        283 => {
            let res = LTZ_hash_auto!(283, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        293 => {
            let res = LTZ_hash_auto!(293, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        307 => {
            let res = LTZ_hash_auto!(307, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        313 => {
            let res = LTZ_hash_auto!(313, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        317 => {
            let res = LTZ_hash_auto!(317, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        337 => {
            let res = LTZ_hash_auto!(337, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        347 => {
            let res = LTZ_hash_auto!(347, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        353 => {
            let res = LTZ_hash_auto!(353, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        367 => {
            let res = LTZ_hash_auto!(367, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        373 => {
            let res = LTZ_hash_auto!(373, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        383 => {
            let res = LTZ_hash_auto!(383, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        397 => {
            let res = LTZ_hash_auto!(397, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        433 => {
            let res = LTZ_hash_auto!(433, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        443 => {
            let res = LTZ_hash_auto!(443, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        457 => {
            let res = LTZ_hash_auto!(457, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        463 => {
            let res = LTZ_hash_auto!(463, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        467 => {
            let res = LTZ_hash_auto!(467, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        487 => {
            let res = LTZ_hash_auto!(487, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        503 => {
            let res = LTZ_hash_auto!(503, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        523 => {
            let res = LTZ_hash_auto!(523, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        547 => {
            let res = LTZ_hash_auto!(547, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        557 => {
            let res = LTZ_hash_auto!(557, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        563 => {
            let res = LTZ_hash_auto!(563, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        577 => {
            let res = LTZ_hash_auto!(577, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        587 => {
            let res = LTZ_hash_auto!(587, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        593 => {
            let res = LTZ_hash_auto!(593, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        607 => {
            let res = LTZ_hash_auto!(607, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        613 => {
            let res = LTZ_hash_auto!(613, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        617 => {
            let res = LTZ_hash_auto!(617, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        643 => {
            let res = LTZ_hash_auto!(643, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        647 => {
            let res = LTZ_hash_auto!(647, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        653 => {
            let res = LTZ_hash_auto!(653, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        673 => {
            let res = LTZ_hash_auto!(673, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        677 => {
            let res = LTZ_hash_auto!(677, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        683 => {
            let res = LTZ_hash_auto!(683, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        727 => {
            let res = LTZ_hash_auto!(727, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        733 => {
            let res = LTZ_hash_auto!(733, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        743 => {
            let res = LTZ_hash_auto!(743, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        757 => {
            let res = LTZ_hash_auto!(757, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        773 => {
            let res = LTZ_hash_auto!(773, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        787 => {
            let res = LTZ_hash_auto!(787, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        797 => {
            let res = LTZ_hash_auto!(797, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        823 => {
            let res = LTZ_hash_auto!(823, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        827 => {
            let res = LTZ_hash_auto!(827, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        853 => {
            let res = LTZ_hash_auto!(853, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        857 => {
            let res = LTZ_hash_auto!(857, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        863 => {
            let res = LTZ_hash_auto!(863, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        877 => {
            let res = LTZ_hash_auto!(877, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        883 => {
            let res = LTZ_hash_auto!(883, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        887 => {
            let res = LTZ_hash_auto!(887, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        907 => {
            let res = LTZ_hash_auto!(907, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        937 => {
            let res = LTZ_hash_auto!(937, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        947 => {
            let res = LTZ_hash_auto!(947, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        953 => {
            let res = LTZ_hash_auto!(953, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        967 => {
            let res = LTZ_hash_auto!(967, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        977 => {
            let res = LTZ_hash_auto!(977, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        983 => {
            let res = LTZ_hash_auto!(983, input);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            println!("hash value is [{}, {}, {}]", res.0, res.1, res.2);
        }
        997 => {
            let res = LTZ_hash_auto!(997, input);
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
        7 => {
            const DEPTH: usize = collision_depth(7);
            let (m1, m2) = find_collision_auto!(7, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(7, &m1);
            let h2 = LTZ_hash_auto!(7, &m2);
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
        13 => {
            const DEPTH: usize = collision_depth(13);
            let (m1, m2) = find_collision_auto!(13, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(13, &m1);
            let h2 = LTZ_hash_auto!(13, &m2);
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
        17 => {
            const DEPTH: usize = collision_depth(17);
            let (m1, m2) = find_collision_auto!(17, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(17, &m1);
            let h2 = LTZ_hash_auto!(17, &m2);
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
        23 => {
            const DEPTH: usize = collision_depth(23);
            let (m1, m2) = find_collision_auto!(23, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(23, &m1);
            let h2 = LTZ_hash_auto!(23, &m2);
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
        37 => {
            const DEPTH: usize = collision_depth(37);
            let (m1, m2) = find_collision_auto!(37, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(37, &m1);
            let h2 = LTZ_hash_auto!(37, &m2);
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
        43 => {
            const DEPTH: usize = collision_depth(43);
            let (m1, m2) = find_collision_auto!(43, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(43, &m1);
            let h2 = LTZ_hash_auto!(43, &m2);
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
        47 => {
            const DEPTH: usize = collision_depth(47);
            let (m1, m2) = find_collision_auto!(47, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(47, &m1);
            let h2 = LTZ_hash_auto!(47, &m2);
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
        53 => {
            const DEPTH: usize = collision_depth(53);
            let (m1, m2) = find_collision_auto!(53, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(53, &m1);
            let h2 = LTZ_hash_auto!(53, &m2);
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
        67 => {
            const DEPTH: usize = collision_depth(67);
            let (m1, m2) = find_collision_auto!(67, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(67, &m1);
            let h2 = LTZ_hash_auto!(67, &m2);
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
        73 => {
            const DEPTH: usize = collision_depth(73);
            let (m1, m2) = find_collision_auto!(73, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(73, &m1);
            let h2 = LTZ_hash_auto!(73, &m2);
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
        83 => {
            const DEPTH: usize = collision_depth(83);
            let (m1, m2) = find_collision_auto!(83, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(83, &m1);
            let h2 = LTZ_hash_auto!(83, &m2);
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
        97 => {
            const DEPTH: usize = collision_depth(97);
            let (m1, m2) = find_collision_auto!(97, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(97, &m1);
            let h2 = LTZ_hash_auto!(97, &m2);
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
        107 => {
            const DEPTH: usize = collision_depth(107);
            let (m1, m2) = find_collision_auto!(107, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(107, &m1);
            let h2 = LTZ_hash_auto!(107, &m2);
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
        113 => {
            const DEPTH: usize = collision_depth(113);
            let (m1, m2) = find_collision_auto!(113, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(113, &m1);
            let h2 = LTZ_hash_auto!(113, &m2);
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
        127 => {
            const DEPTH: usize = collision_depth(127);
            let (m1, m2) = find_collision_auto!(127, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(127, &m1);
            let h2 = LTZ_hash_auto!(127, &m2);
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
        137 => {
            const DEPTH: usize = collision_depth(137);
            let (m1, m2) = find_collision_auto!(137, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(137, &m1);
            let h2 = LTZ_hash_auto!(137, &m2);
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
        157 => {
            const DEPTH: usize = collision_depth(157);
            let (m1, m2) = find_collision_auto!(157, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(157, &m1);
            let h2 = LTZ_hash_auto!(157, &m2);
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
        163 => {
            const DEPTH: usize = collision_depth(163);
            let (m1, m2) = find_collision_auto!(163, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(163, &m1);
            let h2 = LTZ_hash_auto!(163, &m2);
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
        167 => {
            const DEPTH: usize = collision_depth(167);
            let (m1, m2) = find_collision_auto!(167, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(167, &m1);
            let h2 = LTZ_hash_auto!(167, &m2);
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
        173 => {
            const DEPTH: usize = collision_depth(173);
            let (m1, m2) = find_collision_auto!(173, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(173, &m1);
            let h2 = LTZ_hash_auto!(173, &m2);
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
        193 => {
            const DEPTH: usize = collision_depth(193);
            let (m1, m2) = find_collision_auto!(193, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(193, &m1);
            let h2 = LTZ_hash_auto!(193, &m2);
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
        197 => {
            const DEPTH: usize = collision_depth(197);
            let (m1, m2) = find_collision_auto!(197, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(197, &m1);
            let h2 = LTZ_hash_auto!(197, &m2);
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
        223 => {
            const DEPTH: usize = collision_depth(223);
            let (m1, m2) = find_collision_auto!(223, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(223, &m1);
            let h2 = LTZ_hash_auto!(223, &m2);
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
        227 => {
            const DEPTH: usize = collision_depth(227);
            let (m1, m2) = find_collision_auto!(227, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(227, &m1);
            let h2 = LTZ_hash_auto!(227, &m2);
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
        233 => {
            const DEPTH: usize = collision_depth(233);
            let (m1, m2) = find_collision_auto!(233, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(233, &m1);
            let h2 = LTZ_hash_auto!(233, &m2);
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
        257 => {
            const DEPTH: usize = collision_depth(257);
            let (m1, m2) = find_collision_auto!(257, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(257, &m1);
            let h2 = LTZ_hash_auto!(257, &m2);
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
        263 => {
            const DEPTH: usize = collision_depth(263);
            let (m1, m2) = find_collision_auto!(263, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(263, &m1);
            let h2 = LTZ_hash_auto!(263, &m2);
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
        277 => {
            const DEPTH: usize = collision_depth(277);
            let (m1, m2) = find_collision_auto!(277, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(277, &m1);
            let h2 = LTZ_hash_auto!(277, &m2);
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
        283 => {
            const DEPTH: usize = collision_depth(283);
            let (m1, m2) = find_collision_auto!(283, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(283, &m1);
            let h2 = LTZ_hash_auto!(283, &m2);
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
        293 => {
            const DEPTH: usize = collision_depth(293);
            let (m1, m2) = find_collision_auto!(293, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(293, &m1);
            let h2 = LTZ_hash_auto!(293, &m2);
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
        307 => {
            const DEPTH: usize = collision_depth(307);
            let (m1, m2) = find_collision_auto!(307, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(307, &m1);
            let h2 = LTZ_hash_auto!(307, &m2);
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
        313 => {
            const DEPTH: usize = collision_depth(313);
            let (m1, m2) = find_collision_auto!(313, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(313, &m1);
            let h2 = LTZ_hash_auto!(313, &m2);
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
        317 => {
            const DEPTH: usize = collision_depth(317);
            let (m1, m2) = find_collision_auto!(317, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(317, &m1);
            let h2 = LTZ_hash_auto!(317, &m2);
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
        337 => {
            const DEPTH: usize = collision_depth(337);
            let (m1, m2) = find_collision_auto!(337, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(337, &m1);
            let h2 = LTZ_hash_auto!(337, &m2);
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
        347 => {
            const DEPTH: usize = collision_depth(347);
            let (m1, m2) = find_collision_auto!(347, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(347, &m1);
            let h2 = LTZ_hash_auto!(347, &m2);
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
        353 => {
            const DEPTH: usize = collision_depth(353);
            let (m1, m2) = find_collision_auto!(353, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(353, &m1);
            let h2 = LTZ_hash_auto!(353, &m2);
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
        367 => {
            const DEPTH: usize = collision_depth(367);
            let (m1, m2) = find_collision_auto!(367, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(367, &m1);
            let h2 = LTZ_hash_auto!(367, &m2);
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
        373 => {
            const DEPTH: usize = collision_depth(373);
            let (m1, m2) = find_collision_auto!(373, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(373, &m1);
            let h2 = LTZ_hash_auto!(373, &m2);
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
        383 => {
            const DEPTH: usize = collision_depth(383);
            let (m1, m2) = find_collision_auto!(383, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(383, &m1);
            let h2 = LTZ_hash_auto!(383, &m2);
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
        397 => {
            const DEPTH: usize = collision_depth(397);
            let (m1, m2) = find_collision_auto!(397, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(397, &m1);
            let h2 = LTZ_hash_auto!(397, &m2);
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
        433 => {
            const DEPTH: usize = collision_depth(433);
            let (m1, m2) = find_collision_auto!(433, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(433, &m1);
            let h2 = LTZ_hash_auto!(433, &m2);
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
        443 => {
            const DEPTH: usize = collision_depth(443);
            let (m1, m2) = find_collision_auto!(443, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(443, &m1);
            let h2 = LTZ_hash_auto!(443, &m2);
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
        457 => {
            const DEPTH: usize = collision_depth(457);
            let (m1, m2) = find_collision_auto!(457, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(457, &m1);
            let h2 = LTZ_hash_auto!(457, &m2);
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
        463 => {
            const DEPTH: usize = collision_depth(463);
            let (m1, m2) = find_collision_auto!(463, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(463, &m1);
            let h2 = LTZ_hash_auto!(463, &m2);
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
        467 => {
            const DEPTH: usize = collision_depth(467);
            let (m1, m2) = find_collision_auto!(467, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(467, &m1);
            let h2 = LTZ_hash_auto!(467, &m2);
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
        487 => {
            const DEPTH: usize = collision_depth(487);
            let (m1, m2) = find_collision_auto!(487, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(487, &m1);
            let h2 = LTZ_hash_auto!(487, &m2);
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
        503 => {
            const DEPTH: usize = collision_depth(503);
            let (m1, m2) = find_collision_auto!(503, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(503, &m1);
            let h2 = LTZ_hash_auto!(503, &m2);
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
        523 => {
            const DEPTH: usize = collision_depth(523);
            let (m1, m2) = find_collision_auto!(523, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(523, &m1);
            let h2 = LTZ_hash_auto!(523, &m2);
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
        547 => {
            const DEPTH: usize = collision_depth(547);
            let (m1, m2) = find_collision_auto!(547, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(547, &m1);
            let h2 = LTZ_hash_auto!(547, &m2);
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
        557 => {
            const DEPTH: usize = collision_depth(557);
            let (m1, m2) = find_collision_auto!(557, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(557, &m1);
            let h2 = LTZ_hash_auto!(557, &m2);
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
        563 => {
            const DEPTH: usize = collision_depth(563);
            let (m1, m2) = find_collision_auto!(563, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(563, &m1);
            let h2 = LTZ_hash_auto!(563, &m2);
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
        577 => {
            const DEPTH: usize = collision_depth(577);
            let (m1, m2) = find_collision_auto!(577, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(577, &m1);
            let h2 = LTZ_hash_auto!(577, &m2);
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
        587 => {
            const DEPTH: usize = collision_depth(587);
            let (m1, m2) = find_collision_auto!(587, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(587, &m1);
            let h2 = LTZ_hash_auto!(587, &m2);
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
        593 => {
            const DEPTH: usize = collision_depth(593);
            let (m1, m2) = find_collision_auto!(593, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(593, &m1);
            let h2 = LTZ_hash_auto!(593, &m2);
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
        607 => {
            const DEPTH: usize = collision_depth(607);
            let (m1, m2) = find_collision_auto!(607, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(607, &m1);
            let h2 = LTZ_hash_auto!(607, &m2);
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
        613 => {
            const DEPTH: usize = collision_depth(613);
            let (m1, m2) = find_collision_auto!(613, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(613, &m1);
            let h2 = LTZ_hash_auto!(613, &m2);
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
        617 => {
            const DEPTH: usize = collision_depth(617);
            let (m1, m2) = find_collision_auto!(617, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(617, &m1);
            let h2 = LTZ_hash_auto!(617, &m2);
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
        643 => {
            const DEPTH: usize = collision_depth(643);
            let (m1, m2) = find_collision_auto!(643, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(643, &m1);
            let h2 = LTZ_hash_auto!(643, &m2);
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
        647 => {
            const DEPTH: usize = collision_depth(647);
            let (m1, m2) = find_collision_auto!(647, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(647, &m1);
            let h2 = LTZ_hash_auto!(647, &m2);
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
        653 => {
            const DEPTH: usize = collision_depth(653);
            let (m1, m2) = find_collision_auto!(653, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(653, &m1);
            let h2 = LTZ_hash_auto!(653, &m2);
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
        673 => {
            const DEPTH: usize = collision_depth(673);
            let (m1, m2) = find_collision_auto!(673, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(673, &m1);
            let h2 = LTZ_hash_auto!(673, &m2);
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
        677 => {
            const DEPTH: usize = collision_depth(677);
            let (m1, m2) = find_collision_auto!(677, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(677, &m1);
            let h2 = LTZ_hash_auto!(677, &m2);
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
        683 => {
            const DEPTH: usize = collision_depth(683);
            let (m1, m2) = find_collision_auto!(683, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(683, &m1);
            let h2 = LTZ_hash_auto!(683, &m2);
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
        727 => {
            const DEPTH: usize = collision_depth(727);
            let (m1, m2) = find_collision_auto!(727, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(727, &m1);
            let h2 = LTZ_hash_auto!(727, &m2);
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
        733 => {
            const DEPTH: usize = collision_depth(733);
            let (m1, m2) = find_collision_auto!(733, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(733, &m1);
            let h2 = LTZ_hash_auto!(733, &m2);
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
        743 => {
            const DEPTH: usize = collision_depth(743);
            let (m1, m2) = find_collision_auto!(743, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(743, &m1);
            let h2 = LTZ_hash_auto!(743, &m2);
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
        757 => {
            const DEPTH: usize = collision_depth(757);
            let (m1, m2) = find_collision_auto!(757, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(757, &m1);
            let h2 = LTZ_hash_auto!(757, &m2);
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
        773 => {
            const DEPTH: usize = collision_depth(773);
            let (m1, m2) = find_collision_auto!(773, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(773, &m1);
            let h2 = LTZ_hash_auto!(773, &m2);
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
        787 => {
            const DEPTH: usize = collision_depth(787);
            let (m1, m2) = find_collision_auto!(787, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(787, &m1);
            let h2 = LTZ_hash_auto!(787, &m2);
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
        797 => {
            const DEPTH: usize = collision_depth(797);
            let (m1, m2) = find_collision_auto!(797, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(797, &m1);
            let h2 = LTZ_hash_auto!(797, &m2);
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
        823 => {
            const DEPTH: usize = collision_depth(823);
            let (m1, m2) = find_collision_auto!(823, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(823, &m1);
            let h2 = LTZ_hash_auto!(823, &m2);
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
        827 => {
            const DEPTH: usize = collision_depth(827);
            let (m1, m2) = find_collision_auto!(827, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(827, &m1);
            let h2 = LTZ_hash_auto!(827, &m2);
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
        853 => {
            const DEPTH: usize = collision_depth(853);
            let (m1, m2) = find_collision_auto!(853, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(853, &m1);
            let h2 = LTZ_hash_auto!(853, &m2);
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
        857 => {
            const DEPTH: usize = collision_depth(857);
            let (m1, m2) = find_collision_auto!(857, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(857, &m1);
            let h2 = LTZ_hash_auto!(857, &m2);
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
        863 => {
            const DEPTH: usize = collision_depth(863);
            let (m1, m2) = find_collision_auto!(863, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(863, &m1);
            let h2 = LTZ_hash_auto!(863, &m2);
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
        877 => {
            const DEPTH: usize = collision_depth(877);
            let (m1, m2) = find_collision_auto!(877, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(877, &m1);
            let h2 = LTZ_hash_auto!(877, &m2);
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
        883 => {
            const DEPTH: usize = collision_depth(883);
            let (m1, m2) = find_collision_auto!(883, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(883, &m1);
            let h2 = LTZ_hash_auto!(883, &m2);
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
        887 => {
            const DEPTH: usize = collision_depth(887);
            let (m1, m2) = find_collision_auto!(887, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(887, &m1);
            let h2 = LTZ_hash_auto!(887, &m2);
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
        907 => {
            const DEPTH: usize = collision_depth(907);
            let (m1, m2) = find_collision_auto!(907, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(907, &m1);
            let h2 = LTZ_hash_auto!(907, &m2);
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
        937 => {
            const DEPTH: usize = collision_depth(937);
            let (m1, m2) = find_collision_auto!(937, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(937, &m1);
            let h2 = LTZ_hash_auto!(937, &m2);
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
        947 => {
            const DEPTH: usize = collision_depth(947);
            let (m1, m2) = find_collision_auto!(947, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(947, &m1);
            let h2 = LTZ_hash_auto!(947, &m2);
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
        953 => {
            const DEPTH: usize = collision_depth(953);
            let (m1, m2) = find_collision_auto!(953, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(953, &m1);
            let h2 = LTZ_hash_auto!(953, &m2);
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
        967 => {
            const DEPTH: usize = collision_depth(967);
            let (m1, m2) = find_collision_auto!(967, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(967, &m1);
            let h2 = LTZ_hash_auto!(967, &m2);
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
        977 => {
            const DEPTH: usize = collision_depth(977);
            let (m1, m2) = find_collision_auto!(977, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(977, &m1);
            let h2 = LTZ_hash_auto!(977, &m2);
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
        983 => {
            const DEPTH: usize = collision_depth(983);
            let (m1, m2) = find_collision_auto!(983, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(983, &m1);
            let h2 = LTZ_hash_auto!(983, &m2);
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
        997 => {
            const DEPTH: usize = collision_depth(997);
            let (m1, m2) = find_collision_auto!(997, DEPTH);
            let d1 = find_non_residue(p);
            let d2 = find_non_residue_fp2(p, d1);
            println!(
                "p = {}, Fp4 = Fp[x, y], where x^2 = {}, y^2 = {} + {}*x",
                p, d1, d2.c0, d2.c1
            );
            let h1 = LTZ_hash_auto!(997, &m1);
            let h2 = LTZ_hash_auto!(997, &m2);
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
