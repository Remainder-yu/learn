use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTreeProps {
    root: Node<T>,
    props:BTreeProps,
}

struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index:usize,
}

impl<T> Node<T> where T: Ord, {
    
}