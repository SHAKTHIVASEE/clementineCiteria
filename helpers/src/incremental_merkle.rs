use crate::{
    config::ZEROES,
    constant::{Data, EMPTYDATA},
    sha256_hash,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct IncrementalMerkleTree<const DEPTH: usize>
where
    [Data; DEPTH]: Serialize + DeserializeOwned + Copy,
{
    pub filled_subtrees: [Data; DEPTH],
    pub root: Data,
    pub index: u32,
}

impl<const DEPTH: usize> Default for IncrementalMerkleTree<DEPTH>
where
    [Data; DEPTH]: Serialize + DeserializeOwned + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const DEPTH: usize> IncrementalMerkleTree<DEPTH>
where
    [Data; DEPTH]: Serialize + DeserializeOwned + Copy,
{
    pub fn new() -> Self {
        Self {
            filled_subtrees: [EMPTYDATA; DEPTH],
            root: ZEROES[DEPTH],
            index: 0,
        }
    }

    pub fn add(&mut self, a: Data) {
        let mut current_index = self.index;
        let mut current_level_hash = a;

        for i in 0..DEPTH {
            let (left, right) = if current_index % 2 == 0 {
                self.filled_subtrees[i] = current_level_hash;
                (current_level_hash, ZEROES[i])
            } else {
                (self.filled_subtrees[i], current_level_hash)
            };
            current_level_hash = sha256_hash!(left, right);
            current_index /= 2;
        }
        self.root = current_level_hash;
        self.index += 1;
    }
}
