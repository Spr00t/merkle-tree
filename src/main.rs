extern crate md5;

mod merkle_tree;

use merkle_tree::{MerkleTree};


#[derive(Debug, Clone)]
struct Data {
    data: Vec<u8>
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Data {
            data: data
        }
    }
}
impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> & [u8] {
        &self.data
    }
}

fn main() {
    println!("Hello, world!");
    let mut mtree = MerkleTree::<Data>::new();
    let val = Data {data: vec![1,2,3]};
    mtree.insert(val, |data: &[u8]| {
        let mut value = Vec::<u8>::new();
        value.extend(md5::compute(data).iter());
        value
    });
}
