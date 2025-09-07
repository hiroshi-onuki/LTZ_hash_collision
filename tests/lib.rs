use LTZ_hash::{Fp, Fp2, Fp4};

type F7 = Fp<7>;
type F7Ext = Fp2<7, 6>; // x^2 + 1 over F7
type F5 = Fp<5>;
type F5Ext = Fp2<5, 2>; // x^2 - 2 over F5 (2 is a non-residue)

#[test]
fn addition() {
    let a = F7::new(3);
    let b = F7::new(6);
    assert_eq!((a + b).value(), 2); // 3 + 6 mod 7 = 2
}

#[test]
fn multiplication() {
    let a = F7::new(3);
    let b = F7::new(5);
    assert_eq!((a * b).value(), 1); // 15 mod 7 = 1
}

#[test]
fn inverse() {
    let a = F7::new(3);
    let inv_a = a.inverse();
    assert_eq!((a * inv_a).value(), 1);
}

#[test]
fn division() {
    let a = F7::new(3);
    let b = F7::new(5);
    assert_eq!(a / b, a * b.inverse());
}

#[test]
fn division_int() {
    let a = F7::new(3);
    assert_eq!(a / 2u64, a * F7::new(2).inverse());
}

#[test]
fn fp2_addition() {
    let a = F7Ext::new(F7::new(3), F7::new(1));
    let b = F7Ext::new(F7::new(2), F7::new(5));
    let c = a + b;
    assert_eq!(c.c0.value(), 5);
    assert_eq!(c.c1.value(), 6);
}

#[test]
fn fp2_multiplication() {
    let a = F7Ext::new(F7::new(3), F7::new(1));
    let b = F7Ext::new(F7::new(2), F7::new(5));
    let c = a * b;
    assert_eq!(c.c0.value(), 1);
    assert_eq!(c.c1.value(), 3);
}

#[test]
fn fp2_inverse() {
    let a = F7Ext::new(F7::new(3), F7::new(1));
    let inv_a = a.inverse();
    let prod = a * inv_a;
    assert_eq!(prod.c0.value(), 1);
    assert_eq!(prod.c1.value(), 0);
}

#[test]
fn fp2_division() {
    let a = F7Ext::new(F7::new(3), F7::new(1));
    let b = F7Ext::new(F7::new(2), F7::new(5));
    assert_eq!(a / b, a * b.inverse());
}

#[test]
fn fp2_division_int() {
    let a = F7Ext::new(F7::new(3), F7::new(1));
    assert_eq!(a / 2u64, a * F7Ext::new(F7::new(2), F7::new(0)).inverse());
}

#[test]
fn fp2_non_residue_field() {
    let a = F5Ext::new(F5::new(1), F5::new(1));
    let b = F5Ext::new(F5::new(2), F5::new(4));
    let c = a * b; // should be (0 + 1*x)
    assert_eq!(c.c0.value(), 0);
    assert_eq!(c.c1.value(), 1);
}

#[test]
fn fp2_frobenius() {
    let a = F7Ext::new(F7::new(2), F7::new(3));
    let ap = a.pow(7);
    assert_eq!(ap.c0.value(), a.c0.value());
    assert_eq!(ap.c1.value(), (-a.c1).value());
}

type F7Ext4 = Fp4<7, 6, 1, 2>; // y^2 - (1 + 2*x) over F7Ext
// Field with prime 1013 for LTZ_hash tests
type F1013 = Fp<1013>;
type F1013Ext = Fp2<1013, 2>; // x^2 - 2 over F1013
type F1013Ext4 = Fp4<1013, 2, 1, 2>; // y^2 - (1 + 2*x) over F1013Ext

#[test]
fn fp4_addition() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(3), F7::new(1)),
        F7Ext::new(F7::new(2), F7::new(4)),
    );
    let b = F7Ext4::new(
        F7Ext::new(F7::new(5), F7::new(6)),
        F7Ext::new(F7::new(1), F7::new(0)),
    );
    let c = a + b;
    assert_eq!(c.c0, F7Ext::new(F7::new(1), F7::new(0)));
    assert_eq!(c.c1, F7Ext::new(F7::new(3), F7::new(4)));
}

