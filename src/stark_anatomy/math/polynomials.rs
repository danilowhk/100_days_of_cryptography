use super::field_element::FieldElement;
use core;
use itertools::{enumerate, EitherOrBoth, Itertools};
use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Polynomial(Vec<FieldElement>);

impl Polynomial {
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        Polynomial(coefficients)
    }

    pub fn coefs(&self) -> &[FieldElement] {
        &self.0
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

    //TODO: Study more and understand polynomial division algorithms
    // Current code from lambdaclass/STARK101-rs
    pub fn qdiv(self, other: Polynomial) -> (Polynomial, Polynomial) {
        let other_elems = other.0;
        let self_elems = self.0;

        assert!(other_elems.len() != 0, "Dividing by zero polynomial.");

        if self_elems.is_empty() {
            return (Polynomial(vec![]), Polynomial(vec![]));
        }

        // Initialize remainder as a copy of self
        let mut rem = self_elems.clone();

        // Compute the difference between the degrees of self and other
        let mut degree_difference = rem.len() as usize - other_elems.len() as usize;

        // Initialize quotient with zeros
        let mut quotient: Vec<FieldElement> = if degree_difference > 0 {
            vec![FieldElement::zero(); degree_difference + 1]
        } else {
            vec![FieldElement::zero()]
        };

        // Perform long division until the remainder is smaller than "other"
        while degree_difference >= 0 {
            // Compute the next coefficient of the quotient
            let tmp = rem.last().unwrap().to_owned() * other_elems.last().unwrap().inverse();
            quotient[degree_difference] = quotient[degree_difference] + tmp;

            // Update the remainder
            let mut last_non_zero = degree_difference as isize - 1;
            for (i, coef) in enumerate(other_elems.clone()) {
                let k = i + degree_difference as usize;
                rem[k] = rem[k] - (tmp * coef);
                if rem[k] != FieldElement::zero() {
                    last_non_zero = k as isize;
                }
            }
            //TODO: Understand why we need to eliminate trailing zeros
            // Eliminate trailing zeroes from the remainder
            rem = rem.into_iter().take((last_non_zero + 1) as usize).collect();

            // Update the degree difference
            degree_difference = rem.len() as usize - other_elems.len() as usize;
        }

        return (Polynomial(quotient), Polynomial(rem));
    }

    pub fn module(self, other: Polynomial) -> Polynomial {
        let (q, r) = self.clone().qdiv(other.clone());
        r
    }

    pub fn pow(&self, other: usize) -> Self {
        // Initialize variables
        let mut other = other;
        let mut res = Polynomial(vec![FieldElement::one()]);
        let mut current = self.to_owned();

        // Loop while other is not zero
        loop {
            // If the current bit of other is 1, multiply result by current
            if other % 2 != 0 {
                res = res * current.to_owned();
            }

            if other == 0 {
                break;
            }

            // Divide other by 2
            other >>= 1;

            // Square the current polynomial
            current = current.to_owned() * current;
        }
        // Return the result
        res
    }
    // Evaluate the polynomial at the given point using naive evaluation(not so fast).
    pub fn evaluate(&self, point: FieldElement) -> FieldElement {
        // Initialize xi to be 1 and value to be 0.
        let mut xi = FieldElement::one();
        let mut value = FieldElement::zero();

        // Iterate over each coefficient in the polynomial.
        for coef in self.clone().0 {
            // Evaluate the value of the polynomial at the given point.
            value = value + (coef * xi);
            // Update xi for the next coefficient multiplicantion. (x , x^2, x^3, ...)
            xi = xi * point;
        }

        // Return the evaluated value of the polynomial at the given point.
        value
    }

    // Evaluates the polynomial at the given point using Horner evaluation(Very very interesting!).
    // From lambdaclass/STARK101-rs
    pub fn eval(&self, point: impl Into<FieldElement>) -> FieldElement {
        // Conver point into FieldElement type.
        let point: FieldElement = point.into();
        // Initialize val to be 0.
        let mut val = FieldElement::zero();

        // Iterate over each coefficient in the polynomial in reverse order.
        for coef in self.0.clone().into_iter().rev() {
            // Update val by multiplying it with point and adding the current coefficient.
            val = val * point + coef;
        }

        // Return the evaluated value of the polynomial at the given point.
        val
    }

    // Evaluate the polynomial at the given domain using naive evaluation for each point in the domain.
    pub fn evaluate_domain(&self, domain: &[FieldElement]) -> Vec<FieldElement> {
        // Iterate through domain and run "evaluate function" for each point in domain.
        domain.iter().map(|&d| self.evaluate(d)).collect()
    }

    // Given a set of `domain` and corresponding `values`, returns a polynomial of domain.len() - 1
    // that evaluates to `values[i]` on `domain[i]` for all i.
    pub fn interpolate_domain(domain: &[FieldElement], values: &[FieldElement]) -> Polynomial {
        // Ensure that the lenght of the `domain` and `values` are the same.
        assert_eq!(
            domain.len(),
            values.len(),
            "number of elements in domain does not match number of values -- cannot interpolate"
        );

        //Ensure that the length of the `domain` is at least 1.
        assert!(!domain.is_empty(), "cannot interpolate between zero points");

        // Create the polynomial x = 1 * x^1 + 0 * x^0
        let x = Polynomial(vec![FieldElement::zero(), FieldElement::one()]);

        // Create an empty polynomial as accumulator.
        let mut acc = Polynomial(vec![]);

        //For each element in `domain`, construct a Lagrange polynomial.
        for i in 0..domain.len() {
            // Create the polynomial `prod = values[i] * (x - domain[j]) * ((domain[i] - domain[j])^-1)`.
            let mut prod = Polynomial(vec![values[i]]);
            for j in 0..domain.len() {
                if i == j {
                    continue;
                }
                let diff = domain[i] - domain[j];
                prod = prod
                    * (x.clone() - Polynomial(vec![domain[j]]))
                    * Polynomial(vec![diff.inverse()]);
            }
            // Add `prod` to the accumulator polynomial L(x) = y1 L1(x) + y2 L2(x) + ... + yn Ln(x)
            acc = acc + prod;
        }
        acc
    }

    // Given a vector of `domain` values, returns the zero polynomial with roots at each value in `domain`.
    fn zerofier_domain(domain: &[FieldElement]) -> Polynomial {
        // Create the polynomial x = x^1 + 0 * x^0
        let x = Polynomial(vec![FieldElement::zero(), FieldElement::one()]);
        // Create the value 1
        let mut acc = Polynomial(vec![FieldElement::one()]);
        for d in domain {
            // Create the polynomial d and multiply it with the accumulator polynomial , multiply `acc` by `(x - d)`.
            acc = acc * (x.clone() - Polynomial(vec![*d]));
        }
        acc
    }

    // Scales the polynomial by the given `factor`.
    // Specifically, obtains the coefficients of `f(c*x)` from the coefficients of `f(X)`.
    // For example: f(X) = 1 + 2X + 3X^2, f(2X) = 1 + 2*2X + 4*3X^2
    // This function is useful when `f(X)` is defined to take a sequence of values on the powers of `c`.
    pub fn scale(&self, factor: &FieldElement) -> Self {
        let scaled_coeffs: Vec<FieldElement> = self
            .clone()
            .0
            .iter()
            .enumerate()
            .map(|(i, coeff)| factor.pow(i as usize) * coeff)
            .collect();
        Polynomial(scaled_coeffs)
    }

    // Given a vector of `(x, y)` points, tests whetter they lie on the same line.
    pub fn test_colinearity(points: &[(FieldElement, FieldElement)]) -> bool {
        // Get the `x` values from the points.
        let domain: Vec<FieldElement> = points.iter().map(|(x, _)| *x).collect();
        // Get the `y` values from the points.
        let values: Vec<FieldElement> = points.iter().map(|(_, y)| *y).collect();
        // Interpolate a polynomial from the points.
        let polynomial = Self::interpolate_domain(&domain, &values);
        // Check if polynomial are colinear or not. (If they are in the same line or not)
        polynomial.degree() <= 1
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

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        // Check if either inputs are empty
        if self.clone().0.is_empty() || rhs.clone().0.is_empty() {
            return Polynomial::new(vec![]);
        }
        // Initialize some variables
        // Set `zero` to the additive identity of the field
        let zero = FieldElement::zero();
        // Calculate the length of the product polynomial
        let len = self.clone().0.len() + rhs.clone().0.len() - 1;
        // Create a buffer array to store the coefficients of the product polynomial
        let mut buff = vec![zero; len];
        // Multiply the polynomials
        for i in 0..self.clone().0.len() {
            if self.clone().is_zero() {
                continue;
            }

            for j in 0..rhs.clone().0.len() {
                buff[i + j] += self.clone().0[i] * rhs.clone().0[j]
            }
        }

        Polynomial::new(buff)
    }
}

impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: Self) -> Self::Output {
        let (div, rem) = self.qdiv(rhs);
        assert!(rem.0.is_empty(), "Polynomials are not divisible.");
        div
    }
}

// impl ops::BitXor<usize> for Polynomial {
//     type Output = Self;

//     fn bitxor(self, exponent: usize) -> Self::Output {
//         if self.is_zero() {
//             return Polynomial(vec![]);
//         }
//         if exponent == 0 {
//             return Polynomial(vec![self.coefficients[0].field.one()]);
//         }
//         let mut acc = Polynomial(vec![self.coefficients[0].field.one()]);
//         for i in (0..exponent.trailing_zeros().wrapping_neg() as usize).rev() {
//             acc = acc.clone() * acc.clone();
//             if (1 << i) & exponent != 0 {
//                 acc = acc.clone() * self.clone();
//             }
//         }
//         acc
//     }
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
