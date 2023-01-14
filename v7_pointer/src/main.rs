#[derive(Debug, Clone)]
pub struct Person{
  name: String,
  age: i32
}
impl Person {
    pub fn new( name: String, age: i32 )->Self{
        Person{name , age}
    }
    pub fn greet(&self)-> String {
        format!("my name is {}", self.name)

    }
    pub fn age_up(&mut self, n: i32){
        self.age += n;  
    } 
    pub fn dropme(self){}
}
fn main() {
    let mut p = Person::new("jia".to_string(), 19);
    p.age_up(1);
    let s= p.greet();
    println!("{} i am years {} old", s,p.age);

    let s2 = p.greet();
    println!("really: {}", s2);

    let p = get_age(&p);
    println!("my age is {}", p);
}

pub fn get_age(s: &Person) -> &i32{
    &s.age
}