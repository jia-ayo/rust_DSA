use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Weak, Rc};

type Rcc<T> = Rc<RefCell<T>>;
pub fn rcc<T>(t: T) -> Rcc<T>{
    Rc::new(RefCell::new(t))
}

//edglist
pub struct EdgeListGraph<E, ID>{
   //data on the edge at e dont care too much about the nodes
   //simple but can be slow transversal
   v: Vec<(E, ID, ID)>,
}

//pointer based 
//good for directed graph as edges go one way
//using weak point means the edge will fail safely if node as been removed
//can stick edge data if needed
pub struct RccGraph<T, E>{
   nodes: Vec<Rcc<RccNode<T, E>>>,
}

pub struct RccNode<T, E>{
    data: T,
    edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>
}

//Map based
//Maps point from  key to value normally quickly eg Hashmap
pub struct MapGraph<T, E, ID:Hash + Eq>{
    mp: HashMap<ID, T>,
    edges:  Vec<(E, ID, ID)>,
}

//mappoint based
pub struct MapPGraph<T, E, ID:Hash + Eq>{
    mp: HashMap<ID, (T, Vec<ID>)>,
    edges:  HashMap<ID, (E, ID, ID)>,
}

fn main() {
    println!("Hello, world!");
}
