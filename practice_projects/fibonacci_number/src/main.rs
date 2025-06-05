fn main() {
    let n = 50;
    for i in 1..=n {
        let result = fibonacci(i);
        print!("{result} ")
    }
    print!("\n")
}


fn fibonacci(n: i32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    if n == 1 {
        return a;
    } else if n == 2 {
        return b;
    }
    
    let mut c: u64 = a + b;

    for _i in 3..=n {
        c = a + b;
        a = b;
        b = c;
    }
    return c;
}