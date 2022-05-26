use crate::merkleization::Node;
use sha2::{Digest, Sha256};

pub fn is_valid_merkle_branch<'a>(
    leaf: &Node,
    mut branch: impl Iterator<Item = &'a Node>,
    depth: usize,
    index: usize,
    root: &Node,
) -> bool {
    let mut value = *leaf;

    let mut hasher = Sha256::new();
    for i in 0..depth {
        let next_node = match branch.next() {
            Some(node) => node,
            None => return false,
        };
        if (index / 2usize.pow(i as u32)) % 2 != 0 {
            hasher.update(&next_node.0);
            hasher.update(&value.0);
        } else {
            hasher.update(&value.0);
            hasher.update(&next_node.0);
        }
        value.0.copy_from_slice(&hasher.finalize_reset());
    }
    value == *root
}
