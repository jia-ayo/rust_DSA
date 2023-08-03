enum Pet {
    dog,
    cat,
    fish,
}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::dog => "i am a dog",
            Pet::cat => "i am a cat",
            Pet::fish => "i am a fish",
        }
    }
}

enum IpAddrKind {
    // v4,
    v4(String),
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//Assigment
enum Shapes {
    triangle,
    square,
    pentagon,
    octagon,
}

impl Shapes {
    fn corners(self) -> i32 {
        match self {
            Shapes::triangle => 3,
            Shapes::square => 4,
            Shapes::pentagon => 5,
            Shapes::octagon => 8,
        }
    }
}

fn main() {
    let dog1 = Pet::dog;
    println!("{}", dog1.what_am_i());

    // let home = IpAddr{
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.2")
    // };
    let home = IpAddrKind::v4(String::from("127.0.0.1"));

    let loopack = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i8> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    //let sum = x + y; does not work because we cannot add i32 to option<i32> currently

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Fish");
    what_pet("Goath");

    let dog2 = Some(Pet::dog);
    if let Some(Pet::dog) = dog2 {
        println!("thee animal is a dog");
    } else {
        println!("Not a dog");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let x = 5;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("Not one or two"),
    }

    match x {
        1..=5 => println!("matches"),
        _ => println!("does not match"),
    }

    let x = Some(5);
    let y = 5;
    match x {
        Some(10) => println!("10!!"),
        Some(x) if x == y => println!("Matches!"),
        _ => println!("default"),
    }

    //Assigment

    let triangle1 = Shapes::triangle;
    let square1 = Shapes::square;
    println!(
        "{} sides triangle, {} sides square ",
        triangle1.corners(),
        square1.corners()
    )
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog"),
        "Fish" => println!("I have a fish"),
        "Cat" => println!("I have a cat"),
        _ => println!("i have no clue what u are"),
    }
}
