// struct Point<T, U> {
//     x: T,
//     y: U
// }

// trait Overview {
//     fn overview(&self) -> String {
//         String::from("This is a rust course")
//     }
// }

// struct Course {
//     headline: String,
//     author: String,
// }

// impl Drop for Course {
//     fn drop(&mut self) {
//         println!("Dropping: {}", self.author)
//     }
// }

// struct AnotherCourse {
//     headline: String,
//     author: String,
// }

// impl Overview for Course {
//     fn overview(&self)-> String{
//         format!("{},{}", self.headline, self.author)
//     }
// }

// impl Overview for AnotherCourse {
//     fn overview(&self) -> String {
//         format!("{},{}", self.headline, self.author)
//     }
// }

// use std::ops::Add;
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Add for Point<T>
// where
//     T: Add<Output = T>,
// {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

//Assignment

#[derive(Debug)]
struct Car {
    mpg: i8,
    color: String,
    top_speed: i16
}

#[derive(Debug)]
struct Motorcycle {
    mpg: i8,
    color: String,
    top_speed: i16
}

pub trait Properties {
    fn set_mpg(&mut self, mpg: i8);
    fn set_color(&mut self,  color: String);
    fn set_top_speed(&mut self, top_speed: i16);
}

impl Properties for Car {
    fn set_mpg(&mut self, mpg: i8){
        self.mpg = mpg
    }

    fn set_color(&mut self,  color: String){
        self.color = color
    }
    fn set_top_speed(&mut self, top_speed: i16){
        self.top_speed = top_speed
    }
}

impl Properties for Motorcycle {
    fn set_mpg(&mut self, mpg: i8){
        self.mpg = mpg
    }

    fn set_color(&mut self,  color: String){
        self.color = color
    }
    fn set_top_speed(&mut self, top_speed: i16){
        self.top_speed = top_speed
    }
}

fn print<T: std::fmt::Debug>(value: T){
    println!("{:?}", value)
}
fn main() {
    // let coord = Point { x: 5.0, y: 5.0 };
    // let coord2 = Point { x: 1.0, y: 2.0 };

    // let sum = coord + coord2;
    // println!("{:?}", sum);
    // let coord = Point{x: 5.0, y:5.0};
    // let cord2 = Point {x: 'x', y: 'y'};
    // let cord2 = Point {x: 'x', y: 5.0};

    // let course1 = Course {
    //     headline: String::from("headline!"),
    //     author: String::from("jia"),
    // };
    // let course2 = AnotherCourse {
    //     headline: String::from("another headline!"),
    //     author: String::from("jia-ayo"),
    // };
    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);

    // drop(course1)


    //Assignmate

    let mut car = Car{
        mpg: 0,
        color: String::from("Red"),
        top_speed: 0,
    };

    let mut motocycle = Motorcycle{
        mpg: 0,
        color: String::from("green"),
        top_speed: 0,
    };

    motocycle.set_mpg(10);
    motocycle.set_color(String::from("black"));
    motocycle.set_top_speed(350);

    car.set_mpg(15);
    car.set_color(String::from("blue"));
    car.set_top_speed(250);

    println!("{:?}", car);
    println!("{:?}", motocycle);

    print(vec![1,2,3]);
    print("string");
    print(String::from("string number two"));
    print(10)
}

// fn call_overview(item: &impl Overview) {
//     println!("overview: {}", item.overview())
// }

//fn overview(item1: &impl Overview,  item2: &impl Overview);
//fn overview<T: Overview>(item1: &T, item2: &T)
//fn overview(item1: &impl Overview + AnotherTrait)
//fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)

// trait Clone: Sized {
//     fn clone(&self) -> Self;
//     fn clone_from(&mut self, source: &Self) {
//         *self = source.clone()
//     }
// }
