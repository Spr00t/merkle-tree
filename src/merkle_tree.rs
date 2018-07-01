use std::rc::Rc;
use std::fmt::Display;
use std::mem::discriminant;
type THash = Vec<u8>;

struct MerkleLeaf<T> {
    data: T,
    hash: THash
}

struct MerkleNode<T> {
    sons: [Rc<MerkleNode<T>>; 2],
    hash: THash,
    items_count: usize
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MerkleKnot<T> 
{
    Leaf(MerkleLeaf),
    Node(MerkleNode),
    Nil
}
//MerkleNode
impl<T> Clone for MerkleNode<T> 
    where T:Clone
{
    fn clone(&self) -> MerkleNode<T> {
        match self {
            MerkleNode::Leaf{data, hash} => {
                return MerkleNode::Leaf {
                    data: (*data).clone(),
                    hash: hash.clone()
                };
            },
            MerkleNode::Node{sons, hash, items_count} => {
                return MerkleNode::Node {
                    sons: [sons[0].clone(), sons[1].clone()],
                    hash: hash.clone(),
                    items_count: *items_count
                };
            },
            MerkleNode::Nil => {
                return MerkleNode::Nil;
            },
        }
    }
}


pub struct MerkleTree<T> {
    pub root: MerkleNode<T>
}

impl<T> MerkleTree<T> 
    where T: Clone
{
    pub fn new() -> MerkleTree<T>
        where T: AsRef<[u8]> 
    {
        MerkleTree::<T> {
            root: MerkleNode::Nil
        }
    }
    pub fn insert<F>(&mut self, t: T, f: F)
        where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]>
    {
        insert(&mut self.root, t, f);
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
pub fn to_xml<T, F>(node: &MerkleNode<T>, t: T, mut f: F)
{
    
}

#[allow(dead_code)]
pub fn create_node_from_data<T, F>( item: T, opt_hash: Option<THash>, f: &mut F) -> MerkleNode<T>
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{
    if let Some(hash) = opt_hash {
        MerkleNode::Leaf {
            data: item,
            hash: hash
        }
    } else {
        
        MerkleNode::Leaf {
            data: item.clone(),
            hash: f(item.as_ref())
        }
    }
}

#[allow(dead_code)]
pub fn create_node_from_leaf<T, F>(leaf: MerkleLeaf<T>, f: &mut F) -> MerkleNode<T>
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{
    MerkleNode::Nil
}


pub fn insert<T, F>(node: &mut MerkleNode<T>, t: T, mut f: F)
    where F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{

    match node {
        MerkleNode::Leaf(_) => {
            unreachable!();
        },
        MerkleNode::Nil => {
            unreachable!();
        },
        MerkleNode::Node(ref mut node) => {

            let leaf_data: T;
            let leaf_hash: THash;
            
            let son: Rc<MerkleNode<T>> = Rc::clone(&sons[0]);
            match *son {
                MerkleNode::Leaf{ref data, ref hash} => {
                    let mut combined_hash: Vec<u8> = hash.clone();
                    leaf_data = data.clone();
                    leaf_hash = hash.clone();
                    let left_son: MerkleNode<T> = create_node_from_data(leaf_data, Some(leaf_hash), &mut f);

                    let right_hash: Vec<u8> = f(t.as_ref());
                    let right_son: MerkleNode<T> = create_node_from_data(t.clone(), Some(right_hash.clone()), &mut f);

                    combined_hash.extend(right_hash);

                    sons[0] = Rc::new(MerkleNode::Node {
                        sons: [Rc::new(MerkleNode::Nil), Rc::new(MerkleNode::Nil)],
                        hash: combined_hash,
                        items_count: *items_count
                    });
                },
                MerkleNode::Nil => {
                    unreachable!();
                },
                MerkleNode::Node{ref sons, ref hash, ref items_count} => {
                    unreachable!();
                }
            }
        }
    }
}