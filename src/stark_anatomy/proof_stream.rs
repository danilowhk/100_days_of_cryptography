use bincode;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Default, Serialize, Deserialize)]
pub struct ProofStream {
    objects: Vec<String>,
    read_index: usize,
}

impl ProofStream {
    // A funciton to push an object into the vector
    pub fn push(&mut self, obj: String) {
        self.objects.push(obj);
    }

    // A funciton to pull an object from the vector
    // Returns None if read_index > objects.len()
    pub fn pull(&mut self) -> Option<&String> {
        if self.read_index < self.objects.len() {
            let obj = &self.objects[self.read_index];
            self.read_index += 1;
            Some(&obj)
        } else {
            None
        }
    }

    // A functino to serialize the ProofStream object to a vector of bytes
    pub fn serialize2(self) -> Vec<u8> {
        bincode::serialize(&self.objects).unwrap() // Use bincode to serialize the objects vector to a vector of bytes
    }

    // A function to deserialize the ProofStream object from a vector of bytes
    pub fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }

    pub fn prover_fiat_shamir(self, num_bytes: usize) -> [u8; 32] {
        // Create a new Sha3_256 hasher
        let mut hasher = Sha3_256::new();
        // Serialize the ProofStream object
        let serialized = self.serialize2();
        // Update the hasher with the serialized object and finalize
        hasher.update(&serialized);
        let hash = hasher.finalize();
        // Create a new byte array of length 32
        let mut result = [0_u8; 32];
        // Copy the first num_bytes of the hash to the result byte array (wouldn't it always be 32 bytes though?)
        result.copy_from_slice(&hash[..num_bytes]);
        // Return result
        result
    }

    pub fn verifier_fiat_shamir(self, num_bytes: usize) -> [u8; 32] {
        // Create a new Sha3_256 hasher
        let mut hasher = Sha3_256::new();
        // Serialize only the objects that have been read from the vector
        let serialized = bincode::serialize(&self.objects[..self.read_index]).unwrap();
        // Update the hasher with the serialized objects
        hasher.update(&serialized);
        let hash = hasher.finalize();
        let mut result = [0_u8; 32];
        // Copy the hashed data on results
        result.copy_from_slice(&hash[..]);
        //return the byte array
        result
    }
}
