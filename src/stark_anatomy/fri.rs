use blake2::Blake2b;
use blake2::Digest;
use generic_array::typenum::U64;

use super::{math::field_element::FieldElement, proof_stream::ProofStream, merkle_tree::Merkle};

// Define a new struct to represent the FRI protocol

struct Fri {
    pub offset: FieldElement,
    pub omega: FieldElement,
    domain_length: usize,
    expansion_factor: usize,
    num_colinearity_tests: usize,
}

impl Fri {
    // Initialize FRI protocol with the offset, omega, domain length, expansion factor, and number of colinearity tests
    pub fn new(offset: FieldElement, omega: FieldElement, domain_length: usize, expansion_factor: usize, num_colinearity_tests: usize) -> Self {
        Fri {
            offset,
            omega,
            domain_length,
            expansion_factor,
            num_colinearity_tests,
        }
    }
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
        let codewords = self.commit(codeword.try_into().unwrap(),proof_stream);

        // // Sample indices for the top level and collect them into a list.
        // let top_level_indices = self.sample_indices(proof_stream.prover_fiat_shamir(), codewords[1].len(), codewords.last().unwrap().len(), self.num_colinearity_tests);
        // let mut indices = top_level_indices.iter().clones().collect::<Vec<usize>>; 

        // Sample indices for the top level and collect them into a list.
        let top_level_indices = self.sample_indices(&proof_stream.prover_fiat_shamir(32), codewords[1].len(), codewords.last().unwrap().len(), self.num_colinearity_tests);
        let mut indices = top_level_indices.iter().cloned().collect::<Vec<usize>>();

        // Query phase: For each round, fold the indices and query the codeword.
        // for i in 0..codewords.len() - 1 {
        //     indices = indices.iter().map(|&index| index % (codewords[i].len() / 2)).collect(); // fold
        //     self.query(&codewords[i], &codewords[i + 1], &indices, proof_stream);
        // }

        // Query phase: For each round, fold the indices and query the codeword.
        for i in 0..codewords.len() -1 {
            indices = indices.iter().map(|&index| index% (codewords[i].len() / 2)).collect(); // fold ?
            self.query(&codewords[i], &codewords[i+1], &indices, proof_stream);
        }

        // Return the top-level indices used in the proof.
        // top_level_indices

        // Return the top-level indices used in the proof.
        top_level_indices

    }


// // Define a method to commit to the codeword
// fn commit(&self, mut codeword: Vec<FieldElement>, proof_stream: &mut ProofStream) -> Vec<Vec<FieldElement>> {
fn commit(&self, mut codeword: Vec<FieldElement>, proof_stream: &mut ProofStream) -> Vec<Vec<FieldElement>> {
    let one = FieldElement::one();
    let two = FieldElement::new(2);
    let mut omega = self.omega; // root generator
    let mut offset = self.offset;
    let mut codewords = Vec::new();

    // For each round
    for r in 0..self.num_rounds() {
        // Compute and send Merkle root to proof_stream
        let root = Merkle::commit(&field_elements_to_bytes(&codeword));
        proof_stream.push(root.to_vec());

        // Prepare next round, if necessary
        if r == self.num_rounds() -1 {
            break;
        }

        // Get challenge
        let alpha = FieldElement::sample(proof_stream.prover_fiat_shamir(32).as_slice());

        // Collect codeword
        codewords.push(codeword.clone());

        // Split and fold
        let half_length = codeword.len() / 2;
        codeword = (0..half_length).map(|i| {
            let alpha_i = alpha / (offset *omega.pow(i.into()));
            let c0 = (one + alpha_i) * &codeword[i];
            let c1 = (one - alpha_i) * &codeword[half_length + i];
            two.inverse() * &(c0 + c1)
        })
        .collect();
        omega = omega.pow(2);
        offset = offset.pow(2);
    }

    // Send last codeword
    // proof_stream.push(serialize_codeword(&codeword));

    // collect last codeword too
    codewords.push(codeword);

    codewords
    
}

// Define a method to query the codeword
fn query(&self, current_codeword: &[FieldElement], next_codeword: &[FieldElement], c_indices: &[usize], proof_stream: &mut ProofStream) -> Vec<usize> {
    // Infer a and b indices
    let a_indices = c_indices.to_vec();
    let b_indices: Vec<_> = c_indices.iter().map(|&i| i + current_codeword.len() / 2).collect();

    // Reveal leafs
    for s in 0..self.num_colinearity_tests{
        let a = current_codeword[a_indices[s]].clone();
        let b = current_codeword[b_indices[s]].clone();
        let c = next_codeword[c_indices[s]].clone();
        proof_stream.push([
            field_elements_to_bytes(&[a.clone()]),
            field_elements_to_bytes(&[b.clone()]),
            field_elements_to_bytes(&[c.clone()]),
        ].concat());
    }

    // Reveal authetication paths
    for s in 0..self.num_colinearity_tests {
        let proof_a = Merkle::open(a_indices[s], &field_elements_to_bytes(current_codeword));
        let proof_b = Merkle::open(b_indices[s], &field_elements_to_bytes(current_codeword));
        let proof_c = Merkle::open(c_indices[s], &field_elements_to_bytes(next_codeword));
        proof_stream.push(proof_a);
        proof_stream.push(proof_b);
        proof_stream.push(proof_c);
    }

    // Return a and b indices
    a_indices.into_iter().chain(b_indices.into_iter()).collect()
}

    pub fn sample_indices(&self, seed: &[u8], size: usize, reduced_size: usize, number: usize) -> Vec<usize> {
        assert!(number <= reduced_size, "cannot sample more indices than available in last codeword");
        assert!(number <= 2 * reduced_size, "not enough entropy in indices wrt last codeword");

        let mut indices = Vec::new();
        let mut reduced_indices = Vec::new();
        let mut counter: usize = 0;
        while indices.len() < number {
            let mut hasher : Blake2b<U64> = Blake2b::new();
            hasher.update(seed);
            hasher.update(&counter.to_be_bytes());
            let index = Fri::sample_index(&hasher.finalize()[..], size);
            let reduced_index = index % reduced_size;
            counter += 1;
            if !reduced_indices.contains(&reduced_index) {
                indices.push(index);
                reduced_indices.push(reduced_index);
            }
        }
        indices
    }


    pub fn sample_index(byte_array: &[u8], size: usize) -> usize {
        let mut acc = 0;
        for b in byte_array {
            acc = (acc << 8) ^ *b as usize;
        }
        acc % size
    }
}

fn field_elements_to_bytes(elements: &[FieldElement]) -> Vec<u8> {
    elements.iter().flat_map(|elem| elem.to_bytes()).collect()
}




