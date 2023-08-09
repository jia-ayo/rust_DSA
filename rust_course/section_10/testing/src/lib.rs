pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn  simple_add()-> bool{
    if 2+2 ==4 {
        true
    }else{
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }
 
    #[test]
    #[should_panic]
    fn it_fails(){
        panic!("test fails")
    }
    #[test]
    fn call_simple_add(){
        assert!(simple_add())
    }
}
