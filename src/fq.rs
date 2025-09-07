#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fp<const p: u64> {
    value: u64,
}

impl<const p: u64> Fp<p> {
    /// Create a new field element, reducing the input modulo `p`.
    pub fn new(value: u64) -> Self {
        Self { value: value % p }
    }

    /// The additive identity element.
    pub fn zero() -> Self {
        Self { value: 0 }
    }

    /// The multiplicative identity element.
    pub fn one() -> Self {
        Self { value: 1 % p }
    }

    /// Access the underlying integer representation.
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Exponentiate by `exp` using binary exponentiation.
    pub fn pow(mut self, mut exp: u64) -> Self {
        let mut result = Self::one();
        while exp > 0 {
            if exp % 2 == 1 {
                result *= self;
            }
            self *= self;
            exp /= 2;
        }
        result
    }

    /// Multiplicative inverse using Fermat's little theorem.
    /// Panics if `self` is zero.
    pub fn inverse(self) -> Self {
        assert!(self.value != 0, "zero has no multiplicative inverse");
        self.pow(p - 2)
    }
}

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign, Div, DivAssign};
use core::hash::{Hash, Hasher};
use core::fmt;

impl<const p: u64> fmt::Display for Fp<p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const p: u64> Hash for Fp<p> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<const p: u64> Add for Fp<p> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.value + rhs.value)
    }
}

impl<const p: u64> AddAssign for Fp<p> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        self.value %= p;
    }
}

impl<const p: u64> Sub for Fp<p> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new((self.value + p - rhs.value % p) % p)
    }
}

impl<const p: u64> SubAssign for Fp<p> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = (self.value + p - rhs.value % p) % p;
    }
}

impl<const p: u64> Neg for Fp<p> {
    type Output = Self;
    fn neg(self) -> Self {
        if self.value == 0 {
            self
        } else {
            Self::new(p - self.value)
        }
    }
}

impl<const p: u64> Mul for Fp<p> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new((self.value * rhs.value) % p)
    }
}

impl<const p: u64> MulAssign for Fp<p> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.value) % p;
    }
}

impl<const p: u64> Div for Fp<p> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse()
    }
}

impl<const p: u64> DivAssign for Fp<p> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse();
    }
}

impl<const p: u64> Div<u64> for Fp<p> {
    type Output = Self;
    fn div(self, rhs: u64) -> Self {
        self / Self::new(rhs)
    }
}

impl<const p: u64> DivAssign<u64> for Fp<p> {
    fn div_assign(&mut self, rhs: u64) {
        *self /= Self::new(rhs);
    }
}

impl<const p: u64> Div<i64> for Fp<p> {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        let r = ((rhs % p as i64) + p as i64) as u64 % p;
        self / Self::new(r)
    }
}

impl<const p: u64> DivAssign<i64> for Fp<p> {
    fn div_assign(&mut self, rhs: i64) {
        let r = ((rhs % p as i64) + p as i64) as u64 % p;
        *self /= Self::new(r);
    }
}

impl<const p: u64> Mul<i32> for Fp<p> {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        Self::new((self.value * r) % p)
    }
}

impl<const p: u64> MulAssign<i32> for Fp<p> {
    fn mul_assign(&mut self, rhs: i32) {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        self.value = (self.value * r) % p;
    }
}

impl<const p: u64> core::ops::Mul<Fp<p>> for i32 {
    type Output = Fp<p>;
    fn mul(self, rhs: Fp<p>) -> Fp<p> {
        rhs * self
    }
}

impl<const p: u64> Mul<u32> for Fp<p> {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self {
        Self::new((self.value * (rhs as u64)) % p)
    }
}

impl<const p: u64> MulAssign<u32> for Fp<p> {
    fn mul_assign(&mut self, rhs: u32) {
        self.value = (self.value * (rhs as u64)) % p;
    }
}

impl<const p: u64> core::ops::Mul<Fp<p>> for u32 {
    type Output = Fp<p>;
    fn mul(self, rhs: Fp<p>) -> Fp<p> {
        rhs * self
    }
}

impl<const p: u64> Div<u32> for Fp<p> {
    type Output = Self;
    fn div(self, rhs: u32) -> Self {
        let r = rhs as u64 % p;
        self / Self::new(r)
    }
}