#[test]
fn fp4_multiplication() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(3), F7::new(1)),
        F7Ext::new(F7::new(2), F7::new(4)),
    );
    let b = F7Ext4::new(
        F7Ext::new(F7::new(5), F7::new(6)),
        F7Ext::new(F7::new(1), F7::new(0)),
    );
    let c = a * b;
    assert_eq!(c.c0, F7Ext::new(F7::new(3), F7::new(3)));
    assert_eq!(c.c1, F7Ext::new(F7::new(3), F7::new(5)));
}

#[test]
fn fp4_inverse() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(3), F7::new(1)),
        F7Ext::new(F7::new(2), F7::new(4)),
    );
    let inv_a = a.inverse();
    let prod = a * inv_a;
    assert_eq!(prod.c0, F7Ext::one());
    assert_eq!(prod.c1, F7Ext::zero());
}

#[test]
fn fp4_division() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(3), F7::new(1)),
        F7Ext::new(F7::new(2), F7::new(4)),
    );
    let b = F7Ext4::new(
        F7Ext::new(F7::new(5), F7::new(6)),
        F7Ext::new(F7::new(1), F7::new(0)),
    );
    assert_eq!(a / b, a * b.inverse());
}

#[test]
fn fp4_division_int() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(3), F7::new(1)),
        F7Ext::new(F7::new(2), F7::new(4)),
    );
    assert_eq!(
        a / 2u64,
        a * F7Ext4::new(
            F7Ext::new(F7::new(2), F7::new(0)),
            F7Ext::zero(),
        )
        .inverse()
    );
}

#[test]
fn fp4_frobenius() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(2), F7::new(3)),
        F7Ext::new(F7::new(4), F7::new(5)),
    );
    let ap = a.pow(49);
    assert_eq!(ap.c0, a.c0);
    assert_eq!(ap.c1, -a.c1);
}

#[test]
fn fp4_sqrt_square() {
    let a = F7Ext4::new(
        F7Ext::new(F7::new(3), F7::new(1)),
        F7Ext::new(F7::new(2), F7::new(4)),
    );
    let square = a * a;
    let root = square.sqrt().unwrap();
    assert!(root == a || root == -a);
}

#[test]
fn fp4_sqrt_non_square() {
    let ns = F7Ext4::new(
        F7Ext::new(F7::new(0), F7::new(1)),
        F7Ext::new(F7::new(1), F7::new(1)),
    );
    assert!(ns.sqrt().is_none());
}

#[test]
fn fp4_display() {
    let elem = F7Ext4::new(
        F7Ext::new(F7::new(1), F7::new(2)),
        F7Ext::new(F7::new(3), F7::new(4)),
    );
    assert_eq!(format!("{}", elem), "((1 + 2*x) + (3 + 4*x)*y)");
}


use LTZ_hash::counting::number_of_vertices;

#[test]
fn counting_small_primes() {
    assert_eq!(number_of_vertices(7), 40);
    assert_eq!(number_of_vertices(11), 605);
    assert_eq!(number_of_vertices(13), 1657);
}

use LTZ_hash::hash::{self, LTZ_hash, LTZ_hash_with_init};
use LTZ_hash::theta::{compute_twoisogeny, theta_to_rosenhain, Theta};
use LTZ_hash::rosenhain::Rosenhain;

#[test]
fn ltz_hash_empty() {
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );
    let result = LTZ_hash::<1013, 2, 1, 2>(i, "");
    let expected = Rosenhain::<1013, 2, 1, 2>::generate_init_curve().IgusaInvariants();
    assert_eq!(result, expected);
}

#[test]
fn ltz_hash_basic() {
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );
    // Applying two sequential isogenies with index 0 corresponds to the
    // octal message "00".
    let init = Rosenhain::<1013, 2, 1, 2>::generate_init_curve();
    let mut theta = Theta::<1013, 2, 1, 2>::new(init);
    let new_null = compute_twoisogeny::<1013, 2, 1, 2>(&theta.null, 0, i);
    theta = Theta { null: new_null };
    let new_null = compute_twoisogeny::<1013, 2, 1, 2>(&theta.null, 0, i);
    theta = Theta { null: new_null };
    let ram = theta_to_rosenhain::<1013, 2, 1, 2>(&theta.null);
    let expected = ram.IgusaInvariants();
    let result = LTZ_hash::<1013, 2, 1, 2>(i, "00");
    assert_eq!(result, expected);
}

