#![deny(clippy::all)]

use int_utils::addition::add;
use int_utils::subtraction::sub;

fn main() {
    let added = add(1, 2);
    let subbed = sub(1, 2);

    println!("added is {}, subtracted is {}", added, subbed);
}
