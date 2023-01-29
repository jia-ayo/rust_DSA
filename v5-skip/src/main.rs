use std::cell::RefCell;
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

fn main() {
    let mut s = SkipNode::new(4);
    s.insert(4);
    s.insert(77);
    s.insert(74);
    s.insert(94);
    println!("s={:?}", s);
}