#[test]
fn ltz_hash_sequence() {
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );
    // Sequentially apply isogenies 0, 1, 2, 3 on the initial curve
    let init = Rosenhain::<1013, 2, 1, 2>::generate_init_curve();
    let mut theta = Theta::<1013, 2, 1, 2>::new(init);
    for m in [0usize, 1, 2, 3].iter() {
        let new_null = compute_twoisogeny::<1013, 2, 1, 2>(&theta.null, *m, i);
        theta = Theta { null: new_null };
    }
    let ram = theta_to_rosenhain::<1013, 2, 1, 2>(&theta.null);
    let expected = ram.IgusaInvariants();
    // The octal string "0123" encodes the indices 0, 1, 2, 3.
    let result = LTZ_hash::<1013, 2, 1, 2>(i, "0123");
    assert_eq!(result, expected);
}

use LTZ_hash::{LTZ_hash_auto, LTZ_hash_auto_with_init, path_to_Fp2_auto};

#[test]
fn ltz_hash_auto_macro() {
    // Using the macro should yield the same result as the manual call above.
    let expected = LTZ_hash_auto!(1013, "0123");
    let manual_i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );
    let init = hash::generate_hash_init_curve::<1013, 2, 1, 2>();
    let manual = LTZ_hash_with_init::<1013, 2, 1, 2>(manual_i, "0123", init);
    assert_eq!(expected, manual);
}

#[test]
fn ltz_hash_with_custom_init() {
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );
    let init = Rosenhain::<1013, 2, 1, 2>::generate_init_curve();
    let default_res = LTZ_hash::<1013, 2, 1, 2>(i, "0123");
    let custom_res = LTZ_hash_with_init::<1013, 2, 1, 2>(i, "0123", init);
    assert_eq!(default_res, custom_res);
}

#[test]
fn path_to_fp2_auto_with_init_macro() {
    let init = hash::generate_hash_init_curve::<1013, 2, 1, 2>();
    let path_default = path_to_Fp2_auto!(1013, 3);
    let path_custom = path_to_Fp2_auto!(1013, init.la, init.mu, init.nu, 3);
    assert_eq!(path_default, path_custom);
}

#[test]
fn ltz_hash_auto_with_init_macro() {
    let init = Rosenhain::<1013, 2, 1, 2>::generate_init_curve();
    let res = LTZ_hash_auto_with_init!(1013, init.la, init.mu, init.nu, "0123");
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );
    let manual = LTZ_hash_with_init::<1013, 2, 1, 2>(i, "0123", init);
    assert_eq!(res, manual);
}

#[test]
fn generate_hash_init_curve_function() {
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );

    let mut theta = Theta::<1013, 2, 1, 2>::new(
        Rosenhain::<1013, 2, 1, 2>::generate_init_curve(),
    );
    for _ in 0..10 {
        let new_null = compute_twoisogeny::<1013, 2, 1, 2>(&theta.null, 0, i);
        theta = Theta { null: new_null };
    }
    let expected = theta_to_rosenhain::<1013, 2, 1, 2>(&theta.null);

    let generated = LTZ_hash::hash::generate_hash_init_curve::<1013, 2, 1, 2>();
    assert_eq!(generated, expected);
}

#[test]
fn invariants_to_message_roundtrip() {
    let i = F1013Ext4::new(
        F1013Ext::new(F1013::new(45), F1013::new(0)),
        F1013Ext::zero(),
    );

    let init = hash::generate_hash_init_curve::<1013, 2, 1, 2>();
    let mut theta = Theta::<1013, 2, 1, 2>::new(init);
    let mut invariants = Vec::new();
    invariants.push(init.IgusaInvariants());

    let message: String = (0..100)
        .map(|i| std::char::from_digit((i % 8) as u32, 8).unwrap())
        .collect();

    for ch in message.chars() {
        let digit = ch.to_digit(8).unwrap();
        let new_null =
            compute_twoisogeny::<1013, 2, 1, 2>(&theta.null, digit as usize, i);
        theta = Theta { null: new_null };
        let ram = theta_to_rosenhain::<1013, 2, 1, 2>(&theta.null);
        invariants.push(ram.IgusaInvariants());
    }

    let recovered = hash::invariants_to_message::<1013, 2, 1, 2>(&invariants);
    assert_eq!(recovered, message);
}
