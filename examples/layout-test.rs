//! Testing out our new layout algorithm
use penrose::util::print_layout_result;
use penrose_from_scratch::layouts::Fibonacci;

fn main() {
    print_layout_result(&mut Fibonacci::default(), 6, 40, 15);
}
