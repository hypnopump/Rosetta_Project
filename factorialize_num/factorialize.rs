
fn factorialize(num: u32) -> u32 {
    match num {
        0 => 1,
        _ => num * factorialize(num - 1)
    }
}

fn main() {
    let test = 5;
    println!("{}", factorialize(test));
}
