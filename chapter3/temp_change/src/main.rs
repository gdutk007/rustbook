
fn convert_to_celsius(x: i32)-> f64{
    ((x-32)*5/9).into()
}

fn convert_to_fahrenheit(x: i32)-> f64{
    (x*9/5+32).into()
}

fn fib(x:i32)-> i32{
    if x <= 1{
        1
    }else{
        fib(x-2) + fib(x-1)
    }
}

fn main() {
    
    let c = convert_to_celsius(32);
    let f = convert_to_fahrenheit(0);
    println!("32 f in Celsius:{c}");
    println!("0 c in fahrenheit:{f} ");

    let n = fib(15);
    println!("The 5th fib number is: {n}");
}
