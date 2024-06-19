fn fact(num: i32) -> i32 {
    if num > 1 {
        return num * fact(num - 1);
    } else {
        return 1;
    }
}

fn fib(num: i32) -> i32 {
    if num == 1 {
        return 1;
    }
    if num > 1 {
        return fib(num - 1) + fib(num - 2);
    } else {
        0
    }
}

fn palindrom(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }
    if array[start] == array[end] {
        return palindrom(array, start + 1, end - 1);
    } else {
        return false;
    }
}

fn toh(n: i32) -> i32{
    if n == 0{
        return 0
    }
    return toh(n-1) + 1 + toh(n-1);
}

fn main() {
    println!("{}", fact(5));
    println!("{}", fib(6));
    let array = vec![1, 2, 3, 4];
    println!("{:?}", palindrom(&array, 0, array.len() - 1));
    let array = vec![1, 2, 3, 4, 3, 2, 1];
    println!("{:?}", palindrom(&array, 0, array.len() - 1));

    println!("{}", toh(2));
    println!("{}", toh(3 ));
}