impl<const p: u64> DivAssign<u32> for Fp<p> {
    fn div_assign(&mut self, rhs: u32) {
        let r = rhs as u64 % p;
        *self /= Self::new(r);
    }
}

impl<const p: u64> Div<i32> for Fp<p> {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        self / Self::new(r)
    }
}

impl<const p: u64> DivAssign<i32> for Fp<p> {
    fn div_assign(&mut self, rhs: i32) {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        *self /= Self::new(r);
    }
}

/// Helper: modular exponentiation for `u64` values.
pub const fn mod_pow(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    let mut result = 1u64 % modu;
    base %= modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(base) % modu;
        }
        base = base.wrapping_mul(base) % modu;
        exp /= 2;
    }
    result
}

/// Helper: Legendre symbol to test quadratic residues.
pub const fn legendre_symbol(a: u64, p: u64) -> i32 {
    if a % p == 0 {
        0
    } else {
        let r = mod_pow(a % p, (p - 1) / 2, p);
        if r == 1 { 1 } else { -1 }
    }
}

/// Find the smallest quadratic non-residue mod `p`.
pub const fn find_non_residue(p: u64) -> u64 {
    let mut d = 2u64;
    while legendre_symbol(d, p) != -1 {
        d += 1;
    }
    d
}

/// Helper structure for constant-time `Fp2` computations in const contexts.
#[derive(Copy, Clone)]
pub struct Cf2 {
    pub c0: u64,
    pub c1: u64,
}

/// Multiply two `Cf2` elements in `Fp2(p, D1)`.
pub const fn fp2_mul(a: Cf2, b: Cf2, p: u64, d1: u64) -> Cf2 {
    let c0 = (a.c0.wrapping_mul(b.c0) + d1.wrapping_mul(a.c1).wrapping_mul(b.c1)) % p;
    let c1 = (a.c0.wrapping_mul(b.c1) + a.c1.wrapping_mul(b.c0)) % p;
    Cf2 { c0, c1 }
}

/// Exponentiate a `Cf2` element by `exp`.
pub const fn fp2_pow(mut base: Cf2, mut exp: u128, p: u64, d1: u64) -> Cf2 {
    let mut result = Cf2 { c0: 1 % p, c1: 0 };
    while exp > 0 {
        if exp % 2 == 1 {
            result = fp2_mul(result, base, p, d1);
        }
        base = fp2_mul(base, base, p, d1);
        exp /= 2;
    }
    result
}

/// Determine if a `Cf2` element is a quadratic residue.
pub const fn fp2_is_square(a: Cf2, p: u64, d1: u64) -> bool {
    if a.c0 == 0 && a.c1 == 0 {
        return true;
    }
    let exp = (p as u128 * p as u128 - 1) / 2;
    let r = fp2_pow(a, exp, p, d1);
    r.c0 % p == 1 % p && r.c1 % p == 0
}

/// Pick a small non-square in `Fp2`.
pub const fn find_non_residue_fp2(p: u64, d1: u64) -> Cf2 {
    let cand = [
        Cf2 { c0: 1, c1: 2 },
        Cf2 { c0: 0, c1: 1 },
        Cf2 { c0: 1, c1: 1 },
        Cf2 { c0: 2, c1: 1 },
    ];
    let mut i = 0usize;
    while i < cand.len() {
        if !fp2_is_square(cand[i], p, d1) {
            return cand[i];
        }
        i += 1;
    }
    Cf2 { c0: 1, c1: 0 }
}

/// Quadratic extension field over `Fp` defined by the relation `x^2 = D`.
///
/// For primes `p` where `p % 4 == 3` this corresponds to `D = p - 1` (i.e. `x^2 + 1`).
/// Otherwise choose a small quadratic non-residue `D` so that `x^2 - D` is irreducible.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fp2<const p: u64, const D: u64> {
    pub c0: Fp<p>,
    pub c1: Fp<p>,
}

impl<const p: u64, const D: u64> Fp2<p, D> {
    /// Create a new element `c0 + c1 * x`.
    pub fn new(c0: Fp<p>, c1: Fp<p>) -> Self {
        Self { c0, c1 }
    }

