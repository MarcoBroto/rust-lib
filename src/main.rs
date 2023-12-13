use std::env;
use project::test_heap;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // test_forest();
    test_heap();
}
