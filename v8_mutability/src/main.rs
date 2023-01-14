#[derive(Debug, Clone)]
pub struct People{
    name: String,
    age: i32,
}
fn main() {
    let mut  x = 34;
    let y = x;
    x += 5;
    println!("x= {} , y ={}", x, y );

    let mut p = People{
        name: "jia".to_string(),
        age: 19,
    };
    let p2 = p.clone();

    p.name.push_str("-ayo");

    println!("p = {:?}\np2={:?}", p, p2)
}
