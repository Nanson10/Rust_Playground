use std::{thread, time::Duration};
fn main() {
    for countdown in (1..=10).rev() {
        println!("{}...", countdown);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Liftoff!");
}
