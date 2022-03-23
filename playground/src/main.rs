mod array_test;
mod vector_test;
mod slice_test;
mod string_test;
mod own_test;
mod copy_test;
mod rc_and_arc_test;
mod reference_test;
mod formula_test;
mod panic_test;

fn main() {
    array_test::execute();
    vector_test::execute();
    slice_test::execute();
    string_test::execute();
    own_test::execute();
    copy_test::execute();
    rc_and_arc_test::execute();
    reference_test::execute();
    formula_test::execute();
    panic_test::execute();
}