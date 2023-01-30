use std::cell::RefCell;
use std::fmt::{Debug, Write, self};
use std::rc::Rc;


type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t:T) -> Rcc<T>{
    Rc::new(RefCell::new(t))
}

#[derive(Debug)]
pub struct SkipNode<T:PartialOrd>{
    right: Option<Rcc<SkipNode<T>>>,
    down:  Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

#[derive(Debug)]
pub struct SkipList<T:PartialOrd>(Vec<SkipNode<T>>);

impl<T:PartialOrd> SkipNode<T>{
    pub fn new(t:T) -> Self{
        SkipNode { 
            right: None, 
            down: None, 
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, dt: T) -> Option<Rcc<SkipNode<T>>>{
        //bigger than right then go right
        if let Some(ref mut rt) =self.right{
            if dt> *rt.borrow().data.borrow(){
                return rt.borrow_mut().insert(dt);
            }
        }

        //has lower chidren try then
        if let Some( ref dw ) = self.down{
            return match dw.borrow_mut().insert(dt){
                Some(child) => match rand::random::<bool>() {
                    true =>{
                        let dt = child.borrow().data.clone();//pointer copy
                        let nn = SkipNode{
                            right: self.right.take(),
                            data:dt,
                            down: Some(child)
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    },
                    false=>None
                }
                None => None,
            };
        }

        //should be before right hand object, at buttom node 
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)

    }
}

impl <T:Debug + PartialOrd> SkipNode<T>{
    pub fn print_row<W:Write>(&self, w:&mut W)-> std::fmt::Result{
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref r) = self.right{
            write!(w, ",")?;
            r.borrow().print_row(w)?;
        }
        Ok(())
    }
}

impl <T: PartialOrd> SkipList<T>{
    pub fn new() -> Self{
        SkipList(Vec::new())
    }

    pub fn insert(&mut self, data:T){
        if self.0.len() == 0{
            self.0.push(SkipNode::new(data));
            return;
        } 
        //our vec will have the lowest row with the lowest numbers
        //we need to try and insert the heighest available row
        for i in (0..self.0.len()).rev(){
            if data > *self.0[i].data.borrow(){
                if let Some(ch) = self.0[i].insert(data){
                    self.loop_up(ch, i+1);
                    //todo loop up in 50-50 chance
                }
                return;
            }
        }
        //if none of those succeded, that means we have an element to replace the first
        let mut nn = SkipNode::new(data);
        std::mem::swap(&mut nn,&mut self.0[0]);//put our new element int he front of the row
        let res = rcc(nn);
        self.0[0].right = Some(res.clone());
        self.loop_up(res,1);
        //TODO loop up on a 50-50 
    }
    
    pub fn loop_up(&mut self, ch:Rcc<SkipNode<T>>, n:usize){
        if rand::random::<bool>() == true {
            return;
        }
        let dt = ch.borrow().data.clone();
        let mut nn = SkipNode{
            right:None,
            down: Some(ch),
            data: dt,
        };
        if n >= self.0.len(){
            self.0.push(nn);
            return;
        }
        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rcc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n+1);

    }

}

impl <T:Debug + PartialOrd> fmt::Display for SkipList<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len()==0{
            return  write!(f, "SkipList<Empty>");
        }

        for sn in &self.0{
            write!(f, "\n")?;
            sn.print_row(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut s = SkipList::new();
    s.insert(4);
    s.insert(77);
    s.insert(74);
    s.insert(94);
    s.insert(95);
    s.insert(85);
    s.insert(76);
    println!("s={}", s);
}
