use crate::hash::LTZ_hash;
use crate::Fp4;
use sha2::{Digest, Sha256};
use std::time::Instant;

fn invariants_to_string<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    j1: Fp4<p, D1, D2C0, D2C1>,
    j2: Fp4<p, D1, D2C0, D2C1>,
    j3: Fp4<p, D1, D2C0, D2C1>,
) -> String {
    let combined = format!("{}{}{}", j1, j2, j3);

    let mut hash_bytes: [u8; 32] = Sha256::digest(combined.as_bytes()).into();

    let max_bits = hash_bytes.len() * 8;
    let mut bit_len = ((10.0 * (p as f64).log2()).round() as usize).max(1);
    bit_len = bit_len.min(max_bits);

    if bit_len < max_bits {
        let drop_bits = max_bits - bit_len;
        let bytes_to_clear = drop_bits / 8;
        let rem_bits = drop_bits % 8;

        for idx in 0..bytes_to_clear {
            hash_bytes[idx] = 0;
        }
        if rem_bits > 0 {
            let idx = bytes_to_clear;
            let mask = (1u8 << (8 - rem_bits)) - 1;
            hash_bytes[idx] &= mask;
        }
    }

    let mut digits_rev = Vec::with_capacity((bit_len + 2) / 3);
    let mut bits_remaining = bit_len;
    let mut buffer: u32 = 0;
    let mut bits_in_buffer: usize = 0;
    let mut byte_iter = hash_bytes.iter().rev();

    while bits_remaining > 0 {
        if bits_in_buffer < 3 && bits_remaining > bits_in_buffer {
            if let Some(&byte) = byte_iter.next() {
                buffer |= (byte as u32) << bits_in_buffer;
                bits_in_buffer += 8;
                continue;
            } else if bits_in_buffer == 0 {
                bits_in_buffer = bits_remaining;
                buffer = 0;
            }
        }

        let bits_to_take = bits_remaining.min(3);
        let mask = if bits_to_take == 3 {
            0x7
        } else {
            (1u32 << bits_to_take) - 1
        };
        let digit_bits = buffer & mask;
        digits_rev.push(std::char::from_digit(digit_bits, 8).unwrap());
        buffer >>= bits_to_take;
        bits_in_buffer = bits_in_buffer.saturating_sub(bits_to_take);
        bits_remaining -= bits_to_take;
    }

    digits_rev.reverse();
    digits_rev.into_iter().collect()
}

fn modified_hash<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    i: Fp4<p, D1, D2C0, D2C1>,
    m: &str,
) -> String {
    let (j1, j2, j3) = LTZ_hash::<p, D1, D2C0, D2C1>(i, m);
    invariants_to_string::<p, D1, D2C0, D2C1>(j1, j2, j3)
}

pub fn generic_attack<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
) -> (String, String) {
    let i = (-Fp4::<p, D1, D2C0, D2C1>::one())
        .sqrt()
        .expect("sqrt of -1 should exist in Fp4");

    let m0: String = "The initial message".to_string();
    let mut h_slow: String = String::new();
    let mut h_slow_prev: String = String::new();
    let mut h_fast: String = String::new();
    let mut h_fast_prev: String = String::new();
    let mut k: usize = 1;

    h_slow_prev = m0.clone();
    h_slow = modified_hash::<p, D1, D2C0, D2C1>(i, &m0);
    h_fast_prev = h_slow.clone();
    h_fast = modified_hash::<p, D1, D2C0, D2C1>(i, &h_slow);

    while h_slow != h_fast {
        h_slow_prev = h_slow.clone();
        h_slow = modified_hash::<p, D1, D2C0, D2C1>(i, &h_slow);
        h_fast_prev = modified_hash::<p, D1, D2C0, D2C1>(i, &h_fast);
        h_fast = modified_hash::<p, D1, D2C0, D2C1>(i, &h_fast_prev);
        k += 1;
    }
    if h_slow_prev == h_fast_prev {
        h_fast = h_slow.clone(); // H^k(m0)
        h_slow = m0.clone();
        while h_slow != h_fast {
            h_slow_prev = h_slow.clone();
            h_slow = modified_hash::<p, D1, D2C0, D2C1>(i, &h_slow_prev);
            h_fast_prev = h_fast.clone();
            h_fast = modified_hash::<p, D1, D2C0, D2C1>(i, &h_fast_prev);
        }
    }
    assert!(
        LTZ_hash::<p, D1, D2C0, D2C1>(i, &h_slow_prev)
            == LTZ_hash::<p, D1, D2C0, D2C1>(i, &h_fast_prev)
    );
    assert!(h_slow_prev != h_fast_prev);
    (h_slow_prev, h_fast_prev)
}

