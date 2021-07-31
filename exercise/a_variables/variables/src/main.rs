const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (ready, missles) = (READY_AMOUNT, STARTING_MISSLES);
    println!("Hello {} {}", missles, ready);
    println!("New missles is {}", missles - ready);
    println!("{}", what());
}

fn what() -> i32 {
    33
}
