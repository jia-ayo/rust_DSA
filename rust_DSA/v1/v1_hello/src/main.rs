#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fav_color: Color,
}
#[derive(Debug)]
pub enum  Color{
    Red(String),
    Green,
    Blue
}
impl Person{
    pub fn print(self) -> String{
        format!("name = {}, age = {}, has {} childrean", self.name, self.age, self.children)
    }
}
fn main() {
    let p = Person {
        name: "jia-ayo".to_string(),
        age: 19,
        children: 0,
        fav_color: Color::Blue,
    };
    let c = Color::Red("hello".to_string());
    match c {
        Color::Red(s) => println!("the color is red {:?}", s),
        Color::Green => println!("the color is green"),
        Color::Blue => println!("the color is blue")
    }
    println!("Hello, {:?}", p);
}