    /// The additive identity element.
    pub fn zero() -> Self {
        Self {
            c0: Fp::<p>::zero(),
            c1: Fp::<p>::zero(),
        }
    }

    /// The multiplicative identity element.
    pub fn one() -> Self {
        Self {
            c0: Fp::<p>::one(),
            c1: Fp::<p>::zero(),
        }
    }

    /// Exponentiation by `exp` using binary exponentiation.
    pub fn pow(mut self, mut exp: u64) -> Self {
        let mut result = Self::one();
        while exp > 0 {
            if exp % 2 == 1 {
                result *= self;
            }
            self *= self;
            exp /= 2;
        }
        result
    }

    /// Conjugate of `a + bx` is `a - bx`.
    pub fn conjugate(self) -> Self {
        Self {
            c0: self.c0,
            c1: -self.c1,
        }
    }

    /// Norm of the element: `a^2 - D * b^2`.
    pub fn norm(self) -> Fp<p> {
        let d = Fp::<p>::new(D);
        let t0 = self.c0 * self.c0;
        let t1 = self.c1 * self.c1 * d;
        t0 - t1
    }

    /// Multiplicative inverse using the norm and conjugate.
    /// Panics if `self` is zero.
    pub fn inverse(self) -> Self {
        assert!(self != Self::zero(), "zero has no multiplicative inverse");
        let n = self.norm();
        let inv_n = n.inverse();
        let conj = self.conjugate();
        Self {
            c0: conj.c0 * inv_n,
            c1: conj.c1 * inv_n,
        }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> fmt::Display
    for Fp4<p, D1, D2C0, D2C1>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {}*y)", self.c0, self.c1)
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Hash
    for Fp4<p, D1, D2C0, D2C1>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.c0.hash(state);
        self.c1.hash(state);
    }
}

impl<const p: u64, const D: u64> fmt::Display for Fp2<p, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {}*x)", self.c0, self.c1)
    }
}

impl<const p: u64, const D: u64> Hash for Fp2<p, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.c0.hash(state);
        self.c1.hash(state);
    }
}

impl<const p: u64, const D: u64> Add for Fp2<p, D> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            c0: self.c0 + rhs.c0,
            c1: self.c1 + rhs.c1,
        }
    }
}

impl<const p: u64, const D: u64> AddAssign for Fp2<p, D> {
    fn add_assign(&mut self, rhs: Self) {
        self.c0 += rhs.c0;
        self.c1 += rhs.c1;
    }
}

impl<const p: u64, const D: u64> Sub for Fp2<p, D> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            c0: self.c0 - rhs.c0,
            c1: self.c1 - rhs.c1,
        }
    }
}

impl<const p: u64, const D: u64> SubAssign for Fp2<p, D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.c0 -= rhs.c0;
        self.c1 -= rhs.c1;
    }
}

impl<const p: u64, const D: u64> Neg for Fp2<p, D> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            c0: -self.c0,
            c1: -self.c1,
        }
    }
}

impl<const p: u64, const D: u64> Mul for Fp2<p, D> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let d = Fp::<p>::new(D);
        let t0 = self.c0 * rhs.c0;
        let t1 = self.c1 * rhs.c1;
        let c0 = t0 + t1 * d;
        let c1 = self.c0 * rhs.c1 + self.c1 * rhs.c0;
        Self { c0, c1 }
    }
}

impl<const p: u64, const D: u64> MulAssign for Fp2<p, D> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const p: u64, const D: u64> Div for Fp2<p, D> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse()
    }
}

impl<const p: u64, const D: u64> DivAssign for Fp2<p, D> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse();
    }
}

impl<const p: u64, const D: u64> Div<u64> for Fp2<p, D> {
    type Output = Self;
    fn div(self, rhs: u64) -> Self {
        self / Fp2::new(Fp::<p>::new(rhs), Fp::<p>::zero())
    }
}

impl<const p: u64, const D: u64> DivAssign<u64> for Fp2<p, D> {
    fn div_assign(&mut self, rhs: u64) {
        *self /= Fp2::new(Fp::<p>::new(rhs), Fp::<p>::zero());
    }
}

