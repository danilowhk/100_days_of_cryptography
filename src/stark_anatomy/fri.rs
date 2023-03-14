use super::{math::field_element::FieldElement, proof_stream::ProofStream};

// Define a new struct to represent the FRI protocol
struct Fri {
    pub offset: FieldElement,
    pub omega: FieldElement,
    domain_length: usize,
    field: FieldElement,
    expansion_factor: usize,
    num_colinearity_tests: usize,
}

impl Fri {
    // Define a new method to calculate the number of rounds needed for the FRI protocol
    fn num_rounds(&self) -> usize {
        // Get the domain length of the FRI protocol
        let mut codeword_length = self.domain_length;
        // Initialize the number of rounds to 0
        let mut num_rounds = 0;
        // Calculate the number of rounds based on the codeword length, expansion factor, and number of colinearity tests
        while codeword_length > self.expansion_factor
            && 4 * self.num_colinearity_tests < codeword_length
        {
            codeword_length /= 2;
            num_rounds += 1;
        }
        num_rounds
    }
    // Define a new method to evaluate the domain of the FRI protocol
    fn eval_domain(&self) -> Vec<FieldElement> {
        let mut domain = vec![];
        // Generate a list of field elements by raising omega to the powers of i
        for i in 0..self.domain_length {
            domain.push(self.offset * self.omega.pow(i));
        }
        domain
    }

    // The `prove` function takes in a `codeword` and a `proof_stream` object, and returns a list of indices used in the proof.
    fn prove(&self, codeword:&[FieldElement], proof_stream: &mut ProofStream) -> Vec<usize> {
        // Ensure that the lenght of the initial codeword matches the length of the domain.
        assert!(self.domain_length == codeword.len(), "initial codeword length does not match length of initial codeword");

        // Commit phase: Compute and send Merkle root for the initial codeword.
        // let codewords = self.commit(codeword, proof_stream);

        // // Sample indices for the top level and collect them into a list.
        // let top_level_indices = self.sample_indices(proof_stream.prover_fiat_shamir(), codewords[1].len(), codewords[codewords.len() - 1].len(), self.num_colinearity_tests);
        // let mut indices = top_level_indices.iter().clones().collect::<Vec<usize>>;

        // Query phase: For each round, fold the indices and query the codeword.
        // for i in 0..codewords.len() - 1 {
        //     indices = indices.iter().map(|index| index % (codewords[i].len() / 2)).collect::<Vec<usize>>();
        //     self.query(&codewords[i], &codewords[i + 1], &indices, proof_stream);
        // }

        // Return the top-level indices used in the proof.
        // top_level_indices

        todo!()
    }

// // Define a method to commit to the codeword
// fn commit(&self, mut codeword: Vec<FieldElement>, proof_stream: &mut ProofStream) -> Vec<Vec<FieldElement>> {
//     let one = FieldElement::one(&self.field);
//     let two = FieldElement::from(2, &self.field);
//     let mut omega = self.omega;
//     let mut offset = self.offset;
//     let mut codewords = Vec::new();

//     // For each round
//     for r in 0..self.num_rounds() {
//         // Compute and send Merkle root
//         let root = Merkle::commit(&codeword);
//         proof_stream.push(root);

//         // Prepare next round, if necessary
//         if r == self.num_rounds() - 1 {
//             break;
//         }

//         // Get challenge
//         let alpha = self.field.sample(proof_stream.prover_fiat_shamir());

//         // Collect codeword
//         codewords.push(codeword.clone());

//         // Split and fold
//         let half_length = codeword.len() / 2;
//         codeword = (0..half_length)
//             .map(|i| {
//                 let alpha_i = alpha / (offset * omega.pow(i as u32));
//                 let c0 = (one + alpha_i) * &codeword[i];
//                 let c1 = (one - alpha_i) * &codeword[half_length + i];
//                 two.inverse() * &(c0 + c1)
//             })
//             .collect();
//         omega = omega.square();
//         offset = offset.square();
//     }

//     // Send last codeword
//     proof_stream.push(codeword.clone());

//     // Collect last codeword too
//     codewords.push(codeword);

//     codewords
// }

// // Define a method to query the codeword
// fn query(
//     &self,
//     current_codeword: &[FieldElement],
//     next_codeword: &[FieldElement],
//     c_indices: &[usize],
//     proof_stream: &mut ProofStream,
// ) -> Vec<usize> {
//     // Infer a and b indices
//     let a_indices = c_indices.to_vec();
//     let b_indices: Vec<_> = c_indices.iter().map(|&i| i + current_codeword.len() / 2).collect();

//     // Reveal leafs
//     for s in 0..self.num_colinearity_tests {
//         let a = current_codeword[a_indices[s]].clone();
//         let b = current_codeword[b_indices[s]].clone();
//         let c = next_codeword[c_indices[s]].clone();
//         proof_stream.push((a, b, c));
//     }

//     // Reveal authentication paths
//     for s in 0..self.num_colinearity_tests {
//         let proof_a = Merkle::open(a_indices[s], current_codeword);
//         let proof_b = Merkle::open(b_indices[s], current_codeword);
//         let proof_c = Merkle::open(c_indices[s], next_codeword);
//         proof_stream.push(proof_a);
//         proof_stream.push(proof_b);
//         proof_stream.push(proof_c);
//     }

//     // Return a and b indices
//     a_indices.into_iter().chain(b_indices.into_iter()).collect()
// }

}

