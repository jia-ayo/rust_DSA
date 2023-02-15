#[derive(Copy,Clone, PartialEq, Debug)]
pub struct GenData{
    pos: usize,
    gen: u64,
}

pub struct EntityActive{
    active: bool,
    gen: u64,
}

//wher we get new GenerationIds from 
pub struct GenManger{
    items: Vec<EntityActive>,
    drops:Vec<usize>,//list of all drop
}

impl GenManger{
    pub fn new()->Self{
        GenManger { 
            items: Vec::new(), 
            drops: Vec::new() 
        }
    }

    pub fn next (&mut self) -> GenData{
        if let Some(loc) = self.drops.pop(){
            //most recent drop 
            let ea = &mut  self.items[loc];
            ea.active = true;
            ea.gen += 1;
            return GenData{
                pos: loc,
                gen: ea.gen
            }
        }
        //if nothing left in drops , add to the end 
        self.items.push(EntityActive {
             active: true, 
             gen: 0
            }
        );
        return GenData{
            gen: 0,
            pos: self.items.len() - 1,
        };

    }

    pub fn drop(&mut self, g:GenData){
        if let Some(ea) = self.items.get_mut(g.pos){
            if ea.active && ea.gen == g.gen {
                //dont drop newer item than given 
                ea.active = false;
                self.drops.push(g.pos);

            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_items_drop(){
        let mut gm = GenManger::new();

        let  g = gm.next();
        assert_eq!(g,GenData{gen:0,pos:0})
    }
}