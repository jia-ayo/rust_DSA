// struct User{
//     active: bool,
//     username: String,
//     sign_in_count: u32
// }

// //Turble structs
// struct Coordinates(i32, i32, i32);

//struct UnitStruct;

// struct Square {
//     width: u32,
//     height: u32,
// }

// impl Square {
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
//     fn whats_my_width (&self) -> u32{
//         self.width
//     }
//     fn change_width(&mut self, new_width:u32){
//         self.width = new_width
//     }
// }

// struct MyString<'a>{
//     text: &'a str,
// }

// Assignment
#[derive(Debug)]
struct Car {
    mpg: u32,
    color: String,
    top_speed: u32,
}
impl Car {
    fn set_mpg(&mut self, new_mpg: u32) {
        self.mpg = new_mpg
    }
    fn set_color(&mut self, new_color: String) {
        self.color = new_color
    }
    fn set_top_speed(&mut self, new_top_speed: u32) {
        self.top_speed = new_top_speed
    }
}
fn main() {
    // let str1 = String::from("This is my string");
    // let x = MyString{text: str1.as_str()};
    // let s: &'static str = "I have static lifetime";

    // let user1 = User{active: true, username: String::from("TYLEER"), sign_in_count: 0};
    // println!("{}", user1.username);
    // let user2 = build_user("tyler2".to_string());
    // println!("{}", user2.username);

    // let cords = Coordinates(1,2,3);

    // let mut sq = Square{width: 5, height: 5};
    // println!("{}", sq.area());
    // println!("{}", sq.whats_my_width());
    // sq.change_width(32);
    // println!("{}", sq.whats_my_width());

    // let r;

    // {
    //     let x =5;
    //     r = &x;
    // }//x is dropped

    // println!("{}",r); //'a

    // //&i32
    // //&'a i32
    // //&'a mut 32

    //Assigment
    let mut car1 = Car {
        mpg: 2,
        color: "red".to_string(),
        top_speed: 45,
    };
    println!("{:?}", car1);
    car1.set_color(String::from("blue"));
    car1.set_mpg(6);
    car1.set_top_speed(60);
    println!("{:?}", car1);
}

// fn example<'a>(x: &'a str, y: &'b str)-> &'a str {
//     x
// }//'a for one parameter, 'b for second parameter

// fn build_user(username: String) -> User {
//     User { active: true, username, sign_in_count: 1 }
// }
