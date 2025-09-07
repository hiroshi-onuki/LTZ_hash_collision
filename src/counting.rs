pub fn number_of_vertices(p: u64) -> u128 {
    assert!(p > 5, "p must be greater than 5");
    if matches!(p % 5, 1 | 4) {
        let p2 = p as u128 * p as u128;
        let term = p2 - 1;
        p2 * term * term / 2880u128
    } else {
        let p2 = p as u128 * p as u128;
        1u128 + ((p2 - 9) * (p2 - 3 * (p as u128) + 8) * (p2 + 3 * (p as u128) + 8)) / 2880u128
    }
}

const LCG_MULTIPLIER: u64 = 6364136223846793005;
const LCG_INCREMENT: u64 = 1;
// Largest prime less than 2^64
const LCG_MODULUS: u64 = 18446744073709551557;

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed % LCG_MODULUS)
    }

    fn next(&mut self) -> u64 {
        let next = ((self.0 as u128 * LCG_MULTIPLIER as u128
            + LCG_INCREMENT as u128)
            % LCG_MODULUS as u128) as u64;
        self.0 = next;
        self.0
    }
}

use crate::theta::{compute_twoisogeny, theta_to_rosenhain, Theta};
use crate::rosenhain::Rosenhain;
use crate::{Fp2, Fp4};
use std::collections::HashSet;


/// Convenience macro to enumerate invariants with automatically chosen extension parameters.
#[macro_export]
macro_rules! enumerate_igusa_auto {
    ($p:expr) => {{
        const D1: u64 = $crate::find_non_residue($p);
        const D2: $crate::Cf2 = $crate::find_non_residue_fp2($p, D1);
        $crate::counting::enumerate_igusa_invariants::<$p, D1, { D2.c0 }, { D2.c1 }>()
    }};
}

pub fn enumerate_igusa_invariants<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>()
    -> (
        Vec<(
            Fp4<p, D1, D2C0, D2C1>,
            Fp4<p, D1, D2C0, D2C1>,
            Fp4<p, D1, D2C0, D2C1>,
        )>,
        usize,
    )
{
    assert!(p > 5, "p must be greater than 5");

    let i = (-Fp4::<p, D1, D2C0, D2C1>::one())
        .sqrt()
        .expect("sqrt of -1 should exist in Fp4");

    let target = number_of_vertices(p) as usize;

    let mut invariants = Vec::new();
    let mut rams = Vec::new();
    let mut seen: HashSet<(
        Fp4<p, D1, D2C0, D2C1>,
        Fp4<p, D1, D2C0, D2C1>,
        Fp4<p, D1, D2C0, D2C1>,
    )> = HashSet::new();
    let mut ram = Rosenhain::<p, D1, D2C0, D2C1>::generate_init_curve();
    let first_inv = ram.IgusaInvariants();
    if first_inv.0.c1 == Fp2::<p, D1>::zero()
        && first_inv.1.c1 == Fp2::<p, D1>::zero()
        && first_inv.2.c1 == Fp2::<p, D1>::zero()
    {
        invariants.push(first_inv);
    }
    seen.insert(first_inv);
    rams.push(ram);

    let mut theta = Theta::<p, D1, D2C0, D2C1>::new(ram);
    let mut rng = Lcg::new(1);

    let mut cnt = 0;
    while seen.len() < target {
        let m = (rng.next() % 8) as usize;
        let new_null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&theta.null, m, i);
        theta = Theta { null: new_null };
        ram = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&theta.null);
        let inv = ram.IgusaInvariants();
        if seen.insert(inv) {
            if inv.0.c1 == Fp2::<p, D1>::zero()
                && inv.1.c1 == Fp2::<p, D1>::zero()
                && inv.2.c1 == Fp2::<p, D1>::zero()
            {
                invariants.push(inv);
            }
            rams.push(ram);
        }
        cnt += 1;
        if cnt % 10000 == 0 {
            print!(
                "\rp = {}, found {}/{} invariants, {} Fp2 invariants",
                p,
                seen.len(),
                target,
                invariants.len()
            );
        }
    }
    println!();

    println!("Check the number of invariants is equal to the number of vertices:");
    for ram in rams {
        theta = Theta::new(ram);
        for m in 0..15 {
            let new_null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&theta.null, m, i);
            let ram_new = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&new_null);
            let inv = ram_new.IgusaInvariants();
            if seen.insert(inv) {
                println!("The number of invariants does not match the number of vertices!");
                return (Vec::new(), 0);
            }
        }
    }
    println!("Ok.");
    let n = invariants.len();
    (invariants, n)
}
