pub fn learn() {
    let squares_of_evens: Vec<i32> = {
        (1..)
            .map(|x| x * x)
            .filter(|&x| x % 2 == 0)
            .take(10)
            .collect()
    };

    println!("{:?}", squares_of_evens);
    // move_semantics();
}

fn move_semantics() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

fn slices() {
    println!("{}", "wat");
    // let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    // println!("a: {a:?}");

    // let s: &[i32] = &a[2..4];
    // a[3] = 3;
    // println!("s: {s:?}");
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
