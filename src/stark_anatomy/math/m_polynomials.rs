use super::{field_element::FieldElement, polynomials::Polynomial};
use std::{
    collections::HashMap,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Debug)]
pub struct MPolynomial(HashMap<Vec<usize>, FieldElement>);

impl MPolynomial {
    // Create a new polynomial from a dictionary
    pub fn new(dictionary: HashMap<Vec<usize>, FieldElement>) -> Self {
        MPolynomial(dictionary)
    }

    // Create a new zero polynomial
    pub fn zero() -> Self {
        MPolynomial(HashMap::new())
    }

    // Create a new constant polynomial with the given element as its coefficient
    pub fn constant(element: FieldElement) -> Self {
        let mut dictionary = HashMap::new();
        dictionary.insert(vec![0], element);
        MPolynomial(dictionary)
    }

    // Check if the m_polynomial is zero
    pub fn is_zero(&self) -> bool {
        if self.0.is_empty() || self.0.values().all(|v| v.is_zero()) {
            return true;
        };
        false
    }

    // Crete a vector of variables, one for each variable in the polynomial
    pub fn variables(num_variables: usize, field: &FieldElement) -> Vec<Self> {
        let mut variables = Vec::new();
        for i in 0..num_variables {
            let mut exponent = vec![0, num_variables];
            exponent[i] = 1;
            variables.push(MPolynomial(
                [(exponent, FieldElement::one())].iter().cloned().collect(),
            ));
        }
        variables
    }

    // Calculate exponantiations of the polynomial using binary mechanism
    pub fn pow(&self, exponent: usize) -> Self {
        //If self is zero,return zero
        if self.is_zero() {
            return MPolynomial::zero();
        }

        // Get the number of variables from the length of the first exponent in self
        let num_variables = self.0.keys().next().unwrap().len();
        // Initialzie the exponent to be all zeroes and create the accumulator polynomial with a constant term of 1
        let mut exp = vec![0 as usize; num_variables];
        let mut dictionary = HashMap::new();
        dictionary.insert(exp, FieldElement::one());
        let mut acc = MPolynomial(dictionary);
        // Convert the exponent to binary and iterate over the bits from right to left, skiping the first two characters '0b'
        for b in format!("{:b}", exponent).chars().skip(2) {
            // Square the accumulator polynomial
            acc = acc.clone() * acc.clone();
            // If the current bit is 1, multiply the accumulator polynomial by self
            if b == '1' {
                acc = acc * self.clone();
            }
        }
        //return acc
        acc
    }

    // TODO: Fix errors
    pub fn lift(polynomial: Polynomial, variable_index: usize) -> MPolynomial {
        // If polynomial is zero, return a zero MPolynomial
        if polynomial.clone().is_zero() {
            return MPolynomial::zero();
        }
        // Create an MPolynomial with the variables up to variable_index + 1
        let mut variables = Vec::with_capacity(variable_index + 1);
        for i in 0..=variable_index {
            variables.push(MPolynomial::variables(i, &FieldElement::zero()));
        }

        //Get the variable x from the end of the variables vector
        let x = &variables.clone()[variables.clone().len()];

        // Initialize an accumulator polynomial with all zero coefficients
        let mut acc = MPolynomial::zero();

        // Iterate over the coeeficients of the input polynomial
        for i in 0..polynomial.clone().coefs().len() {
            let constant = MPolynomial::constant(polynomial.clone().coefs()[i].clone());

            // Multiply the constant MPolynomial by x^i and add it to the accumulator polynomial
            acc = acc + (constant * x.clone()[0].pow(i));
        }

        acc
    }

    //TODO: Check evaluate function correctness
    pub fn evaluate(self, point: &[FieldElement]) -> FieldElement {
        // Initialize an accumulator FieldElent 0
        let mut acc = FieldElement::zero();
        for (k, v) in self.0 {
            // Initialize a product FieldElement
            let mut prod = v;

            for i in 0..k.len() {
                prod = prod * (point[i].pow(k[i]));
            }

            acc = acc + prod;
        }
        acc
    }

    //TODO: Check point input type
    pub fn evaluate_symbolic(self, point: &[Polynomial]) -> Polynomial {
        // Initialize an accumulator Polynomial with a constant term of 0
        let mut acc = Polynomial::new(vec![FieldElement::zero()]);

        // Iterate over the exponents and coefficients of the MPolynomial
        for (k, v) in self.0 {
            let mut prod = Polynomial::new(vec![v]);
            // Iterate over k
            for i in 0..k.len() {
                prod = prod * (point[i].pow(k[i]));
            }
            acc = acc + prod;
        }
        //return polynomial
        acc
    }
}

