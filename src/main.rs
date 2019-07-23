mod hello_world;
use hello_world::hello_world;

mod mutability;
use mutability::mutability;

fn main() {
    println!("<<<<< rust_examples >>>>>\n");
    hello_world();
    mutability();
}
