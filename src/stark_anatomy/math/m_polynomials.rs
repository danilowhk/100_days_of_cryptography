use super::field_element::FieldElement;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MPolynomial (HashMap<Vec<usize>, FieldElement>);


