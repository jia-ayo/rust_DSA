use std::fmt::Debug;
//O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]){
    for p in 0 ..v.len(){
        
        let mut sorted = true;
        for i in 0 ..(v.len()-1)-p{
            if v[i] > v[i+1]{
                v.swap(i, i+1);
                sorted = false
            }
        }
        println!("{:?}", v);
        if sorted{
            return;
        }

    }
}

pub fn marge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T>{
    //sort the left half
    //sort the right half
    //bring sorted half together
    if v.len() <= 1{
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len()/2);
    let a= marge_sort(v);
    let b = marge_sort(b);

    //bring them together again add whichever as the lowest to the front or b 

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        match a_peek{
           Some(ref a_val) => match b_peek{
            Some(ref b_val) => {
                if b_val < a_val {
                    res.push(b_peek.take().unwrap());
                    b_peek = b_it.next();
                }else {
                    res.push(a_peek.take().unwrap());
                    a_peek = a_it.next();
                }
            },
            None => {
                res.push(a_peek.take().unwrap());
                res.extend(a_it);
                return res; 
            }
           },
           None =>{
                if let Some(b_val)= b_peek{
                  res.push(b_val);
                }
                res.extend(b_it);
                return res;
           }
        }
    }

} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4,6,1,8,11,13];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,4,6,8,11,13]);
    }
    #[test]
    fn test_marge_sort() {
        let v = vec![4,6,1,8,11,13];
        let v= marge_sort(v);
        assert_eq!(v, vec![1,4,6,8,11,13]);
    }
}
