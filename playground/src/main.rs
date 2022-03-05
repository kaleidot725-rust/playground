mod array_test;
mod vector_test;
mod slice_test;

fn main() {
    array_test::execute();
    vector_test::execute();
    slice_test::execute();
}