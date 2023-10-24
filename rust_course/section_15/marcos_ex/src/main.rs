use procedural_macro::debug_print;

macro_rules! avarage{
    ( $(,)* ) =>{{
        0.0
    }};

    ( $($val: expr), + $(,)*) => {{
        let count = 0usize $(+ {let _ = stringify!($val); 1})*;
        let sum = 0.0 $(+ $val as f64)*;
        sum / count as f64
    }}
}

macro_rules! op {
    ($a: expr, $b: expr, $c: expr) => {{
        let t: u32 = match $c {
            1 => $a + $b,
            2 => $a - $b,
            3 => $a * $b,
            4 => $a / $b,
            _ => $a % $b,
        };
        t as u32
    }};
}

#[debug_print]
fn main() {
    println!("Hello, world!");
    println!("{}", avarage!());
    println!("{}", avarage!(1.0, 2.0, 3.0));
    println!("{}", avarage!(1, 2, 3, 4, 5));
    println!("{}", op!(5, 2, 1)); //should print 7

    println!("{}", op!(5, 2, 2)); //should print 3

    println!("{}", op!(5, 2, 3)); //should print 10

    println!("{}", op!(5, 2, 4)); //should print 2

    println!("{}", op!(5, 2, 5)); //should print 1

    println!("{}", op!(5, 2, 6)); //should print -1
}
