use std::rc::Rc;
use std::fmt;
use std::fmt::{Display, Write, Formatter};
use std::convert::From;
use std::mem::replace;
use std::ops::IndexMut;
use std::string::ToString;

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

impl<T> Display for MerkleKnot<T>
    where T: Display
{
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        match self {
            MerkleKnot::Leaf(leaf) => {
                buf.write_fmt(format_args!("<leaf><data>{}</data> <hash>{:?}</hash></leaf>", &leaf.data, &leaf.hash)).unwrap();        
            },
            MerkleKnot::Nil => {
                buf.write_str("<NUL/>").unwrap();        
            },
            MerkleKnot::Node(node) => {
                buf.write_fmt(format_args!("<node>\n<left_son>{}</left_son>\n<right_son>{}</right_son><hash>{:x?}</hash></node>", &*node.sons[0], &*node.sons[1], &node.hash)).unwrap();
            }
        }

        buf.shrink_to_fit();
        write!(f, "{}", buf)
    }
}
impl<T> Display for MerkleTree<T>
    where T: Display
{
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{}", &self.root)
    }
}
pub struct MerkleTree<T> {
    pub root: MerkleKnot<T>,
    hasher: Box<FnMut(&[u8]) -> Vec<u8>>
}

impl<T> MerkleTree<T> 
    where T: Clone
{
    pub fn new<F>(f: Box<F>) -> MerkleTree<T>
        where F: 'static + for<'a> FnMut(&'a [u8]) -> Vec<u8>, T: AsRef<[u8]> 
    {
        MerkleTree::<T> {
            root: MerkleKnot::Nil,
            hasher: f
        }
    }
    pub fn insert(&mut self, t: T)
        where T: AsRef<[u8]>
    {
        let mut knot: MerkleKnot<T> = replace(&mut self.root, MerkleKnot::Nil);
        knot = insert(knot, t, &mut *self.hasher);
        replace(&mut self.root, knot);

    }
}

#[allow(dead_code)]
pub fn leaf_to_xml<T, F>( data: T) -> String
    where T: AsRef<[u8]> + Clone + Display
{
    let mut result = String::new();
    result.push_str("<leaf>");
    result
}

#[allow(dead_code)]
fn leaf_t_xml<T, F>(node: &MerkleNode<T>) -> String
{
    let mut result = String::new();
    result.push_str("<leaf>");
    result
}

// #[allow(dead_code)]
// fn knot_to_xml<T, F>(knot: &MerkleKnot<T>) -> String
// {
//     let mut result = String::new();
//       match node {
//         MerkleKnot::Leaf(leaf) => {
//             MerkleKnot::Node(create_node_from_leaf(leaf, t, &mut f))
//         },
//         MerkleKnot::Nil => {
//              MerkleKnot::Leaf(create_leaf_from_data(t, None, &mut f))
//         },
//         MerkleKnot::Node(node) => {
//             MerkleKnot::Node(insert_to_node(node, t, &mut f))
//         }
//     }

//     result.push_str("<leaf>");
//     result
// }

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

fn insert_to_node<T, F>(mut node: MerkleNode<T>, data: T, f: &mut F) -> MerkleNode<T>
    where for<'r> F: FnMut(&[u8]) -> Vec<u8>, T: AsRef<[u8]> + Clone
{

    let counts: Vec<usize> = node.sons.iter().map(|node| {
        match &**node {
            MerkleKnot::Leaf(_) => {
                1
            },
            MerkleKnot::Nil => {
                0
            },
            MerkleKnot::Node(node) => {
                node.items_count
            }
        }
    }).collect();

    let insert_index = 
    if counts[0] < counts[1] {
        0
    } else {
        1
    };

    let owned_node: MerkleKnot<T> = {
        let node0: &mut Rc<MerkleKnot<T>> = &mut node.sons[insert_index];
        
        let rc_node: &mut MerkleKnot<T> = Rc::get_mut(node0).unwrap();
        replace(rc_node, MerkleKnot::Nil)
    };
    
    match owned_node {
        MerkleKnot::Leaf(leaf) => {
            let m_node: Rc<MerkleKnot<T>> = Rc::from(MerkleKnot::Node(create_node_from_leaf(leaf, data, f)));
            
            node.sons[insert_index] = m_node;
        },
        MerkleKnot::Nil => {
            node.sons[insert_index] = Rc::from(MerkleKnot::Leaf(create_leaf_from_data(data, None, f)));;
        },
        MerkleKnot::Node(node) => {
            insert_to_node(node, data, f);
        }
    }   

    
    node
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
            MerkleKnot::Node(insert_to_node(node, t, &mut f))
        }
    }
}