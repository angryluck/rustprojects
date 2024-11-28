fn main() {
    println!("Hello, world!");
    let mut x: Vec<i32> = Vec::new();
    add_two(&mut x);
    println!("{:?}", x);
}



fn add_two (y: &mut Vec<i32>) -> &Vec<i32> {
    y.push(22);
    return y
}
