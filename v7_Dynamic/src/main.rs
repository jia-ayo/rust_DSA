fn main() {
    for i in 0..10{
        println!("native ={}, iter={}, dynamic={:?}", fibonacci(i), fibonacci_iter(i), fibonacci_dynamic(i))
    }
}
//fib5 = fib(4) + fib(3) = fib3 + fib2 + fib2 + fib1
//= fib2 + fib1 + fib1 + fib0 + fib1 + fib1 + fib0 + fib1  = 0
pub fn fibonacci(n: i32) -> i32{
    if n <= 1{
        return 1;
    }

    //fibonacci(n-1) will call fibonacci(n-2) so we do that function twice
    fibonacci(n-1)+ fibonacci(n-2)
}

pub fn fibonacci_iter(n:i32) -> i32{
    let mut a = 1;
    let mut b= 1;
    let mut res = 1;

    for _ in 1..n{
        res= a+b;
        a=b;
        b=res;
    }
    res

}

//return(res,prev)
//if you are going to use the same function more than once 
//store the result somewhere
pub fn fibonacci_dynamic(n:i32) ->(i32, i32){
    if n == 0 {
        return (1,0);
    }
    let (a,b) = fibonacci_dynamic(n-1);
    (a+b, a)
}

//challenge: fibonacci_dynamic_tail_recursive 