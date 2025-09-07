use crate::{Fp4, rosenhain::Rosenhain};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Theta<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> {
    pub null: [Fp4<p, D1, D2C0, D2C1>; 16],
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>
    Theta<p, D1, D2C0, D2C1>
{
    /// Construct the theta structure from Rosenhain invariants.
    /// This is a direct translation of the magma function `rosenhain_to_theta`.
    pub fn new(ram: Rosenhain<p, D1, D2C0, D2C1>) -> Self {
        let la = ram.la;
        let mu = ram.mu;
        let nu = ram.nu;

        // initialize array with zeros
        let mut null = [Fp4::<p, D1, D2C0, D2C1>::zero(); 16];

        null[0] = Fp4::<p, D1, D2C0, D2C1>::one();
        null[4] = (mu / (la * nu))
            .sqrt()
            .expect("sqrt failure in rosenhain_to_theta");
        null[8] = ((mu * (nu - Fp4::one()) * (la - mu))
            / (nu * (mu - Fp4::one()) * (la - nu)))
            .sqrt()
            .expect("sqrt failure in rosenhain_to_theta");
        null[1] = ((mu * (nu - Fp4::one()) * (la - Fp4::one()))
            / (la * nu * (mu - Fp4::one())))
            .sqrt()
            .expect("sqrt failure in rosenhain_to_theta");
        null[2] = ((mu * (la - Fp4::one()) * (nu - mu))
            / (la * (mu - Fp4::one()) * (nu - la)))
            .sqrt()
            .expect("sqrt failure in rosenhain_to_theta");
        null[6] = null[2] / (nu * null[4]);
        null[12] = null[8] / (la * null[4]);
        null[3] = (nu - Fp4::one()) * (null[4] * null[6]) / null[1];
        null[9] = (la - Fp4::one()) * (null[4] * null[12]) / null[1];
        null[15] = (null[0] * null[3] - null[1] * null[2]) / null[12];

        Self { null }
    }
}

pub const TRANS_TABLE: [[[u8; 16]; 15]; 2] = [
    [
        [0, 1, 2, 3, 4, 0, 6, 0, 8, 9, 0, 0, 12, 0, 0, 15],
        [4, 1, 6, 3, 0, 0, 2, 0, 12, 9, 0, 0, 8, 0, 0, 15],
        [0, 9, 6, 15, 4, 0, 2, 0, 8, 1, 0, 0, 12, 0, 0, 3],
        [4, 9, 2, 15, 0, 0, 6, 0, 12, 1, 0, 0, 8, 0, 0, 3],
        [8, 9, 2, 3, 12, 0, 6, 0, 0, 1, 0, 0, 4, 0, 0, 15],
        [12, 9, 6, 3, 8, 0, 2, 0, 4, 1, 0, 0, 0, 0, 0, 15],
        [8, 1, 6, 15, 12, 0, 2, 0, 0, 9, 0, 0, 4, 0, 0, 3],
        [12, 1, 2, 15, 8, 0, 6, 0, 4, 9, 0, 0, 0, 0, 0, 3],
        [1, 4, 3, 6, 0, 0, 2, 0, 9, 12, 0, 0, 8, 0, 0, 15],
        [2, 3, 8, 9, 6, 0, 12, 0, 0, 1, 0, 0, 4, 0, 0, 15],
        [9, 12, 3, 6, 8, 0, 2, 0, 1, 4, 0, 0, 0, 0, 0, 15],
        [6, 3, 12, 9, 2, 0, 8, 0, 4, 1, 0, 0, 0, 0, 0, 15],
        [9, 4, 15, 2, 0, 0, 6, 0, 1, 12, 0, 0, 8, 0, 0, 3],
        [1, 12, 15, 2, 8, 0, 6, 0, 9, 4, 0, 0, 0, 0, 0, 3],
        [0, 9, 6, 15, 2, 0, 4, 0, 1, 8, 0, 0, 3, 0, 0, 12],
    ],
    [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3],
        [0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
        [0, 3, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1],
        [0, 0, 3, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        [0, 3, 3, 2, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 2],
        [0, 0, 3, 1, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 3, 3, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1],
        [0, 0, 3, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 3, 3, 2, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 3, 3, 2, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 3, 2, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3],
        [0, 3, 1, 2, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 2],
        [0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    ],
];

pub const ISOGY_TABLE: [[[u8; 16]; 4]; 2] = [
    [
        [0, 1, 2, 3, 0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 0, 3],
        [1, 0, 3, 2, 1, 0, 3, 0, 1, 0, 0, 0, 1, 0, 0, 2],
        [2, 3, 0, 1, 2, 0, 0, 0, 2, 3, 0, 0, 2, 0, 0, 1],
        [3, 2, 1, 0, 3, 0, 1, 0, 3, 2, 0, 0, 3, 0, 0, 0],
    ],
    [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1],
        [0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0],
    ],
];

/// Transform a squared theta null-point by the `m`-th symplectic matrix.
pub fn transform_nullpoint<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    null: &[Fp4<p, D1, D2C0, D2C1>; 16],
    m: usize,
    i: Fp4<p, D1, D2C0, D2C1>,
) -> [Fp4<p, D1, D2C0, D2C1>; 16] {
    let mut trans_null = [Fp4::<p, D1, D2C0, D2C1>::zero(); 16];
    let index_list = &TRANS_TABLE[0][m];
    let sign_list = &TRANS_TABLE[1][m];

    for &num in [0usize, 1, 2, 3, 4, 6, 8, 9, 12, 15].iter() {
        let index = index_list[num] as usize;
        let sign = sign_list[num];
        trans_null[index] = match sign {
            0 => null[num],
            1 => i * null[num],
            2 => -null[num],
            _ => -(i * null[num]),
        };
    }

    trans_null
}

/// Compute the squared theta null-point of the `m`-th (2,2)-isogenous surface.
pub fn compute_twoisogeny<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    null: &[Fp4<p, D1, D2C0, D2C1>; 16],
    m: usize,
    i: Fp4<p, D1, D2C0, D2C1>,
) -> [Fp4<p, D1, D2C0, D2C1>; 16] {
    let trans_null = transform_nullpoint::<p, D1, D2C0, D2C1>(null, m, i);

    let sqrt_null = [
        trans_null[0],
        (trans_null[0] * trans_null[1])
            .sqrt()
            .expect("sqrt failure in compute_twoisogeny"),
        (trans_null[0] * trans_null[2])
            .sqrt()
            .expect("sqrt failure in compute_twoisogeny"),
        (trans_null[0] * trans_null[3])
            .sqrt()
            .expect("sqrt failure in compute_twoisogeny"),
    ];

    let mut product_matrix = [[Fp4::<p, D1, D2C0, D2C1>::zero(); 4]; 4];
    for k in 0..4 {
        for j in 0..=k {
            let value = sqrt_null[j] * sqrt_null[k];
            product_matrix[j][k] = value;
            product_matrix[k][j] = value;
        }
    }

    let mut image_null = [Fp4::<p, D1, D2C0, D2C1>::zero(); 16];
    for &num in [0usize, 1, 2, 3, 4, 6, 8, 9, 12, 15].iter() {
        let mut value = Fp4::<p, D1, D2C0, D2C1>::zero();
        for k in 0..4 {
            let index = ISOGY_TABLE[0][k][num] as usize;
            let min = index.min(k);
            let max = index.max(k);
            if ISOGY_TABLE[1][k][num] == 0 {
                value += product_matrix[min][max];
            } else {
                value -= product_matrix[min][max];
            }
        }
        image_null[num] = value;
    }

    image_null
}

/// Recover Rosenhain invariants from a squared theta null-point.
pub fn theta_to_rosenhain<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
    null: &[Fp4<p, D1, D2C0, D2C1>; 16],
) -> Rosenhain<p, D1, D2C0, D2C1> {
    // Indices in the Magma reference are 1-based.  Convert them to Rust's 0-based
    // indexing when accessing the squared theta constants.
    let la = (null[0] * null[8]) / (null[4] * null[12]);
    let mu = (null[8] * null[2]) / (null[12] * null[6]);
    let nu = (null[2] * null[0]) / (null[6] * null[4]);

    Rosenhain::new(la, mu, nu)
}

