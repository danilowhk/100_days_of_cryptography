use super::field_element::FieldElement;
use core;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Polynomial(Vec<FieldElement>);

impl Polynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        Polynomial(coefficients)
    }

    pub fn degree(self) -> i32 {
        // Check if its empty polynomial
        if self.0.is_empty() {
            return -1;
        }

        let zero = FieldElement::zero();
        // Check if all elements == 0
        if self.0.iter().all(|&c| c == zero) {
            return -1;
        }
        let mut maxindex = 0;
        for i in (0..self.0.len()).rev() {
            if self.0[i] != zero {
                // Store the biggest non-zero index
                maxindex = i;
            }
        }
        maxindex as i32
    }

    pub fn is_zero(self) -> bool {
        if self.clone().degree() == -1 {
            return false;
        }
        true
    }

    pub fn get_nth_degree_coefficient(self, n: usize) -> FieldElement {
        if n > self.clone().degree() as _ {
            FieldElement::zero()
        } else {
            self.0[n]
        }
    }

    pub fn leading_coefficient(self) -> FieldElement {
        self.clone()
            .get_nth_degree_coefficient(self.clone().degree() as usize)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Polynomial) -> Self::Output {
        self + (-rhs)
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Polynomial) -> Self::Output {
        if self.clone().degree() == -1 {
            return rhs.clone();
        } else if rhs.clone().degree() == -1 {
            return self.clone();
        }
        let len = core::cmp::max(self.clone().0.len(), rhs.clone().0.len());
        let mut coeffs = vec![FieldElement::zero(); len];
        for i in 0..len {
            let value_1 = self.clone().0[i];
            let value_2 = rhs.clone().0[i];
            coeffs[i] = value_1 + value_2;
        }
        Polynomial(coeffs)
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Self::Output {
        Polynomial::new(self.0.iter().map(|&c| -c).collect())
    }
}

// impl Mul for Polynomial {
//     type Output = Polynomial;

// }

// pub fn mul(&self, other: &Polynomial) -> Polynomial {
//     if self.coefficients.is_empty() || other.coefficients.is_empty() {
//         return Polynomial::new(vec![]);
//     }
//     let zero = self.coefficients[0].field().zero();
//     let len = self.coefficients.len() + other.coefficients.len() - 1;
//     let mut buf = vec![zero; len];
//     for i in 0..self.coefficients.len() {
//         if self.coefficients[i].is_zero() {
//             continue; // optimization for sparse polynomials
//         }
//         for j in 0..other.coefficients.len() {
//             buf[i+j] += self.coefficients[i] * other.coefficients[j];
//         }
//     }
//     Polynomial::new(buf)
// }

// impl From<usize> for Polynomial {
//     fn from(value: usize) -> Self {
//         let fe: FieldElement = value.into();
//         fe.into()
//     }
// }

// impl From<FieldElement> for Polynomial {
//     fn from(value: FieldElement) -> Self {
//         Polynomial::new(&[value])
//     }
// }

// impl From<FieldElement> for Polynomial {
//     fn from(value: FieldElement) -> Self {
//         Polynomial::new(&[value])
//     }
// }
