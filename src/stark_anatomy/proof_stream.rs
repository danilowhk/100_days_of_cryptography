use std::{any::Any, sync::Arc, error::Error};

use bincode;
use serde::{Deserialize, Serialize};
use sha3::{Shake256, digest::{Update, XofReader, ExtendableOutput}};
use std::io::Read;

#[derive(Default, Serialize, Deserialize)]
pub struct ProofStream {
    objects: Vec<Vec<u8>>,
    read_index: usize,
}

impl ProofStream {
    pub fn new() -> Self {
        ProofStream {
            objects: Vec::new(),
            read_index: 0,
        }
    }
    // A funciton to push an object into the vector
    pub fn push(&mut self, obj: Vec<u8>) {
        self.objects.push(obj);
    }

    // A funciton to pull an object from the vector
    // Returns None if read_index > objects.len()
    pub fn pull(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        if self.read_index < self.objects.len() {
            let obj = self.objects[self.read_index].clone();
            self.read_index += 1;
            Ok(obj)
        } else {
            Err("ProofStream: cannot pull object; queue empty.".into())
        }
    }

    // A functino to serialize the ProofStream object to a vector of bytes
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self.objects).unwrap()
    }

    // A function to deserialize the ProofStream object from a vector of bytes
    // pub fn deserialize(data: &[u8]) -> Self {
    //     bincode::deserialize(data).unwrap()
    // }

    // pub fn prover_fiat_shamir(self, num_bytes: usize) -> [u8; 32] {
    //     // Create a new Sha3_256 hasher
    //     let mut hasher = Sha3_256::new();
    //     // Serialize the ProofStream object
    //     let serialized = self.serialize2();
    //     // Update the hasher with the serialized object and finalize
    //     hasher.update(&serialized);
    //     let hash = hasher.finalize();
    //     // Create a new byte array of length 32
    //     let mut result = [0_u8; 32];
    //     // Copy the first num_bytes of the hash to the result byte array (wouldn't it always be 32 bytes though?)
    //     result.copy_from_slice(&hash[..num_bytes]);
    //     // Return result
    //     result
    // }

    pub fn prover_fiat_shamir(&self, num_bytes: usize) -> Vec<u8> {
        let mut hasher = Shake256::default();
        hasher.update(&self.serialize());
        let mut output = vec![0_u8; num_bytes];
        hasher.finalize_xof().take(num_bytes.try_into().unwrap()).read_exact(&mut output).unwrap();
        output
    }
    // pub fn verifier_fiat_shamir(self, num_bytes: usize) -> [u8; 32] {
    //     // Create a new Sha3_256 hasher
    //     let mut hasher = Sha3_256::new();
    //     // Serialize only the objects that have been read from the vector
    //     let serialized = bincode::serialize(&self.objects[..self.read_index]).unwrap();
    //     // Update the hasher with the serialized objects
    //     hasher.update(&serialized);
    //     let hash = hasher.finalize();
    //     let mut result = [0_u8; 32];
    //     // Copy the hashed data on results
    //     result.copy_from_slice(&hash[..]);
    //     //return the byte array
    //     result
    // }

    pub fn verifier_fiat_shamir(&self, num_bytes: usize) -> Vec<u8> {
        let mut hasher = Shake256::default();
        hasher.update(&bincode::serialize(&self.objects[..self.read_index]).unwrap());
        let mut output = vec![0_u8; num_bytes];
        hasher.finalize_xof().take(num_bytes.try_into().unwrap()).read_exact(&mut output).unwrap();
        output
    }

    pub fn deserialize(bb: Vec<u8>) -> ProofStream {
        let objects: Vec<Vec<u8>> = bincode::deserialize(&bb).unwrap();
        ProofStream {
            objects,
            read_index: 0,
        }
    }
}

#[test]
fn test_serialize() {
    let mut proof1 = ProofStream::new();
    proof1.push(bincode::serialize(&1).unwrap());
    proof1.push(serde_json::to_vec(&serde_json::json!({"1": "1"})).unwrap());
    proof1.push(serde_json::to_vec(&serde_json::json!([1])).unwrap());
    proof1.push(bincode::serialize(&2).unwrap());

    let serialized = proof1.serialize();
    let mut proof2 = ProofStream::deserialize(serialized);

    assert_eq!(bincode::deserialize::<i32>(&proof1.pull().unwrap()).unwrap(), bincode::deserialize::<i32>(&proof2.pull().unwrap()).unwrap(), "pulled object 0 don't match");
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&proof1.pull().unwrap()).unwrap(), serde_json::from_slice::<serde_json::Value>(&proof2.pull().unwrap()).unwrap(), "pulled object 1 don't match");
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&proof1.pull().unwrap()).unwrap(), serde_json::from_slice::<serde_json::Value>(&proof2.pull().unwrap()).unwrap(), "pulled object 2 don't match");
    assert_eq!(bincode::deserialize::<i32>(&proof1.pull().unwrap()).unwrap(), 2, "object 3 pulled from proof1 is not 2");
    assert_eq!(bincode::deserialize::<i32>(&proof2.pull().unwrap()).unwrap(), 2, "object 3 pulled from proof2 is not 2");
    assert_eq!(
        proof1.prover_fiat_shamir(32),
        proof2.prover_fiat_shamir(32),
        "fiat shamir is not the same"
    );
}
