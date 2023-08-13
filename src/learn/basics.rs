pub fn run() {
    println!("testing");
}

fn exercise_implicit_conversions() {
    // let x: i8 = 15;
    // let y: i16 = 1000;
    // println!("{x} * {y} = {}", multiply(x, y));
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn learn_function_overloading() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
    // println!("My pid is {}", process::id());
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

fn learn_methods() {
    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };

    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    let mut new_rect = Rectangle::new(20, 30);
    println!("old area: {}", new_rect.area());
    new_rect.inc_width(5);
    println!("new area: {}", new_rect.area());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}

fn strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}

fn test() {
    let squares_of_evens: Vec<i32> = {
        (1..)
            .map(|x| x * x)
            .filter(|&x| x % 2 == 0)
            .take(10)
            .collect()
    };
    println!("{:?}", squares_of_evens);
}

fn move_semantics() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

fn slices() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &mut [i32] = &mut a[..];
    s[0] = 20;

    println!("s: {s:?}");
}

fn memory() {
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("world");

    unsafe {
        let (ptr, capacity, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
    }
}
struct Point(i32, i32);

impl Drop for Point {
    fn drop(&mut self) {
        // Custom cleanup code here
    }
}

fn ownership() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1);
}
