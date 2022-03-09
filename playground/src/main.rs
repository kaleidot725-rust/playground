mod array_test;
mod vector_test;
mod slice_test;
mod string_test;
mod own_test;
mod copy_test;
mod rc_and_arc_test;

fn main() {
    array_test::execute();
    vector_test::execute();
    slice_test::execute();
    string_test::execute();
    own_test::execute();
    copy_test::execute();
    rc_and_arc_test::execute();
}