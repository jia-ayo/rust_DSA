pub struct Option<T>{
    Some(T),
    None
}
fn main() {
    let a = divide(10,5);
    let b = divide(10,0);
    println!("a = {:?}, b ={:?}", a,b);

    // match a {
    //     Res::Thing(v) => println!("val = {}", v),
    //     _ => {}
    // }

    if let Ok(v) = a {
        println!("val = {}", v);
    }
}

fn divide (a:i32, b:i32) -> Result<i32, String>{
    if b==0{
        return Err("cannot divide by 0 ".to_string());
    }
    Ok(a/b)
}
