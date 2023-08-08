// use std::{fmt::Binary, collections::BinaryHeap};

// use rand::seq::SliceRandom;
// use rand::thread_rng;

// use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    // let mut nums: Vec<i32> = vec![];
    // nums.push(1);
    // nums.push(2);
    // nums.push(3);

    // let pop = nums.pop(); //option<T> returns None and Some(T)
    // println!("{:?}", pop);
    // let two = nums[1]; //copy
    //                    //&nums[1], creates a refrence if copy is not available

    // println!("{}", two);
    // let one = nums.first(); //return an Option<T>, so None if vec is empty, or Some(T) is [0]

    // println!("{:?}", one);

    // //.list
    // //.first_mut and .last_mut, so will borrow mutuable references

    // println!("{}", nums.len());// return a value of the login
    // println!("{}", nums.is_empty());//bool

    // nums.insert(0, 10);
    // nums.insert(3, 12);
    // nums.insert(2, 25);

    // nums.remove(3);
    // nums.sort();
    // println!("{:?}", nums);

    // nums.reverse();
    // println!("{:?}", nums);

    // nums.shuffle(&mut thread_rng());

    // println!("{:?}", nums);

    // let mut bheap = BinaryHeap::new();
    // bheap.push(1);
    // bheap.push(18);
    // bheap.push(20);
    // bheap.push(5);

    // bheap.pop();
    // println!("{:?}", bheap);
    // println!("{:?}", bheap.peek());
    // println!("{:?}", bheap);

    // let mut hm = HashMap::new();
    // hm.insert(1, 1);
    // hm.insert(5, 5);
    // hm.insert(3, 7);
    // hm.insert(30, 7);
    // let old = hm.insert(30, 4);//key is going to update the old value of 3 to the new value of 4 also going to return the old value

    // println!("{:?}", hm);
    // println!("{:?}", old);
    // println!("{}", hm.contains_key(&5));
    // println!("{:?}", hm.get(&5));

    // hm.remove(&1);
    // println!("{:?}", hm);

    // let one =  hm.remove(&1);
    // println!("{:?}", one);

    // let remove  = hm.remove_entry(&5);
    // println!("{:?}", remove);

    // hm.clear();

    // println!("{}", hm.is_empty());

    let mut hs = HashSet::new();

    //len()
    //is_empty()

    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);

    for x in hs.iter() {
        println!("iter: {}", x)
    }

    let mut hs2 = HashSet::new();

    hs2.insert(1);
    hs2.insert(3);
    hs2.insert(5);
    hs2.insert(7);

    for x in hs.intersection(&hs2) {
        println!("Intersaction: {}", x)
    }

    let intersection = &hs & &hs2; // shorthand intersectionusing the binary bitwise &operator

    for x in intersection {
        println!("short hand way: {}", x)
    }

    let union = &hs| &hs2;
    for x in union {
        println!("union: {}", x)
    }

    let diff = &hs - &hs2;

    for x in diff {
        println!("Diff: {}", x)
    }
}
