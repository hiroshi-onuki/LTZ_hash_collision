use crate::{Fp2, Fp4};
use crate::theta::{Theta, compute_twoisogeny, theta_to_rosenhain};
use crate::rosenhain::Rosenhain;
use crate::hash::{generate_hash_init_curve, invariants_to_message};
use std::io::{self, Write};

/// Compute `floor(log2(n))` for a 128-bit integer.
pub const fn floor_log2_u128(mut n: u128) -> usize {
    if n == 0 {
        return 0;
    }
    let mut log = 0usize;
    while n > 1 {
        n >>= 1;
        log += 1;
    }
    log
}

/// Determine a search depth for collision finding.
///
/// The depth is the largest integer less than or equal to
/// `log2(p)`.
pub const fn collision_depth(p: u64) -> usize {
    floor_log2_u128(p as u128)
}

pub fn path_to_Fp2<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    theta: Theta<p, D1, D2C0, D2C1>,
    depth: usize,
    i: Fp4<p, D1, D2C0, D2C1>,
    is_good: bool,
) -> ( Vec<(
        Fp4<p, D1, D2C0, D2C1>,
        Fp4<p, D1, D2C0, D2C1>,
        Fp4<p, D1, D2C0, D2C1>,
    )>, bool
) {
    let ram = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&theta.null);
    let mut path = Vec::new();
    path.push(ram.IgusaInvariants());

    fn dfs<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
        theta: Theta<p, D1, D2C0, D2C1>,
        is_root: bool,
        i: Fp4<p, D1, D2C0, D2C1>,
        path: &mut Vec<(
            Fp4<p, D1, D2C0, D2C1>,
            Fp4<p, D1, D2C0, D2C1>,
            Fp4<p, D1, D2C0, D2C1>,
        )>,
        depth: usize,
        is_good: bool,
        count: &mut usize,
    ) -> (bool, bool) {
        if depth == 0 {
            return (false, is_good);
        }
        let range = { 0usize..8usize };
        for m in range {
            *count += 1;
            if *count % 100000 == 0 {
                print!("\rsearched {} nodes", *count);
                io::stdout().flush().unwrap();
            }
            let new_null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&theta.null, m, i);
            let new_theta = Theta { null: new_null };
            let ram = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&new_theta.null);
            let inv = ram.IgusaInvariants();
            path.push(inv);
            if inv.0.c1 == Fp2::<p, D1>::zero()
                && inv.1.c1 == Fp2::<p, D1>::zero()
                && inv.2.c1 == Fp2::<p, D1>::zero()
            {
                if is_good {
                    if is_good_conjugate(&theta, &new_theta, i).is_some() {
                        return (true, true);
                    }
                } else {
                    return (true, is_good_conjugate(&theta, &new_theta, i).is_some());
                }
            }
            let (found, is_good_path) = dfs::<p, D1, D2C0, D2C1>(
                new_theta,
                false,
                i,
                path,
                depth - 1,
                is_good,
                count,
            );
            if found {
                return (true, is_good_path);
            }
            path.pop();
        }
        (false, false)
    }

    let mut count = 0usize;
    let (found, is_good_path) = dfs::<p, D1, D2C0, D2C1>(
        theta,
        true,
        i,
        &mut path,
        depth,
        is_good,
        &mut count,
    );
    println!("\rsearched {} nodes", count);
    (path, is_good_path)
}

/// Check whether `t1` and `t2` are related by complex conjugation.
fn is_good_conjugate<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    t1: &Theta<p, D1, D2C0, D2C1>,
    t2: &Theta<p, D1, D2C0, D2C1>,
    i : Fp4<p, D1, D2C0, D2C1>,
) -> Option<usize> {

    let ram1 = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&t1.null);
    let inv1 = ram1.IgusaInvariants();
    let target = (
        inv1.0.conjugate(),
        inv1.1.conjugate(),
        inv1.2.conjugate(),
    );

    for m in 0usize..8usize {
        let new_null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&t2.null, m, i);
        let ram = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&new_null);
        if ram.IgusaInvariants() == target {
            return Some(m);
        }
    }

    None
}

