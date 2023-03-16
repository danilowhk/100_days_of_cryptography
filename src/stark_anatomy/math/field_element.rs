use core::fmt;
use rand::Rng;
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

use super::{constants::MODULUS, utils::xgcd};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FieldElement(usize);

impl FieldElement {
    pub fn new(value: usize) -> Self {
        FieldElement(value % MODULUS)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn inverse(&self) -> Self {
        let (t, r, s) = xgcd(MODULUS as i128, self.0 as i128);
        t
    }

    pub fn zero() -> Self {
        FieldElement::new(0)
    }

    pub fn one() -> Self {
        FieldElement::new(1)
    }

    pub fn pow(&self, n: usize) -> Self {
        let mut n = n;
        let mut current_pow = self.to_owned();
        let mut res = FieldElement::one();
        while n > 0 {
            if n % 2 != 0 {
                res *= current_pow;
            }
            n = n / 2;
            current_pow *= current_pow;
        }
        res
    }

    pub fn is_order(&self, n: usize) -> bool {
        assert!(n >= 1);
        let mut h = FieldElement(1);
        for _ in 1..n {
            h *= self;
            if h == FieldElement::one() {
                return false;
            }
        }
        h * self == FieldElement::one()
    }

    pub fn generator() -> Self {
        FieldElement(5)
    }

    pub fn sample(byte_array: &[u8]) -> FieldElement {
        let mut acc = 0;
        for b in byte_array {
            acc = (acc << 8) ^ u128::from(*b);
        }
        let residue = acc % MODULUS as u128;
        return FieldElement(residue as usize);
    }

    /// Generates a random FieldElement.
    pub fn random_element(excluded_elements: &[FieldElement]) -> Self {
        let mut rng = rand::thread_rng();
        let mut fe = FieldElement::new(rng.gen::<usize>());
        while excluded_elements.contains(&fe) {
            fe = FieldElement::new(rng.gen::<usize>());
        }
        fe
    }

    pub fn primitive_nth_root_of_unity(n: usize) -> Self {
        todo!()
        // FieldElement::generator().pow((MODULUS - 1) / n)
    }

    // fn primitive_nth_root(&self, n: u128) -> FieldElement {
    //     assert!(n <= 1u128 << 119 && (n & (n-1)) == 0, "Field does not have nth root of unity where n > 2^119 or not power of two.");
    //     let mut root = FieldElement(85408008396924667383611388730472331217);
    //     let mut order = 1u128 << 119;
    //     while order != n {
    //         root = root*root;
    //         order /= 2;
    //     }
    //     return root;

    // }

    pub fn to_bytes(&self) -> [u8; 8] {
        let bytes = self.0.to_le_bytes(); // Convert usize to little-endian byte array
        bytes.try_into().expect("usize should be exactly 8 bytes")
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new(self.0 + rhs.0)
    }
}

impl std::ops::Add for &FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        FieldElement::new(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for FieldElement {
    fn add_assign(&mut self, rhs: Self) {
        *self = FieldElement::new(self.0 + rhs.0)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new(self.0 * rhs.0)
    }
}

impl Mul<&FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: &FieldElement) -> Self::Output {
        FieldElement::new(self.0 * rhs.0)
    }
}

impl MulAssign for FieldElement {
    fn mul_assign(&mut self, rhs: Self) {
        *self = FieldElement::new(self.0 * rhs.0)
    }
}

impl MulAssign<&FieldElement> for FieldElement {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = FieldElement::new(self.0 * rhs.0)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new(self.0 - rhs.0)
    }
}

impl Sub<&FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: &FieldElement) -> Self::Output {
        FieldElement::new(self.0 - rhs.0)
    }
}

impl Div<usize> for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: usize) -> Self::Output {
        self * FieldElement::new(rhs).inverse()
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        FieldElement::zero() - self
    }
}

impl From<usize> for FieldElement {
    fn from(value: usize) -> Self {
        FieldElement::new(value)
    }
}

impl From<i128> for FieldElement {
    fn from(value: i128) -> Self {
        let value_mod_p = if value > 0 {
            value % (MODULUS as i128)
        } else {
            value + MODULUS as i128
        };
        FieldElement::new(value_mod_p.try_into().unwrap())
    }
}

impl From<FieldElement> for usize {
    fn from(value: FieldElement) -> Self {
        value.0
    }
}

#[test]
fn inverse_test() {
    let x = FieldElement::new(2);
    let x_inv = x.inverse();

    assert_eq!(FieldElement::one(), x * x_inv)
}

#[test]
fn test_field_wrap() {
    let t = FieldElement(2).pow(30) * FieldElement(3) + FieldElement::one();
    assert!(t == FieldElement::zero())
}

#[test]
fn test_field_div() {
    for _ in 1..10000 {
        let t = FieldElement::random_element(&[FieldElement::zero()]);
        let t_inv = FieldElement::one() / t;
        assert!(t_inv == t.inverse());
        assert!(t_inv * t == FieldElement::one());
    }
}
