use crate::field::crandall_field::{reduce128, CrandallField};
use crate::field::field::Field;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait QuarticFieldExtension: Field {
    type BaseField: Field;

    // Element W of BaseField, such that `X^4 - W` is irreducible over BaseField.
    const W: Self::BaseField;

    fn to_canonical_representation(&self) -> [Self::BaseField; 4];

    fn is_in_basefield(&self) -> bool {
        self.to_canonical_representation()[1..]
            .iter()
            .all(|x| x.is_zero())
    }

    /// Frobenius automorphisms: x -> x^p, where p is the order of BaseField.
    fn frobenius(&self) -> Self;

    fn scalar_mul(&self, c: Self::BaseField) -> Self;
}

#[derive(Copy, Clone)]
pub struct QuarticCrandallField([CrandallField; 4]);

impl QuarticFieldExtension for QuarticCrandallField {
    type BaseField = CrandallField;
    // Verifiable in Sage with
    // ``R.<x> = GF(p)[]; assert (x^4 -3).is_irreducible()`.
    const W: Self::BaseField = CrandallField(3);

    fn to_canonical_representation(&self) -> [Self::BaseField; 4] {
        self.0
    }

    fn frobenius(&self) -> Self {
        let [a0, a1, a2, a3] = self.to_canonical_representation();
        let k = (Self::BaseField::ORDER - 1) / 4;
        let z0 = Self::W.exp_usize(k as usize);
        let mut z = Self::BaseField::ONE;
        let b0 = a0 * z;
        z *= z0;
        let b1 = a1 * z;
        z *= z0;
        let b2 = a2 * z;
        z *= z0;
        let b3 = a3 * z;

        Self([b0, b1, b2, b3])
    }

    fn scalar_mul(&self, c: Self::BaseField) -> Self {
        let [a0, a1, a2, a3] = self.to_canonical_representation();
        Self([a0 * c, a1 * c, a2 * c, a3 * c])
    }
}

impl PartialEq for QuarticCrandallField {
    fn eq(&self, other: &Self) -> bool {
        self.to_canonical_representation() == other.to_canonical_representation()
    }
}

impl Eq for QuarticCrandallField {}

impl Hash for QuarticCrandallField {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for l in &self.to_canonical_representation() {
            Hash::hash(l, state);
        }
    }
}

impl Field for QuarticCrandallField {
    const ZERO: Self = Self([CrandallField::ZERO; 4]);
    const ONE: Self = Self([
        CrandallField::ONE,
        CrandallField::ZERO,
        CrandallField::ZERO,
        CrandallField::ZERO,
    ]);
    const TWO: Self = Self([
        CrandallField::TWO,
        CrandallField::ZERO,
        CrandallField::ZERO,
        CrandallField::ZERO,
    ]);
    const NEG_ONE: Self = Self([
        CrandallField::NEG_ONE,
        CrandallField::ZERO,
        CrandallField::ZERO,
        CrandallField::ZERO,
    ]);

    // Does not fit in 64-bits.
    const ORDER: u64 = 0;
    const TWO_ADICITY: usize = 30;
    const MULTIPLICATIVE_GROUP_GENERATOR: Self = Self([
        CrandallField(3),
        CrandallField::ONE,
        CrandallField::ZERO,
        CrandallField::ZERO,
    ]);
    const POWER_OF_TWO_GENERATOR: Self = Self([
        CrandallField::ZERO,
        CrandallField::ZERO,
        CrandallField::ZERO,
        CrandallField(14096607364803438105),
    ]);

    // Algorithm 11.3.4 in Handbook of Elliptic and Hyperelliptic Curve Cryptography.
    fn try_inverse(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }

        let a_pow_p = self.frobenius();
        let a_pow_p_plus_1 = a_pow_p * *self;
        let a_pow_p3_plus_p2 = a_pow_p_plus_1.frobenius().frobenius();
        let a_pow_r_minus_1 = a_pow_p3_plus_p2 * a_pow_p;
        let a_pow_r = a_pow_r_minus_1 * *self;
        debug_assert!(a_pow_r.is_in_basefield());

        Some(a_pow_r_minus_1.scalar_mul(a_pow_r.0[0].inverse()))
    }

    fn to_canonical_u64(&self) -> u64 {
        todo!()
    }

    fn from_canonical_u64(n: u64) -> Self {
        todo!()
    }
}

impl Display for QuarticCrandallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {}*a + {}*a^2 + {}*a^3",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl Debug for QuarticCrandallField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Neg for QuarticCrandallField {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}

impl Add for QuarticCrandallField {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl AddAssign for QuarticCrandallField {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for QuarticCrandallField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl Sub for QuarticCrandallField {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl SubAssign for QuarticCrandallField {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for QuarticCrandallField {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let Self([a0, a1, a2, a3]) = self;
        let Self([b0, b1, b2, b3]) = rhs;
        let a0 = a0.0 as u128;
        let a1 = a1.0 as u128;
        let a2 = a2.0 as u128;
        let a3 = a3.0 as u128;
        let b0 = b0.0 as u128;
        let b1 = b1.0 as u128;
        let b2 = b2.0 as u128;
        let b3 = b3.0 as u128;
        let w = Self::W.0 as u128;

        let c0 = reduce128(a0 * b0 + w * (a1 * b3 + a2 * b2 + a3 * b1));
        let c1 = reduce128(a0 * b1 + a1 * b0 + w * (a2 * b3 + a3 * b2));
        let c2 = reduce128(a0 * b2 + a1 * b1 + a2 * b0 + w * a3 * b3);
        let c3 = reduce128(a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0);

        Self([c0, c1, c2, c3])
    }
}

impl MulAssign for QuarticCrandallField {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Product for QuarticCrandallField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl Div for QuarticCrandallField {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl DivAssign for QuarticCrandallField {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::field::crandall_field::CrandallField;
    use crate::field::extension_field::{QuarticCrandallField, QuarticFieldExtension};
    use crate::field::field::Field;
    use crate::test_arithmetic;

    test_arithmetic!(crate::field::crandall_field::QuarticCrandallField);

    #[test]
    fn test_frobenius() {
        let x = QuarticCrandallField::rand();
        assert_eq!(x.exp_usize(CrandallField::ORDER as usize), x.frobenius());
    }
}