/// Return whether there exists a path of good extensions t1 to t2 to t3.
fn is_good_path<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    t1: &Theta<p, D1, D2C0, D2C1>,
    t2: &Theta<p, D1, D2C0, D2C1>,
    t3: &Theta<p, D1, D2C0, D2C1>,
    i : Fp4<p, D1, D2C0, D2C1>,
) -> bool {
    let ram2 = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&t2.null);
    let inv2 = ram2.IgusaInvariants();
    let ram3 = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&t3.null);
    let inv3 = ram3.IgusaInvariants();
    for m in 0usize..15usize {
        let new_null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&t1.null, m, i);
        let ram = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&new_null);
        if ram.IgusaInvariants() == inv2 {
            for n in 0usize..8usize {
                let new_null2 = compute_twoisogeny::<p, D1, D2C0, D2C1>(&new_null, n, i);
                let ram2 = theta_to_rosenhain::<p, D1, D2C0, D2C1>(&new_null2);
                if ram2.IgusaInvariants() == inv3 {
                    return true;
                }
            }
        }
    }
    false
}

fn conjugate_path<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    path: &[(Fp4<p, D1, D2C0, D2C1>, Fp4<p, D1, D2C0, D2C1>, Fp4<p, D1, D2C0, D2C1>)],
) -> Vec<(
    Fp4<p, D1, D2C0, D2C1>,
    Fp4<p, D1, D2C0, D2C1>,
    Fp4<p, D1, D2C0, D2C1>,
)> {
    path.iter().map(|(a, b, c)| (a.conjugate(), b.conjugate(), c.conjugate())).collect()
}

pub fn find_collision<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    depth: usize,
) -> (String, String)
    {
    let i = (-Fp4::<p, D1, D2C0, D2C1>::one())
        .sqrt()
        .expect("sqrt of -1 should exist in Fp4");

    let init = generate_hash_init_curve::<p, D1, D2C0, D2C1>();
    let init_inv = init.IgusaInvariants();
    let theta0 = Theta::<p, D1, D2C0, D2C1>::new(init);
    let null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&theta0.null, 0, i);
    let theta1 = Theta { null };
    let mut theta2 = Theta::<p, D1, D2C0, D2C1>{ null: [Fp4::<p, D1, D2C0, D2C1>::zero(); 16] };
    let mut found = false;
    for m in 0usize..8usize {
        let null = compute_twoisogeny::<p, D1, D2C0, D2C1>(&theta0.null, m, i);
        theta2 = Theta { null };
        if is_good_path::<p, D1, D2C0, D2C1>(&theta1, &theta0, &theta2, i) {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "failed to find a good path"
    );
    
    println!("starting to find the first Fp2 invariant");
    let (mut path1, is_good_path) = path_to_Fp2::<p, D1, D2C0, D2C1>(theta1, depth, i, false);
    println!("found the first Fp2 invariant which is good: {}", is_good_path);

    println!("starting to find the second Fp2 invariant");
    let (mut path2, _) = path_to_Fp2::<p, D1, D2C0, D2C1>(theta2, depth, i, !is_good_path);
    println!("found the second Fp2 invariant");

    if !is_good_path {
        // swap path1 and path2 if the first path is not good
        std::mem::swap(&mut path1, &mut path2);
    }
    path1.insert(0, init_inv);
    path2.insert(0, init_inv);
    let mut path1conj = conjugate_path::<p, D1, D2C0, D2C1>(&path1);
    path1conj.pop();
    path1conj.reverse();
    path1.extend(path1conj);
    let mut path2conj = conjugate_path::<p, D1, D2C0, D2C1>(&path2);
    path2conj.remove(0);
    path1.extend(path2conj);

    let m1 = invariants_to_message(&path1);
    let m2 = invariants_to_message(&path2);
    (m1, m2)
}

/// Convenience macro to run `find_collision` with the default hashing curve parameters.
#[macro_export]
macro_rules! find_collision_auto {
    ($p:expr, $depth:expr) => {{
        const D1: u64 = $crate::find_non_residue($p);
        const D2: $crate::Cf2 = $crate::find_non_residue_fp2($p, D1);
        $crate::collision::find_collision::<$p, D1, { D2.c0 }, { D2.c1 }>( $depth )
    }};
}