impl<const p: u64, const D: u64> Div<i64> for Fp2<p, D> {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        let r = ((rhs % p as i64) + p as i64) as u64 % p;
        self / Fp2::new(Fp::<p>::new(r), Fp::<p>::zero())
    }
}

impl<const p: u64, const D: u64> DivAssign<i64> for Fp2<p, D> {
    fn div_assign(&mut self, rhs: i64) {
        let r = ((rhs % p as i64) + p as i64) as u64 % p;
        *self /= Fp2::new(Fp::<p>::new(r), Fp::<p>::zero());
    }
}

impl<const p: u64, const D: u64> Mul<u32> for Fp2<p, D> {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self {
        let k = rhs as u64 % p;
        Self {
            c0: Fp::<p>::new((self.c0.value() * k) % p),
            c1: Fp::<p>::new((self.c1.value() * k) % p),
        }
    }
}

impl<const p: u64, const D: u64> MulAssign<u32> for Fp2<p, D> {
    fn mul_assign(&mut self, rhs: u32) {
        let k = rhs as u64 % p;
        self.c0 = Fp::<p>::new((self.c0.value() * k) % p);
        self.c1 = Fp::<p>::new((self.c1.value() * k) % p);
    }
}

impl<const p: u64, const D: u64> core::ops::Mul<Fp2<p, D>> for u32 {
    type Output = Fp2<p, D>;
    fn mul(self, rhs: Fp2<p, D>) -> Fp2<p, D> {
        rhs * self
    }
}

impl<const p: u64, const D: u64> Div<u32> for Fp2<p, D> {
    type Output = Self;
    fn div(self, rhs: u32) -> Self {
        let r = rhs as u64 % p;
        self / Fp2::new(Fp::<p>::new(r), Fp::<p>::zero())
    }
}

impl<const p: u64, const D: u64> DivAssign<u32> for Fp2<p, D> {
    fn div_assign(&mut self, rhs: u32) {
        let r = rhs as u64 % p;
        *self /= Fp2::new(Fp::<p>::new(r), Fp::<p>::zero());
    }
}

impl<const p: u64, const D: u64> Mul<i32> for Fp2<p, D> {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        let k = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        Self {
            c0: Fp::<p>::new((self.c0.value() * k) % p),
            c1: Fp::<p>::new((self.c1.value() * k) % p),
        }
    }
}

impl<const p: u64, const D: u64> MulAssign<i32> for Fp2<p, D> {
    fn mul_assign(&mut self, rhs: i32) {
        let k = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        self.c0 = Fp::<p>::new((self.c0.value() * k) % p);
        self.c1 = Fp::<p>::new((self.c1.value() * k) % p);
    }
}

impl<const p: u64, const D: u64> core::ops::Mul<Fp2<p, D>> for i32 {
    type Output = Fp2<p, D>;
    fn mul(self, rhs: Fp2<p, D>) -> Fp2<p, D> {
        rhs * self
    }
}

impl<const p: u64, const D: u64> Div<i32> for Fp2<p, D> {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        self / Fp2::new(Fp::<p>::new(r), Fp::<p>::zero())
    }
}

impl<const p: u64, const D: u64> DivAssign<i32> for Fp2<p, D> {
    fn div_assign(&mut self, rhs: i32) {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        *self /= Fp2::new(Fp::<p>::new(r), Fp::<p>::zero());
    }
}

