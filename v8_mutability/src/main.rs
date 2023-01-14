#[derive(Debug, Clone)]
pub struct People{
    name: String,
    age: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct Point{
    x: i32,
    y: i32,
}
impl Point{
    pub fn new(x:i32, y:i32)->Self{
        Point{x,y}
    }
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
    println!("p = {:?}\np2={:?}", p, p2);

    let mut pnt = Point::new(3,4);
    let pnt2 =pnt;
    pnt.x += 3;
    println!("pnt = {:?}\npnt2={:?}", pnt, pnt2);

}
