use std::cell::RefCell;
use std::rc::Rc;

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}
fn main() {
    let t = (12, "eggs"); //created on the stack
    let b = Box::new(t); //created on the heap, but b was stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);

    let s1 = Rc::new(String::from("pointer"));
    let s2 = s1.clone();
    let s3 = s1.clone();

    println!("{}, {}, {}", s1.contains("point"), s2, s3.contains("ter"));

    let flag = Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };
    //borrow retruns Ref<T>
    //borrow_mut returns RefMut<T>

    let reference = Rc::new(flag.is_true.clone());
    println!("{:?}", reference);

    let mut mut_ref = reference.borrow_mut();
    *mut_ref = false;
    println!("{}", mut_ref);


    //Assignment
    let val = 5;
    let val2 = Box::new(2);
    println!("{}", val * *val2);

    let value = String::from("Rc value");

    {
        let rc: Rc<String> = Rc::new(value);
        println!("{}", Rc::strong_count(&rc));
        {
            let rc2: Rc<String> = Rc::clone(&rc);
            println!("{}", Rc::strong_count(&rc));
            println!("{}", Rc::strong_count(&rc2));
        }
        println!("{}", Rc::strong_count(&rc));
    }
}
