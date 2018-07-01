use std::rc::Rc;
use std::fmt::Display;
use std::convert::From;
use std::mem::replace;

type THash = Vec<u8>;

#[derive(Debug)]
struct MerkleLeaf<T> {
    data: T,
    hash: THash
}

impl<T> Clone for MerkleLeaf<T> 
    where T:Clone 
{
    fn clone(&self) ->  MerkleLeaf<T> 
    {
        MerkleLeaf {
            data: self.data.clone(),
            hash: self.hash.clone()
        }
    }
}
#[derive(Debug)]
struct MerkleNode<T> {
    sons: [Rc<MerkleKnot<T>>; 2],
    hash: THash,
    items_count: usize
}


impl<T> Clone for MerkleNode<T> 
    where T:Clone 
{
    fn clone(&self) ->  MerkleNode<T> 
    {
        MerkleNode {
            sons: self.sons.clone(),
            hash: self.hash.clone(),
            items_count: self.items_count
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MerkleKnot<T> 
{
    Leaf(MerkleLeaf<T>),
    Node(MerkleNode<T>),
    Nil
}


pub struct MerkleTree<T> {
    pub root: MerkleKnot<T>
}

impl<T> MerkleTree<T> 
    where T: Clone
{
    pub fn new() -> MerkleTree<T>
        where T: AsRef<[u8]> 
    {
        MerkleTree::<T> {
            root: MerkleKnot::Nil
        }
    }
    pub fn insert<F>(&mut self, t: T, f: F)
        where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]>
    {
        let mut knot: MerkleKnot<T> = replace(&mut self.root, MerkleKnot::Nil);
        knot = insert(knot, t, f);
        replace(&mut self.root, knot);

    }
}

#[allow(dead_code)]
pub fn leaf_to_xml<T, F>( data: T, hash: THash) -> String
    where T: AsRef<[u8]> + Clone + Display
{
    let mut result = String::new();
    result.push_str("<leaf>");
    result
}

#[allow(dead_code)]
fn to_xml<T, F>(node: &MerkleNode<T>, t: T, mut f: F)
{
    
}

#[allow(dead_code)]
fn create_leaf_from_data<T, F>( item: T, opt_hash: Option<THash>, f: &mut F) -> MerkleLeaf<T>
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{
    if let Some(hash) = opt_hash {
        MerkleLeaf {
            data: item,
            hash: hash
        }
    } else {
        MerkleLeaf {
            data: item.clone(),
            hash: f(item.as_ref())
        }
    }
}

#[allow(dead_code)]
fn create_node_from_leaf<T, F>(leaf: MerkleLeaf<T>, data: T, f: &mut F) -> MerkleNode<T>
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{
    let mut sons = [Rc::from(MerkleKnot::Nil), Rc::from(MerkleKnot::Nil)];
    let hash: THash;
    let items_count: usize;


    let mut combined_hash: Vec<u8> = leaf.hash.clone();

    let right_leaf: MerkleLeaf<T> = create_leaf_from_data(data, None, f);

    combined_hash.extend(&right_leaf.hash); 

    hash = combined_hash;
    items_count = 2;

    sons[0] = Rc::from(MerkleKnot::Leaf(leaf));
    sons[1] = Rc::from(MerkleKnot::Leaf(right_leaf));

    MerkleNode {
        sons: sons,
        hash: hash,
        items_count: items_count
    }
}

fn create_node_from_node<T, F>(node: MerkleNode<T>, data: T, f: &mut F) -> MerkleNode<T>
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{
    let mut sons = [Rc::from(MerkleKnot::Nil), Rc::from(MerkleKnot::Nil)];
    let hash: THash;
    let items_count: usize;


    let mut combined_hash: Vec<u8> = node.hash.clone();

    let right_leaf: MerkleLeaf<T> = create_leaf_from_data(data, None, f);

    combined_hash.extend(&right_leaf.hash); 

    hash = combined_hash;
    items_count = node.items_count + 1;

    sons[0] = Rc::from(MerkleKnot::Node(node));
    sons[1] = Rc::from(MerkleKnot::Leaf(right_leaf));

    MerkleNode {
        sons: sons,
        hash: hash,
        items_count: items_count
    }
}


pub fn insert<T, F>(node: MerkleKnot<T>, t: T, mut f: F) -> MerkleKnot<T>
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{
    match node {
        MerkleKnot::Leaf(leaf) => {
            MerkleKnot::Node(create_node_from_leaf(leaf, t, &mut f))
        },
        MerkleKnot::Nil => {
             MerkleKnot::Leaf(create_leaf_from_data(t, None, &mut f))
        },
        MerkleKnot::Node(node) => {
            unreachable!();
        }
    }
}