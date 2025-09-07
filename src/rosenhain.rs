use crate::{Fp, Fp2, Fp4};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]

pub struct Rosenhain<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> {
    pub la: Fp4<p, D1, D2C0, D2C1>,
    pub mu: Fp4<p, D1, D2C0, D2C1>,
    pub nu: Fp4<p, D1, D2C0, D2C1>,
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Rosenhain<p, D1, D2C0, D2C1> {
    pub fn new(
        la: Fp4<p, D1, D2C0, D2C1>,
        mu: Fp4<p, D1, D2C0, D2C1>,
        nu: Fp4<p, D1, D2C0, D2C1>,
    ) -> Self {
        Self { la, mu, nu }
    }

    /// Generate Rosenhain invariants for an initial curve isomorphic to
    /// `y^2 = x^5 + 1`.
    pub fn generate_init_curve() -> Self {
        assert!(p > 5);

        // Helper to lift an Fp element into Fp4
        fn lift<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
            a: Fp<p>,
        ) -> Fp4<p, D1, D2C0, D2C1> {
            Fp4::<p, D1, D2C0, D2C1>::new(
                Fp2::<p, D1>::new(a, Fp::<p>::zero()),
                Fp2::<p, D1>::zero(),
            )
        }

        // Polynomial represented from constant coefficient upwards
        type Poly<const p: u64> = Vec<Fp<p>>;

        fn poly_mul<const p: u64>(a: &Poly<p>, b: &Poly<p>) -> Poly<p> {
            let mut res = vec![Fp::<p>::zero(); a.len() + b.len() - 1];
            for i in 0..a.len() {
                for j in 0..b.len() {
                    res[i + j] += a[i] * b[j];
                }
            }
            res
        }

        fn poly_trim<const p: u64>(poly: &mut Poly<p>) {
            while let Some(true) = poly.last().map(|c| *c == Fp::<p>::zero()) {
                poly.pop();
            }
        }

        fn poly_deriv<const p: u64>(poly: &Poly<p>) -> Poly<p> {
            if poly.len() <= 1 {
                return Vec::new();
            }
            let mut res = Vec::with_capacity(poly.len() - 1);
            for i in 1..poly.len() {
                res.push(poly[i] * (i as u32));
            }
            res
        }

        fn poly_mod<const p: u64>(mut a: Poly<p>, b: &Poly<p>) -> Poly<p> {
            poly_trim::<p>(&mut a);
            let mut b_norm = b.clone();
            poly_trim::<p>(&mut b_norm);
            if b_norm.is_empty() {
                return a;
            }
            let b_deg = b_norm.len() - 1;
            let b_lead_inv = b_norm[b_deg].inverse();
            while a.len() >= b_norm.len() && !a.is_empty() {
                let coeff = a[a.len() - 1] * b_lead_inv;
                let shift = a.len() - b_norm.len();
                for i in 0..b_norm.len() {
                    a[shift + i] -= coeff * b_norm[i];
                }
                poly_trim::<p>(&mut a);
            }
            a
        }

        fn poly_gcd<const p: u64>(mut a: Poly<p>, mut b: Poly<p>) -> Poly<p> {
            poly_trim::<p>(&mut a);
            poly_trim::<p>(&mut b);
            while !b.is_empty() {
                let r = poly_mod::<p>(a, &b);
                a = b;
                b = r;
            }
            if !a.is_empty() {
                let lead_inv = a[a.len() - 1].inverse();
                for c in a.iter_mut() {
                    *c *= lead_inv;
                }
            }
            a
        }

        fn resultant<const p: u64>(f: &Poly<p>, g: &Poly<p>) -> Fp<p> {
            let mut f = f.clone();
            let mut g = g.clone();
            poly_trim::<p>(&mut f);
            poly_trim::<p>(&mut g);
            if f.is_empty() || g.is_empty() {
                return Fp::<p>::zero();
            }
            let n = f.len() - 1;
            let m = g.len() - 1;
            let size = n + m;
            let mut mat = vec![vec![Fp::<p>::zero(); size]; size];

            for row in 0..m {
                for (j, coef) in f.iter().enumerate() {
                    if j + row < size {
                        mat[row][j + row] = *coef;
                    }
                }
            }

            for row in 0..n {
                for (j, coef) in g.iter().enumerate() {
                    if j + row < size {
                        mat[m + row][j + row] = *coef;
                    }
                }
            }

            fn det<const p: u64>(mut mat: Vec<Vec<Fp<p>>>) -> Fp<p> {
                let n = mat.len();
                let mut det = Fp::<p>::one();
                for i in 0..n {
                    let mut pivot = i;
                    while pivot < n && mat[pivot][i] == Fp::<p>::zero() {
                        pivot += 1;
                    }
                    if pivot == n {
                        return Fp::<p>::zero();
                    }
                    if pivot != i {
                        mat.swap(i, pivot);
                        det = -det;
                    }
                    let pivot_val = mat[i][i];
                    det *= pivot_val;
                    let inv = pivot_val.inverse();
                    for j in i..n {
                        mat[i][j] *= inv;
                    }
                    let row_i = mat[i].clone();
                    for k in (i + 1)..n {
                        if mat[k][i] != Fp::<p>::zero() {
                            let factor = mat[k][i];
                            for j in i..n {
                                mat[k][j] -= row_i[j] * factor;
                            }
                        }
                    }
                }
                det
            }

            det::<p>(mat)
        }

        fn poly_discriminant<const p: u64>(poly: &Poly<p>) -> Fp<p> {
            let deriv = poly_deriv::<p>(poly);
            resultant::<p>(poly, &deriv)
        }

        fn poly_pow<const p: u64>(mut base: Poly<p>, mut exp: u64) -> Poly<p> {
            let mut result = vec![Fp::<p>::one()];
            while exp > 0 {
                if exp % 2 == 1 {
                    result = poly_mul(&result, &base);
                }
                if exp > 1 {
                    base = poly_mul(&base, &base);
                }
                exp /= 2;
            }
            result
        }

        fn coeff_at<const p: u64>(poly: &Poly<p>, deg: usize) -> Fp<p> {
            if deg < poly.len() { poly[deg] } else { Fp::<p>::zero() }
        }

        // Construct the polynomial f
        if p % 5 == 2 || p % 5 == 3 {
            let zeta = Fp4::<p, D1, D2C0, D2C1>::fifth_root_of_unity();
            let denom = zeta - Fp4::<p, D1, D2C0, D2C1>::one();
            let la = (zeta.pow(2) - Fp4::<p, D1, D2C0, D2C1>::one()) / denom;
            let mu = (zeta.pow(3) - Fp4::<p, D1, D2C0, D2C1>::one()) / denom;
            let nu = (zeta.pow(4) - Fp4::<p, D1, D2C0, D2C1>::one()) / denom;

            Self::new(la, mu, nu)
        } else {
            let e = (p - 1) / 2;
            let mut found = None;
            let f: Poly<p> = {
                'outer: for f0 in 0..p {
                    for f1 in 0..p {
                        for f2 in 0..p {
                            for f3 in 0..p {
                                let poly = vec![
                                    Fp::<p>::new(f0),
                                    Fp::<p>::new(f1),
                                    Fp::<p>::new(f2),
                                    Fp::<p>::new(f3),
                                    Fp::<p>::zero(),
                                    Fp::<p>::one(),
                                ];
                                let g = poly_pow::<p>(poly.clone(), e);
                                let a = coeff_at::<p>(&g, (p - 1) as usize);
                                let b = coeff_at::<p>(&g, (2 * p - 1) as usize);
                                let c = coeff_at::<p>(&g, (p - 2) as usize);
                                let d = coeff_at::<p>(&g, (2 * p - 2) as usize);
                                if a == Fp::<p>::zero()
                                    && b == Fp::<p>::zero()
                                    && c == Fp::<p>::zero()
                                    && d == Fp::<p>::zero()
                                {
                                    continue;
                                }
                                let disc = poly_discriminant::<p>(&poly);
                                if disc != Fp::<p>::zero()
                                    && a * d - b * c == Fp::<p>::zero()
                                    && a * b.pow(p - 1) + d.pow(p) == Fp::<p>::zero()
                                    && a.pow(p) + c.pow(p - 1) * d == Fp::<p>::zero()
                                {
                                    found = Some(poly);
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                found.expect("no polynomial found")
            };

            // Evaluate f and find its five roots in Fp4
            fn eval_f<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>(
                poly: &Poly<p>,
                x: Fp4<p, D1, D2C0, D2C1>,
            ) -> Fp4<p, D1, D2C0, D2C1> {
                let mut result = Fp4::<p, D1, D2C0, D2C1>::zero();
                for coeff in poly.iter().rev() {
                    result *= x;
                    result += lift::<p, D1, D2C0, D2C1>(*coeff);
                }
                result
            }

            let mut roots = Vec::new();
            'search: for a0 in 0..p {
                for a1 in 0..p {
                    for b0 in 0..p {
                        for b1 in 0..p {
                            let elem = Fp4::<p, D1, D2C0, D2C1>::new(
                                Fp2::<p, D1>::new(Fp::<p>::new(a0), Fp::<p>::new(a1)),
                                Fp2::<p, D1>::new(Fp::<p>::new(b0), Fp::<p>::new(b1)),
                            );
                            if eval_f::<p, D1, D2C0, D2C1>(&f, elem) == Fp4::<p, D1, D2C0, D2C1>::zero() {
                                roots.push(elem);
                                if roots.len() == 5 {
                                    break 'search;
                                }
                            }
                        }
                    }
                }
            }

            assert!(roots.len() == 5, "failed to find 5 roots");

            let denom = roots[1] - roots[0];
            let la = (roots[2] - roots[0]) / denom;
            let mu = (roots[3] - roots[0]) / denom;
            let nu = (roots[4] - roots[0]) / denom;

            Self::new(la, mu, nu)
        }
    }

    /// Compute the Igusa invariants from the Rosenhain coefficients.
    pub fn IgusaInvariants(
        &self,
    ) -> (
        Fp4<p, D1, D2C0, D2C1>,
        Fp4<p, D1, D2C0, D2C1>,
        Fp4<p, D1, D2C0, D2C1>,
    ) {
        const G2: [[usize; 6]; 15] = [
            [0, 1, 2, 3, 4, 5],
            [2, 0, 1, 3, 4, 5],
            [3, 0, 1, 2, 4, 5],
            [4, 0, 1, 2, 3, 5],
            [5, 0, 1, 2, 3, 4],
            [4, 1, 0, 2, 3, 5],
            [5, 1, 0, 2, 3, 4],
            [4, 2, 0, 1, 3, 5],
            [5, 2, 0, 1, 3, 4],
            [3, 0, 4, 1, 2, 5],
            [4, 0, 3, 1, 2, 5],
            [5, 0, 3, 1, 2, 4],
            [5, 1, 3, 0, 2, 4],
            [2, 3, 4, 0, 1, 5],
            [5, 0, 4, 1, 2, 3],
        ];

        const G4: [[usize; 6]; 10] = [
            [0, 1, 2, 3, 4, 5],
            [3, 0, 1, 2, 4, 5],
            [4, 0, 1, 2, 3, 5],
            [5, 0, 1, 2, 3, 4],
            [2, 3, 0, 1, 4, 5],
            [4, 2, 0, 1, 3, 5],
            [5, 2, 0, 1, 3, 4],
            [3, 4, 0, 1, 2, 5],
            [5, 3, 0, 1, 2, 4],
            [4, 5, 0, 1, 2, 3],
        ];

        const G6: [[usize; 6]; 60] = [
            [0, 1, 2, 3, 4, 5],
            [1, 0, 2, 3, 4, 5],
            [2, 0, 1, 3, 4, 5],
            [3, 0, 1, 2, 4, 5],
            [4, 0, 1, 2, 3, 5],
            [5, 0, 1, 2, 3, 4],
            [0, 2, 1, 3, 4, 5],
            [1, 2, 0, 3, 4, 5],
            [2, 1, 0, 3, 4, 5],
            [3, 1, 0, 2, 4, 5],
            [4, 1, 0, 2, 3, 5],
            [5, 1, 0, 2, 3, 4],
            [0, 3, 1, 2, 4, 5],
            [1, 3, 0, 2, 4, 5],
            [2, 3, 0, 1, 4, 5],
            [3, 2, 0, 1, 4, 5],
            [4, 2, 0, 1, 3, 5],
            [5, 2, 0, 1, 3, 4],
            [0, 4, 1, 2, 3, 5],
            [1, 4, 0, 2, 3, 5],
            [2, 4, 0, 1, 3, 5],
            [3, 4, 0, 1, 2, 5],
            [4, 3, 0, 1, 2, 5],
            [5, 3, 0, 1, 2, 4],
            [0, 5, 1, 2, 3, 4],
            [1, 5, 0, 2, 3, 4],
            [2, 5, 0, 1, 3, 4],
            [3, 5, 0, 1, 2, 4],
            [4, 5, 0, 1, 2, 3],
            [5, 4, 0, 1, 2, 3],
            [0, 1, 3, 2, 4, 5],
            [1, 0, 3, 2, 4, 5],
            [2, 0, 3, 1, 4, 5],
            [3, 0, 2, 1, 4, 5],
            [4, 0, 2, 1, 3, 5],
            [5, 0, 2, 1, 3, 4],
            [0, 2, 3, 1, 4, 5],
            [1, 2, 3, 0, 4, 5],
            [2, 1, 3, 0, 4, 5],
            [0, 3, 2, 1, 4, 5],
            [1, 3, 2, 0, 4, 5],
            [2, 3, 1, 0, 4, 5],
            [0, 4, 2, 1, 3, 5],
            [1, 4, 2, 0, 3, 5],
            [2, 4, 1, 0, 3, 5],
            [0, 5, 2, 1, 3, 4],
            [1, 5, 2, 0, 3, 4],
            [2, 5, 1, 0, 3, 4],
            [0, 1, 4, 2, 3, 5],
            [1, 0, 4, 2, 3, 5],
            [2, 0, 4, 1, 3, 5],
            [3, 0, 4, 1, 2, 5],
            [0, 2, 4, 1, 3, 5],
            [1, 2, 4, 0, 3, 5],
            [2, 1, 4, 0, 3, 5],
            [3, 1, 4, 0, 2, 5],
            [0, 3, 4, 1, 2, 5],
            [1, 3, 4, 0, 2, 5],
            [2, 3, 4, 0, 1, 5],
            [3, 2, 4, 0, 1, 5],
        ];

        let mut a = Fp4::<p, D1, D2C0, D2C1>::one();
        a += a;
        while a == self.la || a == self.mu || a == self.nu {
            a += Fp4::<p, D1, D2C0, D2C1>::one();
        }

        let rs = [
            Fp4::<p, D1, D2C0, D2C1>::zero(),
            Fp4::<p, D1, D2C0, D2C1>::one(),
            (Fp4::<p, D1, D2C0, D2C1>::one() - a).inverse(),
            self.la * (self.la - a).inverse(),
            self.mu * (self.mu - a).inverse(),
            self.nu * (self.nu - a).inverse(),
        ];

        let mut i2 = Fp4::<p, D1, D2C0, D2C1>::zero();
        for g in G2.iter() {
            let t = (rs[g[0]] - rs[g[1]])
                * (rs[g[2]] - rs[g[3]])
                * (rs[g[4]] - rs[g[5]]);
            i2 += t * t;
        }

        let mut i4 = Fp4::<p, D1, D2C0, D2C1>::zero();
        for g in G4.iter() {
            let t = (rs[g[0]] - rs[g[1]])
                * (rs[g[1]] - rs[g[2]])
                * (rs[g[2]] - rs[g[0]])
                * (rs[g[3]] - rs[g[4]])
                * (rs[g[4]] - rs[g[5]])
                * (rs[g[5]] - rs[g[3]]);
            i4 += t * t;
        }

        let mut i6 = Fp4::<p, D1, D2C0, D2C1>::zero();
        for g in G6.iter() {
            let t = (rs[g[0]] - rs[g[1]])
                * (rs[g[1]] - rs[g[2]])
                * (rs[g[2]] - rs[g[0]])
                * (rs[g[3]] - rs[g[4]])
                * (rs[g[4]] - rs[g[5]])
                * (rs[g[5]] - rs[g[3]])
                * (rs[g[0]] - rs[g[3]])
                * (rs[g[1]] - rs[g[4]])
                * (rs[g[2]] - rs[g[5]]);
            i6 += t * t;
        }

        let mut i10 = Fp4::<p, D1, D2C0, D2C1>::one();
        for i in 0..6 {
            for j in (i + 1)..6 {
                let diff = rs[i] - rs[j];
                i10 *= diff * diff;
            }
        }

        let j2 = i2/(8u32);
        let j4 = (4u32*j2*j2 - i4)/(96u32);
        let j6 = (8u32*j2.pow(3) - 160u32*j2*j4 - i6)/(576u32);
        let j8 = (j2*j6 - j4*j4)/(4u32);
        let j10 = i10/(4096u32);

        if j2 != Fp4::<p, D1, D2C0, D2C1>::zero() {
            (j2.pow(5)/j10, j2.pow(3)*j4/j10, j2.pow(2)*j6/j10)
        } else if j4 != Fp4::<p, D1, D2C0, D2C1>::zero() {
            (Fp4::<p, D1, D2C0, D2C1>::zero(), j4.pow(5)/j10.pow(2), j4*j6/j10)
        } else {
            (
                Fp4::<p, D1, D2C0, D2C1>::zero(),
                Fp4::<p, D1, D2C0, D2C1>::zero(),
                j6.pow(5)/j10.pow(3),
            )
        }
    }
}
