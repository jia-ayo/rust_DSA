fn main() {
    // let var = 1;
    // let mut s = "hello".to_string();// created on the heap 
    // s.push_str(" world");

    // let x = vec!["tyler".to_string()];
    // let y = x;
    // let z = y;
    // println!("{:?}", z);

    // let x = vec!["tyler".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // let x = 1;
    // let y = x;
    // println!("x = {} , y= {} ", x,y);

    // let s = String::from ("takes"); //create a variable with a string takes
    // take_ownership(s); //gives ownership to function 

    // let val = 1 ;
    // make_copy(val);

    // let str1: String = give_ownership();
    // println!("{}", str1);

    // let str3:String = take_and_given(str1 );

    // if (true){
    //     let str4 = str3;
    // }else {
    //     let str5 = str3;
    // }
    
    // let mut str1 = String::from("Tyler");
    // let mut str2: String;
    // loop {
    //     str2 = &str1;
    // }

    // let mut s = String::from("hello");
    // changge_string(&mut s);
    // println!("{}", s);

    //ASSIGMENT

    // let mut val = vec![1, 3, 5 , 7];
    // if  val[0]==1 {println!("true")} else {println!("false")};
    // val.push(15);
    // println!("{:?}", val);
    
    let  val:i8;
    val = 2;
    add_two(val);
    println!("{}", val);

}

fn add_two(val:  i8){
    println!("{}",val + 2) ;

}

// fn changge_string(some_string: &mut String){
//     some_string.push_str(", world")
// }

// fn take_ownership(s: String){
//     let strin = s;
//     println!("{}", strin);
// }

// fn make_copy(one: i32){
//     let val1 = one;
//     println!("{}", val1);
// }

// fn give_ownership()-> String{
//     "given".to_string()
// }

// fn take_and_given(str2: String) -> String {
//      str2
// }
