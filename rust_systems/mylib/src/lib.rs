//!  This is my my module documentation. My library is so nice!
//! 
/// four is a function that returns `4`
/// ```
/// use mylib::four;
/// let x = four();
/// assert_eq!(x, 4)
/// ```
pub fn four() -> i32 {
   return 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = four();
        assert_eq!(result, 4);
    }
}

//create documentation 
// cargo doc --no-deps --open
// test code
//cargo test 
