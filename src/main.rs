mod operators;
mod types;

use operators::operators_fn;
use types::primitive_types;

fn main() {
    // types examples
    primitive_types();

    // operator examples
    operators_fn();
}
