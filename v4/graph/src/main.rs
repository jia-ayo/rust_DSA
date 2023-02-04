use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::write;
use std::hash::Hash;
use std::rc::{Weak, Rc};

#[derive(Debug)]
pub struct GraphErr{
    mass:String,
}

pub trait Weighted{
      fn weight(&self)->i32;
}
impl Weighted for i32{
    fn weight(&self)->i32 {
        *self
    }
}

#[derive(Debug)]
pub struct  Route<ID>{
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len:i32
}

impl<ID:Eq> Route<ID>{
    pub fn start_rc(pos:ID)->Rc<Self>{
        Rc::new(Route { 
            pos,
            path: None, 
            len: 0,
        })
    }
    pub fn contains (&self, id:&ID)->bool{
        if self.pos == *id{
            return true;
        }
        match self.path {
          Some(ref p)=> p.contains(id),
          None=>false
        }
    }
}

impl <ID:fmt::Debug> fmt::Display for Route<ID>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref p)= self.path;
        write!(f, "{}-{}-",p,self.ln)
    }
    write!(f, ":?", self.pos); 
}

impl GraphErr{
    pub fn new(s: &str)-> Self{
        GraphErr{
            mass: s.to_string(),
        }
    }
}
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
    data: HashMap<ID, T>,
    edges:  Vec<(E, ID, ID)>,
}

//mappoint based
#[derive(Debug)]
pub struct MapPGraph<T, E, ID:Hash + Eq>{
    data: HashMap<ID, (T, Vec<ID>)>,
    edges:  HashMap<ID, (E, ID, ID)>,
}

impl <T, E, ID: Clone + Hash + Eq> MapPGraph<T, E, ID>{
    pub fn new()->Self{
        MapPGraph { 
            data: HashMap::new(), 
            edges: HashMap::new() 
        }
    }

    pub fn add_node(&mut self, id:ID, dt:T){
        //node as no edges yet
        self.data.insert(id, (dt,Vec::new()));
    }

    pub fn add_edge(&mut self, ed_id:ID, from:ID, to:ID, edat: E)->Result<(),GraphErr>{
        if ! self.data.contains_key(&from){
            return  Err(GraphErr::new("'from' is not in node"));
        }
        if let Some(ref mut dt) = self.data.get_mut(&to){
            self.edges.insert(ed_id.clone(),(edat, from.clone(),to));
            dt.1.push(ed_id.clone());
        }else{
            return Err(GraphErr::new("'to' not in nodes"));
        }
        self.data.get_mut(&from).unwrap().1.push(ed_id);
        Ok(())
    }
}
fn main() ->Result<(), GraphErr> {
    let mut g = MapPGraph::new();
    for x in vec!['A', 'B','C','D','E', 'F','G','H']{
        g.add_node(x, ());
    }
    g.add_edge('a', 'H', 'D', 6)?;
    g.add_edge('b', 'D', 'C', 18)?;
    g.add_edge('c', 'C', 'B', 10)?;
    g.add_edge('d', 'H', 'A', 7)?;
    g.add_edge('e', 'A', 'C', 4)?;
    g.add_edge('f', 'H', 'G', 5)?;
    g.add_edge('g', 'G', 'A', 8)?;
    println!("Hello, GRAPH {:?}",g);
    Ok(())
}
