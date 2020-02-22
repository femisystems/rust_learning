mod operators;
mod types;
mod scope_and_shadowing;
mod ds_stack_and_heap;

use operators::operators_fn;
use types::primitive_types;
use scope_and_shadowing::{scope_test, shadow_test};
use ds_stack_and_heap::stack_and_heap;



fn main() {
    // types examples
    primitive_types();

    // operator examples
    operators_fn();

    // scope and shadowing
    scope_test();
    shadow_test();

    // stack and heaps
    stack_and_heap();
}
