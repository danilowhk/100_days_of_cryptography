use blake2::{Blake2s256, Digest};

struct Merkle;

impl Merkle {
    // use Blake2b as the hash funciton and commit
    fn commit_(leafs: &[u8]) -> [u8; 32] {
        // ensure that the length of the leafs is a power of two
        assert!(leafs.len().is_power_of_two(), "length must be power of two");
        if leafs.len() == 1 {
            return leafs.try_into().unwrap();
        } else {
            // recursively commit the left and right halves of the leafs
            let left = Merkle::commit_(&leafs[..leafs.len() / 2]);
            let right = Merkle::commit_(&leafs[leafs.len() / 2..]);
            // concatenate and hash the left and right hashes
            let mut hasher = Blake2s256::new();

            Blake2s256::digest(Self::concatenate(&left, &right)).into()
            // hasher.finalize().to_vec()
        }
    }

    // concatenate two byte slices
    fn concatenate(left: &[u8], right: &[u8]) -> Vec<u8> {
        // create a new Vec for left values
        let mut concatenated: Vec<u8> = (*left).into();
        // create a new Vec for right values
        let mut right_node_clone: Vec<u8> = (*right).into();
        concatenated.append(&mut right_node_clone);
        concatenated
    }

    fn open_(index: usize, leafs: &[u8]) -> Vec<u8> {
        // ensure that the length of the leafs is a power of two
        assert!(leafs.len().is_power_of_two(), "length must be power of two");
        // ensure that the index is within the bounds of the leafs array
        assert!(index < leafs.len(), "cannot open invalid index");
        if leafs.len() == 2 {
            return vec![leafs[1 - index]].clone();
        } else if index < leafs.len() / 2 {
            // recursively open the left half of the leafs
            let mut proof = Merkle::open_(index, &leafs[..leafs.len() / 2]);
            proof.extend_from_slice(&Self::commit_(&leafs[leafs.len() / 2..]));
            // commit the right half of the leafs and add it to the proof
            return proof;
        } else {
            // recursively open the right half of the leafs
            let mut proof = Self::open_(index - leafs.len() / 2, &leafs[leafs.len() / 2..]);
            // commit the leaft half of the leafs and add it to the proof
            proof.extend_from_slice(&Self::commit_(&leafs[..leafs.len() / 2]));
            return proof;
        }
    }
    // verify the proof of a leaf at a given index given the root
    fn verify(&self, root: [u8; 32], index: usize, path: &[&[u8]], leaf: [u8; 32]) -> bool {
        // ensure that the index is within the bounds of the leafs array
        assert!(index < 1 << path.len(), "cannot verify invalid index");
        // If the path contains only one node, we're at the leaf level
        if path.len() == 1 {
            // If the index is 0, the leaf is the left child of the root
            if index == 0 {
                return root
                    == <[u8; 32]>::from(Blake2s256::digest(Self::concatenate(&leaf, path[0])));
            }
            // Else, the leaf is the right child of the root
            else {
                return root
                    == <[u8; 32]>::from(Blake2s256::digest(Self::concatenate(path[0], &leaf)));
            }
        } else {
            // If the index is even, the leaf is the left child of the root
            if index % 2 == 0 {
                let new_leaf =
                    <[u8; 32]>::from(Blake2s256::digest(Self::concatenate(&leaf, path[0])));
                return self.verify(root, index >> 1, &path[1..], new_leaf);
            }
            // Else, the leaf is the right child of the root
            else {
                let new_leaf =
                    <[u8; 32]>::from(Blake2s256::digest(Self::concatenate(path[0], &leaf)));
                return self.verify(root, index >> 1, &path[1..], new_leaf);
            }
        }
    }
}