/// Quartic extension field over `Fp` constructed as `Fp2[y] / (y^2 - D2)`.
///
/// The base field is `Fp2<p, D1>` and `D2` is an element of this field defined
/// by the two coefficients `D2C0` and `D2C1`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Fp4<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> {
    pub c0: Fp2<p, D1>,
    pub c1: Fp2<p, D1>,
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64>
    Fp4<p, D1, D2C0, D2C1>
{
    /// Create a new element `c0 + c1 * y`.
    pub fn new(c0: Fp2<p, D1>, c1: Fp2<p, D1>) -> Self {
        Self { c0, c1 }
    }

    /// The additive identity element.
    pub fn zero() -> Self {
        Self {
            c0: Fp2::<p, D1>::zero(),
            c1: Fp2::<p, D1>::zero(),
        }
    }

    /// The multiplicative identity element.
    pub fn one() -> Self {
        Self {
            c0: Fp2::<p, D1>::one(),
            c1: Fp2::<p, D1>::zero(),
        }
    }

    /// Exponentiation by `exp` using binary exponentiation.
    pub fn pow(mut self, mut exp: u64) -> Self {
        let mut result = Self::one();
        while exp > 0 {
            if exp % 2 == 1 {
                result *= self;
            }
            self *= self;
            exp /= 2;
        }
        result
    }

    /// Conjugate of `a + by` is `a - by`.
    pub fn conjugate(self) -> Self {
        Self {
            c0: self.c0,
            c1: -self.c1,
        }
    }

    /// Norm of the element: `a^2 - D2 * b^2`.
    pub fn norm(self) -> Fp2<p, D1> {
        let d = Fp2::<p, D1>::new(Fp::<p>::new(D2C0), Fp::<p>::new(D2C1));
        let t0 = self.c0 * self.c0;
        let t1 = self.c1 * self.c1 * d;
        t0 - t1
    }

    /// Square root using the Tonelli-Shanks algorithm.
    /// Returns `None` if no square root exists.
    pub fn sqrt(self) -> Option<Self> {
        if self == Self::zero() {
            return Some(Self::zero());
        }

        // Field size q = p^4
        let q = (p as u128).pow(4);
        let q_minus_one = q - 1;

        // Check if a square exists using Euler's criterion
        if self.pow((q_minus_one / 2) as u64) != Self::one() {
            return None;
        }

        // Factor q-1 = 2^s * t with t odd
        let mut t = q_minus_one;
        let mut s = 0u32;
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let t_u64 = t as u64;

        // Search for a quadratic non-residue z starting with `y` element
        let mut z = Self::new(Fp2::<p, D1>::zero(), Fp2::<p, D1>::one());
        while z.pow((q_minus_one / 2) as u64) == Self::one() {
            z += Self::one();
        }

        let mut c = z.pow(t_u64);
        let mut x = self.pow((t + 1) as u64 / 2);
        let mut b = self.pow(t_u64);
        let mut r = s;

        while b != Self::one() {
            // Find the smallest m (0 < m < r) such that b^(2^m) = 1
            let mut m = 0u32;
            let mut b2m = b;
            while b2m != Self::one() {
                b2m = b2m * b2m;
                m += 1;
                if m == r {
                    return None;
                }
            }

            let gs = c.pow(1u64 << (r - m - 1));
            x *= gs;
            c = gs * gs;
            b *= c;
            r = m;
        }

        Some(x)
    }

    /// Multiplicative inverse using the norm and conjugate.
    /// Panics if `self` is zero.
    pub fn inverse(self) -> Self {
        assert!(self != Self::zero(), "zero has no multiplicative inverse");
        let n = self.norm();
        let inv_n = n.inverse();
        let conj = self.conjugate();
        Self {
            c0: conj.c0 * inv_n,
            c1: conj.c1 * inv_n,
        }
    }

    // 5th root of unity in Fp4.
    pub fn fifth_root_of_unity() -> Self {
        assert!(p % 5 == 2 || p % 5 == 3, "p must be congruent to 2 or 3 mod 5");
        // take a random element in Fp4
        let c0 = Fp2::<p, D1>::new(Fp::<p>::new(D2C0), Fp::<p>::new(D2C1));
        let c1 = Fp2::<p, D1>::new(Fp::<p>::new(D2C0), Fp::<p>::new(D2C1));
        let mut x = Self::new(c0, c1);
        // raise it to the power of (p^4 - 1) / 5
        let exp = (p as u128).pow(4) - 1;
        let exp = exp / 5;

        let mut cnt = 0;
        while x.pow(exp as u64) == Self::one() {
            if cnt % 4 == 0 {
                x.c0.c0 += Fp::<p>::one();
            } else if cnt % 4 == 1 {
                x.c0.c1 += Fp::<p>::one();
            } else if cnt % 4 == 2 {
                x.c1.c0 += Fp::<p>::one();
            } else {
                x.c1.c1 += Fp::<p>::one();
            }
            cnt += 1;
        }
        x = x.pow(exp as u64);
        assert!(x.pow(5) == Self::one(), "5th root of unity is not correct");
        x
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Add
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            c0: self.c0 + rhs.c0,
            c1: self.c1 + rhs.c1,
        }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> AddAssign
    for Fp4<p, D1, D2C0, D2C1>
{
    fn add_assign(&mut self, rhs: Self) {
        self.c0 += rhs.c0;
        self.c1 += rhs.c1;
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Sub
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            c0: self.c0 - rhs.c0,
            c1: self.c1 - rhs.c1,
        }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> SubAssign
    for Fp4<p, D1, D2C0, D2C1>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.c0 -= rhs.c0;
        self.c1 -= rhs.c1;
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Neg
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            c0: -self.c0,
            c1: -self.c1,
        }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Mul
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let d = Fp2::<p, D1>::new(Fp::<p>::new(D2C0), Fp::<p>::new(D2C1));
        let t0 = self.c0 * rhs.c0;
        let t1 = self.c1 * rhs.c1;
        let c0 = t0 + t1 * d;
        let c1 = self.c0 * rhs.c1 + self.c1 * rhs.c0;
        Self { c0, c1 }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> MulAssign
    for Fp4<p, D1, D2C0, D2C1>
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Div
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse()
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> DivAssign
    for Fp4<p, D1, D2C0, D2C1>
{
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse();
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Div<u64>
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn div(self, rhs: u64) -> Self {
        self / Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(rhs), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        )
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> DivAssign<u64>
    for Fp4<p, D1, D2C0, D2C1>
{
    fn div_assign(&mut self, rhs: u64) {
        *self /= Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(rhs), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        );
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Div<i64>
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        let r = ((rhs % p as i64) + p as i64) as u64 % p;
        self / Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(r), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        )
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> DivAssign<i64>
    for Fp4<p, D1, D2C0, D2C1>
{
    fn div_assign(&mut self, rhs: i64) {
        let r = ((rhs % p as i64) + p as i64) as u64 % p;
        *self /= Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(r), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        );
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Mul<u32>
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn mul(self, rhs: u32) -> Self {
        Self {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> MulAssign<u32>
    for Fp4<p, D1, D2C0, D2C1>
{
    fn mul_assign(&mut self, rhs: u32) {
        self.c0 *= rhs;
        self.c1 *= rhs;
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> core::ops::Mul<Fp4<p, D1, D2C0, D2C1>> for u32 {
    type Output = Fp4<p, D1, D2C0, D2C1>;
    fn mul(self, rhs: Fp4<p, D1, D2C0, D2C1>) -> Fp4<p, D1, D2C0, D2C1> {
        rhs * self
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Div<u32>
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn div(self, rhs: u32) -> Self {
        let r = rhs as u64 % p;
        self / Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(r), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        )
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> DivAssign<u32>
    for Fp4<p, D1, D2C0, D2C1>
{
    fn div_assign(&mut self, rhs: u32) {
        let r = rhs as u64 % p;
        *self /= Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(r), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        );
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Mul<i32>
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self {
            c0: self.c0 * rhs,
            c1: self.c1 * rhs,
        }
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> MulAssign<i32>
    for Fp4<p, D1, D2C0, D2C1>
{
    fn mul_assign(&mut self, rhs: i32) {
        self.c0 *= rhs;
        self.c1 *= rhs;
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> core::ops::Mul<Fp4<p, D1, D2C0, D2C1>> for i32 {
    type Output = Fp4<p, D1, D2C0, D2C1>;
    fn mul(self, rhs: Fp4<p, D1, D2C0, D2C1>) -> Fp4<p, D1, D2C0, D2C1> {
        rhs * self
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> Div<i32>
    for Fp4<p, D1, D2C0, D2C1>
{
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        self / Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(r), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        )
    }
}

impl<const p: u64, const D1: u64, const D2C0: u64, const D2C1: u64> DivAssign<i32>
    for Fp4<p, D1, D2C0, D2C1>
{
    fn div_assign(&mut self, rhs: i32) {
        let r = ((rhs as i64 % p as i64) + p as i64) as u64 % p;
        *self /= Fp4::new(
            Fp2::<p, D1>::new(Fp::<p>::new(r), Fp::<p>::zero()),
            Fp2::<p, D1>::zero(),
        );
    }
}