// Implementing Add trait for MPolynomial
impl Add for MPolynomial {
    type Output = MPolynomial;

    // Implementing add function for MPolynomial
    fn add(self, rhs: MPolynomial) -> Self::Output {
        // Creating an empty dictionary / HashMap
        let mut dictionary = HashMap::new();
        // Finding the maximum length of all keys in both dictionaries
        let num_variables = self
            .0
            .keys()
            .map(|k| k.len())
            .chain(rhs.0.keys().map(|k| k.len()))
            .max()
            .unwrap_or(0);

        // Iterate over each element in self dictionary/HashMap to pad the keys with 0's and add to dictionary/HashMap
        for (k, v) in self.0.into_iter() {
            // Creating a new vector by appending 0's to k such that it's length is equal to num_variables
            let mut pad = k.clone();
            pad.resize(num_variables, 0);
            // Inserting the padded vector and it's value v into the dictionary
            dictionary.insert(pad, v);
        }

        // Iterate over each element in rhs dictionary/HashMap to pad the keys with 0's and add to dictionary/HashMap
        for (k, v) in rhs.0.into_iter() {
            // Creating a new vector by appending 0's to k such that it's length is equal to num_variables
            let mut pad = k.clone();
            pad.resize(num_variables, 0);
            // Checking if the padded vector exists in the dictionary. If it does, adding v to it's existing value.
            if let Some(value) = dictionary.get_mut(&pad) {
                *value = *value + v;
            } else {
                dictionary.insert(pad, v);
            }
        }

        MPolynomial(dictionary)
    }
}

// Implementing Sub trait for MPolynomial
impl Sub for MPolynomial {
    type Output = MPolynomial;

    // Implementing sub function for MPolynomial
    fn sub(self, rhs: MPolynomial) -> Self::Output {
        // Subtracting rhs from the self Polynomial by adding self and -rhs
        self + (-rhs)
    }
}

// Implementing Neg trait for MPolynomial
impl Neg for MPolynomial {
    type Output = MPolynomial;
    // Creating a new HashMap with values of opposite sign of self's HashMap values
    fn neg(self) -> MPolynomial {
        let dictionary = self.0.into_iter().map(|(k, v)| (k, -v)).collect();
        MPolynomial(dictionary)
    }
}

impl Mul for MPolynomial {
    type Output = MPolynomial;

    // Define the multiplication of two polynomials
    fn mul(self, rhs: MPolynomial) -> Self::Output {
        // Initialize an empty HashMap to store the coefficients of the product polynomial
        let mut dictionary = HashMap::new();
        // Compute the maximum number of variables from the lengths of the exponents in both polynomials
        let num_variables = self.0.keys().map(|k| k.len()).max().unwrap_or(0)
            + rhs.0.keys().map(|k| k.len()).max().unwrap_or(0);
        // Iterate over the terms in both polynomials and compute the exponents and coefficients of the product polynomial
        for (k0, v0) in self.0 {
            for (k1, v1) in &rhs.0 {
                // Initialize the exponent to be all zeros with sie num_variables
                let mut exponent = vec![0; num_variables];
                // Add the exponents of the current terms to compute the exponent of the product term
                for k in 0..k0.len() {
                    exponent[k] += k0[k];
                }
                for k in 0..k1.len() {
                    exponent[k + k0.len()] += k1[k];
                }
                // Pad the exponent with zeros to the maximum number of variables and conver it to a tuple
                let pad = exponent
                    .iter()
                    .copied()
                    .map(Some)
                    .chain(std::iter::repeat(None))
                    .take(num_variables)
                    .collect::<Vec<_>>();
                let pad = pad.into_iter().map(|e| e.unwrap_or(0)).collect::<Vec<_>>();
                // Turn into a boxed slice of usize
                let pad = pad.into_boxed_slice();

                let exponent = pad.into();
                //Add the product of the coefficients to the dictionary
                if let Some(value) = dictionary.get_mut(&exponent) {
                    *value = *value + v0.clone() * v1.clone();
                } else {
                    dictionary.insert(exponent, v0.clone() * v1.clone());
                }
            }
        }
        MPolynomial::new(dictionary)
    }
}
