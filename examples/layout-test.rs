//! Testing out our new layout algorithm
use penrose::builtin::layout::MainAndStack;
use penrose::util::print_layout_result;

fn main() {
    print_layout_result(&mut MainAndStack::default(), 4, 40, 15);
}