#[macro_export]
macro_rules! generic_attack_auto {
    ($p:expr) => {{
        const D1: u64 = $crate::find_non_residue($p);
        const D2: $crate::Cf2 = $crate::find_non_residue_fp2($p, D1);
        generic_attack::<$p, D1, { D2.c0 }, { D2.c1 }>()
    }};
}

pub fn handle_generic_attack(p: u64) {
    println!("Starting generic attack for p = {}", p);
    let start = Instant::now();
    match p {
        7 => {
            let (m1, m2) = generic_attack_auto!(7);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        13 => {
            let (m1, m2) = generic_attack_auto!(13);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        17 => {
            let (m1, m2) = generic_attack_auto!(17);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        23 => {
            let (m1, m2) = generic_attack_auto!(23);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        37 => {
            let (m1, m2) = generic_attack_auto!(37);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        43 => {
            let (m1, m2) = generic_attack_auto!(43);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        47 => {
            let (m1, m2) = generic_attack_auto!(47);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        53 => {
            let (m1, m2) = generic_attack_auto!(53);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        67 => {
            let (m1, m2) = generic_attack_auto!(67);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        73 => {
            let (m1, m2) = generic_attack_auto!(73);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        83 => {
            let (m1, m2) = generic_attack_auto!(83);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        97 => {
            let (m1, m2) = generic_attack_auto!(97);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        103 => {
            let (m1, m2) = generic_attack_auto!(103);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        107 => {
            let (m1, m2) = generic_attack_auto!(107);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        113 => {
            let (m1, m2) = generic_attack_auto!(113);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        127 => {
            let (m1, m2) = generic_attack_auto!(127);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        137 => {
            let (m1, m2) = generic_attack_auto!(137);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        157 => {
            let (m1, m2) = generic_attack_auto!(157);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        163 => {
            let (m1, m2) = generic_attack_auto!(163);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        167 => {
            let (m1, m2) = generic_attack_auto!(167);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        173 => {
            let (m1, m2) = generic_attack_auto!(173);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        193 => {
            let (m1, m2) = generic_attack_auto!(193);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        197 => {
            let (m1, m2) = generic_attack_auto!(197);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        223 => {
            let (m1, m2) = generic_attack_auto!(223);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        227 => {
            let (m1, m2) = generic_attack_auto!(227);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        233 => {
            let (m1, m2) = generic_attack_auto!(233);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        257 => {
            let (m1, m2) = generic_attack_auto!(257);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        263 => {
            let (m1, m2) = generic_attack_auto!(263);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        277 => {
            let (m1, m2) = generic_attack_auto!(277);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        283 => {
            let (m1, m2) = generic_attack_auto!(283);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        293 => {
            let (m1, m2) = generic_attack_auto!(293);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        307 => {
            let (m1, m2) = generic_attack_auto!(307);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        313 => {
            let (m1, m2) = generic_attack_auto!(313);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        317 => {
            let (m1, m2) = generic_attack_auto!(317);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        337 => {
            let (m1, m2) = generic_attack_auto!(337);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        347 => {
            let (m1, m2) = generic_attack_auto!(347);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        353 => {
            let (m1, m2) = generic_attack_auto!(353);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        367 => {
            let (m1, m2) = generic_attack_auto!(367);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        373 => {
            let (m1, m2) = generic_attack_auto!(373);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        383 => {
            let (m1, m2) = generic_attack_auto!(383);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        397 => {
            let (m1, m2) = generic_attack_auto!(397);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        433 => {
            let (m1, m2) = generic_attack_auto!(433);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        443 => {
            let (m1, m2) = generic_attack_auto!(443);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        457 => {
            let (m1, m2) = generic_attack_auto!(457);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        463 => {
            let (m1, m2) = generic_attack_auto!(463);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        467 => {
            let (m1, m2) = generic_attack_auto!(467);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        487 => {
            let (m1, m2) = generic_attack_auto!(487);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        503 => {
            let (m1, m2) = generic_attack_auto!(503);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        523 => {
            let (m1, m2) = generic_attack_auto!(523);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        547 => {
            let (m1, m2) = generic_attack_auto!(547);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        557 => {
            let (m1, m2) = generic_attack_auto!(557);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        563 => {
            let (m1, m2) = generic_attack_auto!(563);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        577 => {
            let (m1, m2) = generic_attack_auto!(577);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        587 => {
            let (m1, m2) = generic_attack_auto!(587);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        593 => {
            let (m1, m2) = generic_attack_auto!(593);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        607 => {
            let (m1, m2) = generic_attack_auto!(607);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        613 => {
            let (m1, m2) = generic_attack_auto!(613);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        617 => {
            let (m1, m2) = generic_attack_auto!(617);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        643 => {
            let (m1, m2) = generic_attack_auto!(643);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        647 => {
            let (m1, m2) = generic_attack_auto!(647);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        653 => {
            let (m1, m2) = generic_attack_auto!(653);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        673 => {
            let (m1, m2) = generic_attack_auto!(673);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        677 => {
            let (m1, m2) = generic_attack_auto!(677);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        683 => {
            let (m1, m2) = generic_attack_auto!(683);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        727 => {
            let (m1, m2) = generic_attack_auto!(727);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        733 => {
            let (m1, m2) = generic_attack_auto!(733);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        743 => {
            let (m1, m2) = generic_attack_auto!(743);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        757 => {
            let (m1, m2) = generic_attack_auto!(757);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        773 => {
            let (m1, m2) = generic_attack_auto!(773);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        787 => {
            let (m1, m2) = generic_attack_auto!(787);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        797 => {
            let (m1, m2) = generic_attack_auto!(797);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        823 => {
            let (m1, m2) = generic_attack_auto!(823);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        827 => {
            let (m1, m2) = generic_attack_auto!(827);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        853 => {
            let (m1, m2) = generic_attack_auto!(853);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        857 => {
            let (m1, m2) = generic_attack_auto!(857);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        863 => {
            let (m1, m2) = generic_attack_auto!(863);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        877 => {
            let (m1, m2) = generic_attack_auto!(877);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        883 => {
            let (m1, m2) = generic_attack_auto!(883);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        887 => {
            let (m1, m2) = generic_attack_auto!(887);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        907 => {
            let (m1, m2) = generic_attack_auto!(907);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        937 => {
            let (m1, m2) = generic_attack_auto!(937);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        947 => {
            let (m1, m2) = generic_attack_auto!(947);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        953 => {
            let (m1, m2) = generic_attack_auto!(953);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        967 => {
            let (m1, m2) = generic_attack_auto!(967);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        977 => {
            let (m1, m2) = generic_attack_auto!(977);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        983 => {
            let (m1, m2) = generic_attack_auto!(983);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        997 => {
            let (m1, m2) = generic_attack_auto!(997);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        1013 => {
            let (m1, m2) = generic_attack_auto!(1013);
            println!("Collision found: 0o{} and 0o{}", m1, m2);
        }
        _ => {
            eprintln!("generic attack is only implemented for selected primes");
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in generic attack is: {:?}", duration);
}
