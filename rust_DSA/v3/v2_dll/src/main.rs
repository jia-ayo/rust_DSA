//refcell is immutable from the outside, but can mutate interior
//Rc reference counting point
use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
pub struct DbNode<T>{
    data:T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbList<T>{
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl <T> DbList<T>{
    pub fn new() -> Self{
        DbList{
            last: None,
           first: None
        } 
    }

    pub fn push_front(&mut self, data:T){
        match self.first.take(){
            Some(r)=>{
                // create new front
                let new_front = Rc::new(RefCell::new(DbNode{
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                //tell the first object this is now infront of it 
                let mut  m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                //push this to the front
                self.first = Some(new_front);
            }
            None =>{
                let new_data = Rc::new(RefCell::new(DbNode{
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
    pub fn push_back(&mut self, data:T){
        match self.last.take(){
            Some(r)=>{
                // create new back object
                let new_back = Rc::new(RefCell::new(DbNode{
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                //tell the last object this is now at its back
                let st = Weak::upgrade(&r).unwrap(); 
                let mut  m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                //push this to the back
                m.next= Some(new_back);
            }
            None =>{
                let new_data = Rc::new(RefCell::new(DbNode{
                    data,
                    prev: None,
                    next: None,
                }));
                self.first = Some(new_data.clone());
                self.last = Some(Rc::downgrade(&new_data));
            }
        }
    }
    
}
fn main() {
    let mut dl = DbList::new();
    dl.push_front(5);
    dl.push_back(2);
    dl.push_front(3);
    dl.push_back(5);
    println!("dl ={:?}", dl);
}
