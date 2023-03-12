use super::field_element::FieldElement;
use std::{collections::HashMap, ops::{Add, Neg, Sub}};

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
    fn sub(self, rhs: MPolynomial) -> Self::Output{
        // Subtracting rhs from the self Polynomial by adding self and -rhs
        self + (-rhs)
    }
}

// Implementing Neg trait for MPolynomial
impl Neg for MPolynomial {
    type Output = MPolynomial;
    // Creating a new HashMap with values of opposite sign of self's HashMap values
    fn neg(self) -> MPolynomial {
        let dictionary = self.0.into_iter().map(|(k,v)| (k, -v)).collect();
        MPolynomial(dictionary)
    }
}


// impl Mul for MPolynomial {
//     type Output = MPolynomial;

// }

