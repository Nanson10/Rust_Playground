use rand::{self, Rng};
fn main() {
    for n in 1..11 {
        println!("Roll {} is {}", n, roll_dice());
    }
}

fn roll_dice() -> u8 {
    return rand::rng().random_range(1..=6);
}