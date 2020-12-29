fn main() {
    let v1: bool = true;
    let v2: bool = true;
    let v3: bool = false;

    let x: bool = v1 || v2 || v3;

    let v = 256;

    println!("{}", v >> 1);
    println!("Hello, {} {}!", "day", x);
    println!("get int: {}", get_int());
}

fn get_int() -> i32 {
    3
}
